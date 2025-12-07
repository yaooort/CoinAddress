#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use iced::mouse;
use iced::widget::canvas::{self, path, Canvas};
use iced::{
    executor, theme,
    widget::{button, column, container, progress_bar, row, svg, text, text_input, Container},
    Alignment, Application, Border, Color, Command, Element, Font, Length, Settings, Size, Theme,
};
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::{Duration, Instant};

use tron_vanity::monitor::SystemMonitor;
use tron_vanity::*;
use tron_vanity::Assets;

// 中文字体（根据平台选择系统字体）
#[cfg(target_os = "macos")]
const CHINESE_FONT: Font = Font::with_name("PingFang SC");

#[cfg(target_os = "windows")]
const CHINESE_FONT: Font = Font::with_name("Microsoft YaHei");

#[cfg(target_os = "linux")]
const CHINESE_FONT: Font = Font::with_name("WenQuanYi Micro Hei");

fn accent() -> Color {
    Color::from_rgb8(64, 211, 255)
}

fn card_bg() -> Color {
    Color::from_rgb8(24, 28, 40)
}

fn surface_bg() -> Color {
    Color::from_rgb8(16, 18, 28)
}

fn load_logo() -> svg::Handle {
    if let Some(file) = Assets::get("logos/logo.svg") {
        return svg::Handle::from_memory(file.data);
    }
    // 回退为空 handle，避免 panic
    svg::Handle::from_memory(Vec::<u8>::new())
}

pub struct VanityApp {
    // 链选择（多选）
    selected_chains: Vec<ChainType>,

    // 配置
    patterns_input: String,
    batch_size: String,
    thread_count: String,

    // 状态
    is_running: bool,
    is_paused: bool,
    start_time: Option<Instant>,
    gen_start_time: Option<Instant>,

    // 统计
    total_generated: u64,
    total_found: u64,
    last_count: u64,
    generation_rate: f64,

    // 系统监控
    cpu_percent: f32,
    memory_percent: f32,
    memory_used_mb: u64,
    memory_total_mb: u64,
    monitor: SystemMonitor,

    // 日志
    log_messages: Vec<String>,

    // 后台状态
    stop_signal: Arc<AtomicBool>,
    pause_signal: Arc<AtomicBool>,
    gen_count: Arc<AtomicU64>,
    found_count: Arc<AtomicU64>,
    // 缓存匹配结果，附带完整三链展示文本
    vanity_cache: Arc<Mutex<Option<(VanityAddress, String)>>>,

    // 最近发现的靓号（用于手动保存）
    last_found: Option<VanityAddress>,

    // 内嵌 logo 资源
    logo_handle: svg::Handle,

    // 保存文件路径
    save_file_path: String,
}

impl Default for VanityApp {
    fn default() -> Self {
        Self {
            selected_chains: vec![ChainType::Tron],
            batch_size: "1000".to_string(),
            thread_count: num_cpus::get().to_string(),
            patterns_input: "1111,2222,3333,4444,5555,6666,7777,8888,9999,0000".to_string(),
            is_running: false,
            is_paused: false,
            start_time: None,
            gen_start_time: None,
            total_generated: 0,
            total_found: 0,
            last_count: 0,
            generation_rate: 0.0,
            cpu_percent: 0.0,
            memory_percent: 0.0,
            memory_used_mb: 0,
            memory_total_mb: 0,
            monitor: SystemMonitor::new(),
            log_messages: vec!["启动就绪".to_string()],
            stop_signal: Arc::new(AtomicBool::new(false)),
            pause_signal: Arc::new(AtomicBool::new(false)),
            gen_count: Arc::new(AtomicU64::new(0)),
            found_count: Arc::new(AtomicU64::new(0)),
            vanity_cache: Arc::new(Mutex::new(None)),
            last_found: None,
            logo_handle: load_logo(),
            save_file_path: Self::default_save_path(),
        }
    }
}

