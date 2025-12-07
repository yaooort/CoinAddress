use iced::{
    executor, theme,
    widget::{button, checkbox, column, container, progress_bar, row, text, text_input, Container},
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

pub struct VanityApp {
    // 配置
    patterns_input: String,
    batch_size: String,
    thread_count: String,
    save_all: bool,

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
    vanity_cache: Arc<Mutex<Option<String>>>,
}

impl Default for VanityApp {
    fn default() -> Self {
        Self {
            batch_size: "1000".to_string(),
            thread_count: num_cpus::get().to_string(),
            patterns_input: "1111,2222,3333,4444,5555,6666,7777,8888,9999,0000".to_string(),
            save_all: false,
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
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    PatternsChanged(String),
    BatchSizeChanged(String),
    ThreadCountChanged(String),
    SaveAllToggled(bool),
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
            Message::PatternsChanged(input) => self.patterns_input = input,
            Message::BatchSizeChanged(input) => self.batch_size = input,
            Message::ThreadCountChanged(input) => self.thread_count = input,
            Message::SaveAllToggled(checked) => self.save_all = checked,
            Message::StartPressed => {
                if !self.is_running {
                    self.is_running = true;
                    self.is_paused = false;
                    self.total_generated = 0;
                    self.total_found = 0;
                    self.last_count = 0;
                    self.generation_rate = 0.0;
                    self.start_time = Some(Instant::now());
                    self.gen_start_time = Some(Instant::now());
                    self.log_messages.clear();
                    self.log_messages.push("▶ 生成启动... 开始搜索靓号...".to_string());

                    let stop_signal = Arc::clone(&self.stop_signal);
                    let pause_signal = Arc::clone(&self.pause_signal);
                    let gen_count = Arc::clone(&self.gen_count);
                    let found_count = Arc::clone(&self.found_count);

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

                        thread::spawn(move || loop {
                            if stop.load(Ordering::Relaxed) {
                                break;
                            }

                            if pause.load(Ordering::Relaxed) {
                                thread::sleep(Duration::from_millis(50));
                                continue;
                            }

                            for _ in 0..batch {
                                let addr = generate_tron_address();
                                let patterns_refs: Vec<&str> =
                                    patterns_clone.iter().map(|s| s.as_str()).collect();
                                if is_vanity_address(&addr.address, &patterns_refs) {
                                    found.fetch_add(1, Ordering::Relaxed);
                                    // 保存到文件
                                    let _ = save_address_to_file("tron_vanity.txt", &addr, true);
                                    // 缓存靓号信息用于 UI 显示
                                    if let Ok(mut cache) = vanity_cache.lock() {
                                        *cache = Some(addr.address.clone());
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
                    self.pause_signal
                        .store(new_state, Ordering::Relaxed);
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
                        let rate = if elapsed > 0.0 { total as f64 / elapsed } else { 0.0 };

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
                    if let Some(vanity_info) = cache.take() {
                        self.log_messages.insert(0, format!("✨ 发现靓号: {}", vanity_info));
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
        let header = column![
            text("TRON 波场靓号生成器")
                .size(32)
                .style(iced::theme::Text::Color(Color::from_rgb8(231, 242, 255))),
            text("并行加速 · 实时监控 · 丝滑体验")
                .size(16)
                .style(iced::theme::Text::Color(Color::from_rgb8(142, 162, 185))),
        ]
        .spacing(4)
        .width(Length::Fill)
        .align_items(Alignment::Start);

        let patterns_row = row![
                text("靓号模式 (逗号分隔)").size(14).style(iced::theme::Text::Color(accent())),
            text_input("1111,2222,....", &self.patterns_input)
                .on_input(Message::PatternsChanged)
                .padding(10)
                .size(14)
                .width(Length::Fill),
        ]
        .spacing(12)
        .align_items(Alignment::Center);

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
            checkbox("保存所有地址", self.save_all).on_toggle(Message::SaveAllToggled),
        ]
        .spacing(12)
        .align_items(Alignment::Center);

        let controls = row![
            primary_button("启动", Message::StartPressed),
            ghost_button(if self.is_paused { "继续" } else { "暂停" }, Message::PausePressed),
            danger_button("停止", Message::StopPressed),
        ]
        .spacing(12);

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
                text("系统监控").size(18).style(iced::theme::Text::Color(accent())),
                row![
                    text(format!("CPU {:.1}%", self.cpu_percent)).size(14),
                    progress_bar(0.0..=100.0, self.cpu_percent).height(16.0).width(Length::Fill),
                ]
                .spacing(10)
                .align_items(Alignment::Center),
                row![
                    text(format!(
                        "内存 {:.1}% ({} MB / {} MB)",
                        self.memory_percent, self.memory_used_mb, self.memory_total_mb
                    ))
                    .size(14),
                    progress_bar(0.0..=100.0, self.memory_percent).height(16.0).width(Length::Fill),
                ]
                .spacing(10)
                .align_items(Alignment::Center),
            ]
            .spacing(12),
        );

        let logs = {
            let mut col = column![text("日志").size(18).style(iced::theme::Text::Color(accent()))];
            for msg in self.log_messages.iter().take(8) {
                col = col.push(text(msg).size(13));
            }
            card(col.spacing(6))
                .width(Length::Fill)
                .height(Length::Fixed(240.0))
        };

        let layout = column![
            header,
            card(column![patterns_row, batch_threads_row].spacing(12)),
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
    button(text(label).style(iced::theme::Text::Color(Color::WHITE)).size(14))
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