impl VanityApp {
    fn default_save_path() -> String {
        if let Some(home) = dirs::home_dir() {
            home.join("Desktop").join("vanity_addresses.txt").to_string_lossy().to_string()
        } else {
            "vanity_addresses.txt".to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ChainToggled(ChainType),
    PatternsChanged(String),
    BatchSizeChanged(String),
    ThreadCountChanged(String),
    ChooseSaveFile,
    SaveFileSelected(Option<std::path::PathBuf>),
    StartPressed,
    PausePressed,
    StopPressed,
    Tick,
    VanityFound(String),
}

impl Application for VanityApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        "TRON 波场靓号生成器".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ChainToggled(chain) => {
                if let Some(pos) = self.selected_chains.iter().position(|c| c == &chain) {
                    self.selected_chains.remove(pos);
                } else {
                    self.selected_chains.push(chain);
                }
            }
            Message::PatternsChanged(input) => self.patterns_input = input,
            Message::BatchSizeChanged(input) => self.batch_size = input,
            Message::ThreadCountChanged(input) => self.thread_count = input,
            Message::ChooseSaveFile => {
                return Command::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .set_file_name("vanity_addresses.txt")
                            .set_title("选择保存文件")
                            .save_file()
                            .await
                            .map(|handle| handle.path().to_path_buf())
                    },
                    Message::SaveFileSelected,
                );
            }
            Message::SaveFileSelected(path) => {
                if let Some(path) = path {
                    self.save_file_path = path.to_string_lossy().to_string();
                    self.log_messages.push(format!("✓ 保存路径: {}", self.save_file_path));
                }
            }
            Message::StartPressed => {
                if !self.is_running && !self.selected_chains.is_empty() {
                    self.is_running = true;
                    self.is_paused = false;
                    self.total_generated = 0;
                    self.total_found = 0;
                    self.last_count = 0;
                    self.generation_rate = 0.0;
                    self.start_time = Some(Instant::now());
                    self.gen_start_time = Some(Instant::now());
                    self.log_messages.clear();
                    
                    let chains_str = self.selected_chains
                        .iter()
                        .map(|c| c.label())
                        .collect::<Vec<_>>()
                        .join(" | ");
                    self.log_messages.push(format!(
                        "▶ [{}] 启动，开始搜索靓号...",
                        chains_str
                    ));

                    let stop_signal = Arc::clone(&self.stop_signal);
                    let pause_signal = Arc::clone(&self.pause_signal);
                    let gen_count = Arc::clone(&self.gen_count);
                    let found_count = Arc::clone(&self.found_count);
                    let selected_chains = self.selected_chains.clone();

                    stop_signal.store(false, Ordering::Relaxed);
                    pause_signal.store(false, Ordering::Relaxed);
                    gen_count.store(0, Ordering::Relaxed);
                    found_count.store(0, Ordering::Relaxed);

                    let patterns: Vec<String> = self
                        .patterns_input
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();

                    let batch: u64 = self.batch_size.parse().unwrap_or(1000).max(1);
                    let threads: usize = self
                        .thread_count
                        .parse()
                        .unwrap_or_else(|_| num_cpus::get())
                        .clamp(1, num_cpus::get());

                    for _ in 0..threads {
                        let stop = Arc::clone(&stop_signal);
                        let pause = Arc::clone(&pause_signal);
                        let gen = Arc::clone(&gen_count);
                        let found = Arc::clone(&found_count);
                        let vanity_cache = Arc::clone(&self.vanity_cache);
                        let patterns_clone = patterns.clone();
                        let chains_copy = selected_chains.clone();
                        let save_path = self.save_file_path.clone();

                        thread::spawn(move || loop {
                            if stop.load(Ordering::Relaxed) {
                                break;
                            }

                            if pause.load(Ordering::Relaxed) {
                                thread::sleep(Duration::from_millis(50));
                                continue;
                            }

                            for _ in 0..batch {
                                // 生成单个助记词对应的三种链地址
                                let mnemonic = generate_mnemonic();
                                let multi_addr = generate_from_mnemonic_all(&mnemonic);
                                
                                let patterns_refs: Vec<&str> =
                                    patterns_clone.iter().map(|s| s.as_str()).collect();
                                
                                // 对每个选中的链进行匹配
                                let addresses = vec![
                                    (&multi_addr.tron, ChainType::Tron),
                                    (&multi_addr.evm, ChainType::Evm),
                                    (&multi_addr.sol, ChainType::Sol),
                                ];
                                
                                for (addr, chain_type) in addresses {
                                    if chains_copy.contains(&chain_type)
                                        && is_vanity_address(&addr.address, &patterns_refs)
                                    {
                                        found.fetch_add(1, Ordering::Relaxed);
                                        let _ =
                                            save_multi_address_to_file(&save_path, &multi_addr, chain_type);

                                        let display = format!(
                                            "✨ 发现靓号: [{}] {} | TRON: {} | EVM: {} | SOL: {}",
                                            chain_type.label(),
                                            addr.address,
                                            multi_addr.tron.address,
                                            multi_addr.evm.address,
                                            multi_addr.sol.address,
                                        );

                                        if let Ok(mut cache) = vanity_cache.lock() {
                                            *cache = Some((addr.clone(), display));
                                        }
                                    }
                                }
                                gen.fetch_add(1, Ordering::Relaxed);
                            }
                        });
                    }
                }
            }
            Message::PausePressed => {
                if self.is_running {
                    let new_state = !self.is_paused;
                    self.is_paused = new_state;
                    self.pause_signal.store(new_state, Ordering::Relaxed);
                    if new_state {
                        self.log_messages.insert(0, "⏸ 已暂停".to_string());
                    } else {
                        self.log_messages.insert(0, "▶ 已继续".to_string());
                    }
                }
            }
            Message::StopPressed => {
                if self.is_running {
                    self.is_running = false;
                    self.is_paused = false;
                    self.stop_signal.store(true, Ordering::Relaxed);
                    self.pause_signal.store(false, Ordering::Relaxed);
                    self.log_messages.insert(0, "⏹ 已停止".to_string());

                    if let Some(start) = self.start_time {
                        let total = self.total_generated;
                        let found = self.total_found;
                        let elapsed = start.elapsed().as_secs_f64();
                        let rate = if elapsed > 0.0 {
                            total as f64 / elapsed
                        } else {
                            0.0
                        };

                        self.log_messages.insert(
                            0,
                            format!(
                                "📊 统计: 总数={}, 靓号={}, 速率={:.0} addr/s, 耗时={:.1}s",
                                total, found, rate, elapsed
                            ),
                        );
                    }
                }
            }
            Message::Tick => {
                let stats = self.monitor.get_stats();
                self.cpu_percent = stats.cpu_percent;
                self.memory_percent = stats.memory_percent;
                self.memory_used_mb = stats.memory_used_mb;
                self.memory_total_mb = stats.memory_total_mb;

                // 检查是否有新的靓号
                if let Ok(mut cache) = self.vanity_cache.lock() {
                    if let Some((vanity_info, display)) = cache.take() {
                        self.log_messages.insert(0, display);
                        self.last_found = Some(vanity_info);
                    }
                }

                if self.is_running && !self.is_paused {
                    let current_gen = self.gen_count.load(Ordering::Relaxed);
                    let current_found = self.found_count.load(Ordering::Relaxed);

                    if let Some(gen_start) = self.gen_start_time {
                        let elapsed = gen_start.elapsed().as_secs_f64();
                        if elapsed > 0.1 {
                            self.generation_rate = (current_gen - self.last_count) as f64 / elapsed;
                            self.last_count = current_gen;
                            self.gen_start_time = Some(Instant::now());
                        }
                    }

                    self.total_generated = current_gen;
                    self.total_found = current_found;
                }
            }
            Message::VanityFound(_) => {
                // 此消息由 Tick 内部处理
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        const CHAINS: [ChainType; 3] = [ChainType::Tron, ChainType::Evm, ChainType::Sol];

        // 构建链选择按钮组（多选）
        let chain_buttons = CHAINS
            .iter()
            .fold(row![].spacing(8), |row, &chain| {
                let is_selected = self.selected_chains.contains(&chain);
                let btn = if is_selected {
                    button(
                        text(chain.label())
                            .size(14)
                            .style(iced::theme::Text::Color(Color::from_rgb8(255, 255, 255))),
                    )
                    .padding(10)
                    .style(iced::theme::Button::Positive)
                    .on_press(Message::ChainToggled(chain))
                } else {
                    button(
                        text(chain.label())
                            .size(14)
                            .style(iced::theme::Text::Color(Color::from_rgb8(142, 162, 185))),
                    )
                    .padding(10)
                    .style(iced::theme::Button::Secondary)
                    .on_press(Message::ChainToggled(chain))
                };
                row.push(btn)
            });

        let header = row![
            svg(self.logo_handle.clone())
                .width(Length::Fixed(40.0))
                .height(Length::Fixed(40.0)),
            column![
                text("多链靓号工坊")
                    .size(30)
                    .style(iced::theme::Text::Color(Color::from_rgb8(231, 242, 255))),
                text("TRON · EVM · SOL | 并行加速 · 实时监控")
                    .size(16)
                    .style(iced::theme::Text::Color(Color::from_rgb8(142, 162, 185))),
            ]
            .spacing(4)
            .width(Length::Fill)
            .align_items(Alignment::Start),
            column![
                text("选择链").size(12).style(iced::theme::Text::Color(accent())),
                chain_buttons,
            ]
            .spacing(6),
        ]
        .spacing(12)
        .align_items(Alignment::Center);

        let patterns_row = column![
            text("靓号模式 (末尾匹配, 逗号分隔) ")
                .size(14)
                .style(iced::theme::Text::Color(accent())),
            text_input("1111,2222,...", &self.patterns_input)
                .on_input(Message::PatternsChanged)
                .padding(12)
                .size(14)
                .width(Length::Fill),
        ]
        .spacing(8);

        let file_path_row = column![
            row![
                text("保存路径:").size(14).style(iced::theme::Text::Color(accent())),
                ghost_button("选择文件", Message::ChooseSaveFile),
            ]
            .spacing(12)
            .align_items(Alignment::Center),
            text(&self.save_file_path)
                .size(12)
                .style(iced::theme::Text::Color(Color::from_rgb8(160, 180, 200))),
        ]
        .spacing(6);

        let batch_threads_row = row![
            text("批处理大小").size(14).width(Length::Shrink),
            text_input("1000", &self.batch_size)
                .on_input(Message::BatchSizeChanged)
                .padding(10)
                .width(Length::Fixed(120.0)),
            text("线程数").size(14).width(Length::Shrink),
            text_input("8", &self.thread_count)
                .on_input(Message::ThreadCountChanged)
                .padding(10)
                .width(Length::Fixed(120.0)),
        ]
        .spacing(12)
        .align_items(Alignment::Center);

        let controls = row![
            primary_button("启动", Message::StartPressed),
            danger_button("停止", Message::StopPressed),
            ghost_button(
                if self.is_paused { "继续" } else { "暂停" },
                Message::PausePressed
            ),
        ]
        .spacing(14)
        .align_items(Alignment::Center);

        let uptime = self.start_time.map(|t| t.elapsed().as_secs()).unwrap_or(0);

        let stat_cards = row![
            metric_card("总生成", &format!("{}", self.total_generated)),
            metric_card("靓号数", &format!("{}", self.total_found)),
            metric_card("速率", &format!("{:.0} addr/s", self.generation_rate)),
            metric_card("运行时长", &format!("{} s", uptime)),
        ]
        .spacing(12)
        .width(Length::Fill)
        .align_items(Alignment::Start);

        let system_card = card(
            column![
                text("系统监控·仪表模式")
                    .size(18)
                    .style(iced::theme::Text::Color(accent())),
                row![
                    gauge("CPU", self.cpu_percent, Color::from_rgb8(255, 99, 146)),
                    gauge("内存", self.memory_percent, Color::from_rgb8(92, 225, 230),),
                ]
                .spacing(16)
                .align_items(Alignment::Center),
                row![
                    text(format!("CPU {:.1}%", self.cpu_percent)).size(14),
                    progress_bar(0.0..=100.0, self.cpu_percent)
                        .height(10.0)
                        .width(Length::Fill),
                ]
                .spacing(10)
                .align_items(Alignment::Center),
                row![
                    text(format!(
                        "内存 {:.1}% ({} MB / {} MB)",
                        self.memory_percent, self.memory_used_mb, self.memory_total_mb
                    ))
                    .size(14),
                    progress_bar(0.0..=100.0, self.memory_percent)
                        .height(10.0)
                        .width(Length::Fill),
                ]
                .spacing(10)
                .align_items(Alignment::Center),
            ]
            .spacing(12),
        );

        let logs = {
            let mut col = column![text("日志")
                .size(18)
                .style(iced::theme::Text::Color(accent()))];
            for msg in self.log_messages.iter().take(10) {
                col = col.push(text(msg).size(13));
            }
            card(col.spacing(6))
                .width(Length::Fill)
                .height(Length::Fixed(240.0))
        };

        let layout = column![
            header,
            card(column![file_path_row, patterns_row, batch_threads_row].spacing(12)),
            card(column![controls].spacing(8)),
            card(stat_cards),
            system_card,
            logs,
        ]
        .spacing(14)
        .padding(20)
        .width(Length::Fill);

        container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .style(theme::Container::Custom(Box::new(SurfaceStyle)))
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::time::every(Duration::from_millis(400)).map(|_| Message::Tick)
    }
}

fn primary_button<'a>(label: &str, on_press: Message) -> iced::widget::Button<'a, Message> {
    button(
        text(label)
            .style(iced::theme::Text::Color(Color::WHITE))
            .size(14),
    )
    .padding([10, 16])
    .style(theme::Button::Positive)
    .on_press(on_press)
}

fn ghost_button<'a>(label: &str, on_press: Message) -> iced::widget::Button<'a, Message> {
    button(text(label).size(14))
        .padding([10, 16])
        .style(theme::Button::Secondary)
        .on_press(on_press)
}

fn danger_button<'a>(label: &str, on_press: Message) -> iced::widget::Button<'a, Message> {
    button(text(label).size(14))
        .padding([10, 16])
        .style(theme::Button::Destructive)
        .on_press(on_press)
}

fn metric_card<'a>(title: &str, value: &str) -> Container<'a, Message> {
    card(
        column![
            text(title)
                .size(14)
                .style(iced::theme::Text::Color(Color::from_rgb8(160, 180, 200))),
            text(value)
                .size(22)
                .style(iced::theme::Text::Color(accent())),
        ]
        .spacing(6),
    )
    .width(Length::Fill)
}

fn card<'a>(content: impl Into<Element<'a, Message>>) -> Container<'a, Message> {
    container(content)
        .padding(16)
        .style(theme::Container::Custom(Box::new(CardStyle)))
}

struct CardStyle;

impl iced::widget::container::StyleSheet for CardStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(card_bg().into()),
            border: Border {
                radius: 12.0.into(),
                ..Default::default()
            },
            text_color: Some(Color::WHITE),
            ..Default::default()
        }
    }
}

struct SurfaceStyle;

impl iced::widget::container::StyleSheet for SurfaceStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(surface_bg().into()),
            text_color: Some(Color::WHITE),
            ..Default::default()
        }
    }
}

fn gauge<'a>(label: &str, value: f32, color: Color) -> Element<'a, Message> {
    let clamped = value.clamp(0.0, 100.0);
    let gauge = Gauge {
        label: label.to_string(),
        value: clamped,
        color,
    };

    Canvas::new(gauge)
        .width(Length::Fixed(180.0))
        .height(Length::Fixed(140.0))
        .into()
}

struct Gauge {
    label: String,
    value: f32,
    color: Color,
}

impl canvas::Program<Message> for Gauge {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let center = frame.center();
        let radius = (bounds.width.min(bounds.height) / 2.0) - 10.0;

        // 背景环
        frame.stroke(
            &canvas::Path::circle(center, radius),
            canvas::Stroke::default()
                .with_width(10.0)
                .with_color(Color::from_rgba(1.0, 1.0, 1.0, 0.08)),
        );

        // 值环（180 度半圆）
        let sweep = std::f32::consts::PI * (self.value / 100.0);
        let start = -std::f32::consts::PI;
        let end = start + sweep;
        let arc = canvas::Path::new(|p| {
            p.arc(path::Arc {
                center,
                radius,
                start_angle: iced::Radians(start),
                end_angle: iced::Radians(end),
            });
        });
        frame.stroke(
            &arc,
            canvas::Stroke::default()
                .with_width(10.0)
                .with_color(self.color)
                .with_line_cap(canvas::LineCap::Round),
        );

        // 文本
        frame.fill_text(canvas::Text {
            content: format!("{}", self.label),
            position: center + iced::Vector::new(0.0, -12.0),
            color: Color::from_rgb8(210, 220, 235),
            size: iced::Pixels(16.0),
            horizontal_alignment: iced::alignment::Horizontal::Center,
            ..Default::default()
        });

        frame.fill_text(canvas::Text {
            content: format!("{:.1}%", self.value),
            position: center + iced::Vector::new(0.0, 16.0),
            color: self.color,
            size: iced::Pixels(22.0),
            horizontal_alignment: iced::alignment::Horizontal::Center,
            ..Default::default()
        });

        vec![frame.into_geometry()]
    }
}

pub fn main() -> iced::Result {
    VanityApp::run(Settings {
        window: iced::window::Settings {
            size: Size::new(980.0, 900.0),
            resizable: true,
            ..Default::default()
        },
        default_font: CHINESE_FONT,
        ..Default::default()
    })
}
