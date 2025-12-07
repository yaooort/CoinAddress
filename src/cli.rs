use colored::*;
use rayon::prelude::*;
use std::io::{self, Write};
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc,
};
use std::thread;
use std::time::Instant;

use tron_vanity::*;

struct Config {
    patterns: Vec<String>,
    output_file: String,
    save_all: bool,
    batch_size: usize,
    num_threads: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            patterns: vec![
                "1111".to_string(),
                "2222".to_string(),
                "3333".to_string(),
                "4444".to_string(),
                "5555".to_string(),
                "6666".to_string(),
                "7777".to_string(),
                "8888".to_string(),
                "9999".to_string(),
                "0000".to_string(),
                "AAAA".to_string(),
                "BBBB".to_string(),
                "CCCC".to_string(),
                "DDDD".to_string(),
            ],
            output_file: "tron_vanity.txt".to_string(),
            save_all: false,
            batch_size: 1000,
            num_threads: num_cpus::get(),
        }
    }
}

fn main() {
    println!(
        "{}",
        "╔════════════════════════════════════════════════════════════╗".bright_cyan()
    );
    println!(
        "{}",
        "║  TRON 波场靓号生成器 | TRON Vanity Address Generator  ║".bright_cyan()
    );
    println!(
        "{}",
        "║       快速高效 • 充分利用 CPU 和 GPU 性能              ║".bright_cyan()
    );
    println!(
        "{}",
        "╚════════════════════════════════════════════════════════════╝".bright_cyan()
    );
    println!();

    let mut config = Config::default();

    // 交互式配置
    print_menu();
    let choice = get_user_input("请选择功能 (Select option): ");

    match choice.trim() {
        "1" => {
            // 使用默认靓号
            println!(
                "{}",
                "使用默认靓号模式... Using default vanity patterns...".bright_yellow()
            );
            run_vanity_generator(&config);
        }
        "2" => {
            // 自定义靓号
            config.patterns.clear();
            print!("输入想要的靓号模式 (逗号分隔，如: 1111,AAAA,好运): ");
            io::stdout().flush().unwrap();
            let input = get_user_input("");
            config.patterns = input
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            if config.patterns.is_empty() {
                println!("{}", "未输入模式，使用默认".red());
                config = Config::default();
            }

            run_vanity_generator(&config);
        }
        "3" => {
            // 快速模式 - 测试生成效率
            println!("{}", "快速模式 - 生成100个地址并计时".bright_green());
            benchmark_generation();
        }
        "4" => {
            // 高级设置
            configure_advanced(&mut config);
            run_vanity_generator(&config);
        }
        _ => {
            println!("{}", "无效选择，运行默认模式".red());
            run_vanity_generator(&config);
        }
    }
}

fn print_menu() {
    println!("{}", "┌────────────────────────────────────┐".bright_blue());
    println!("{}", "│  1. 使用默认靓号 (Default)         │".bright_blue());
    println!("{}", "│  2. 自定义靓号模式 (Custom)        │".bright_blue());
    println!("{}", "│  3. 性能测试 (Benchmark)          │".bright_blue());
    println!("{}", "│  4. 高级设置 (Advanced)            │".bright_blue());
    println!("{}", "└────────────────────────────────────┘".bright_blue());
    println!();
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn configure_advanced(config: &mut Config) {
    println!(
        "{}",
        "╔════════════════════════════════════════════════════════════╗".bright_magenta()
    );
    println!(
        "{}",
        "║                   高级设置 (Advanced Settings)             ║".bright_magenta()
    );
    println!(
        "{}",
        "╚════════════════════════════════════════════════════════════╝".bright_magenta()
    );

    let save_all_str = get_user_input("保存所有生成的地址? (Save all addresses? y/n): ");
    config.save_all = save_all_str.trim().to_lowercase().starts_with('y');

    let threads_str = get_user_input(&format!(
        "线程数 (Number of threads, default {}): ",
        config.num_threads
    ));
    if let Ok(n) = threads_str.trim().parse::<usize>() {
        if n > 0 && n <= 256 {
            config.num_threads = n;
        }
    }

    let batch_str = get_user_input(&format!(
        "批处理大小 (Batch size, default {}): ",
        config.batch_size
    ));
    if let Ok(n) = batch_str.trim().parse::<usize>() {
        if n > 0 && n <= 100000 {
            config.batch_size = n;
        }
    }

    println!(
        "{}",
        format!(
            "配置完成: {} 线程, 批大小 {}",
            config.num_threads, config.batch_size
        )
        .bright_green()
    );
}

fn run_vanity_generator(config: &Config) {
    println!();
    println!(
        "{}",
        "╔════════════════════════════════════════════════════════════╗".bright_green()
    );
    println!(
        "{}",
        "║                    开始生成靓号 (Starting)                 ║".bright_green()
    );
    println!(
        "{}",
        "╚════════════════════════════════════════════════════════════╝".bright_green()
    );

    println!(
        "{}",
        format!("靓号模式 | Patterns: {}", config.patterns.join(", ")).bright_yellow()
    );
    println!(
        "{}",
        format!("线程数 | Threads: {}", config.num_threads).bright_yellow()
    );
    println!(
        "{}",
        format!("输出文件 | Output File: {}", config.output_file).bright_yellow()
    );
    println!();

    let start = Instant::now();
    let counter = Arc::new(AtomicU64::new(0));
    let found = Arc::new(AtomicU64::new(0));
    let should_stop = Arc::new(AtomicBool::new(false));

    // 显示统计信息的线程
    let counter_clone = Arc::clone(&counter);
    let found_clone = Arc::clone(&found);
    let should_stop_clone = Arc::clone(&should_stop);

    let stats_thread = thread::spawn(move || {
        let mut last_count = 0u64;
        loop {
            thread::sleep(std::time::Duration::from_secs(1));

            if should_stop_clone.load(Ordering::Relaxed) {
                break;
            }

            let total = counter_clone.load(Ordering::Relaxed);
            let total_found = found_clone.load(Ordering::Relaxed);
            let rate = total - last_count;

            print!(
                "\r{} {} | {} 个靓号已找到 | 速率: {:.0} addr/s    ",
                "▶".bright_cyan(),
                format!("已生成 {} 个地址", total).bright_white(),
                format!("{}", total_found).bright_yellow(),
                rate as f64
            );
            io::stdout().flush().unwrap();

            last_count = total;
        }
    });

    // 停止信号线程
    let should_stop_clone = Arc::clone(&should_stop);
    let stop_thread = thread::spawn(move || {
        let _ = get_user_input("");
        should_stop_clone.store(true, Ordering::Relaxed);
    });

    // 主生成线程
    let patterns: Vec<&str> = config.patterns.iter().map(|s| s.as_str()).collect();

    loop {
        if should_stop.load(Ordering::Relaxed) {
            break;
        }

        let batch: Vec<_> = (0..config.batch_size)
            .map(|_| {
                let mnemonic = generate_mnemonic();
                generate_from_mnemonic_all(&mnemonic)
            })
            .collect();

        for multi in batch {
            if should_stop.load(Ordering::Relaxed) {
                break;
            }

            counter.fetch_add(1, Ordering::Relaxed);

            // 检查三条链是否匹配
            let hits = [
                (&multi.tron, ChainType::Tron),
                (&multi.evm, ChainType::Evm),
                (&multi.sol, ChainType::Sol),
            ];

            let mut matched = false;
            for (addr, chain) in hits {
                if is_vanity_address(&addr.address, &patterns) {
                    matched = true;
                    found.fetch_add(1, Ordering::Relaxed);
                    print_multi_address(&multi, chain);
                    let _ = save_multi_address_to_file(&config.output_file, &multi, chain);
                }
            }

            if !matched && config.save_all {
                // 默认保存 TRON 以兼容旧格式
                let _ = save_address_to_file(&config.output_file, &multi.tron, false);
            }
        }
    }

    should_stop.store(true, Ordering::Relaxed);
    let _ = stats_thread.join();
    let _ = stop_thread.join();

    let elapsed = start.elapsed();
    let total_generated = counter.load(Ordering::Relaxed);
    let total_found = found.load(Ordering::Relaxed);

    println!("\n");
    println!(
        "{}",
        "╔════════════════════════════════════════════════════════════╗".bright_green()
    );
    println!(
        "{}",
        "║                    生成完成 (Finished)                     ║".bright_green()
    );
    println!(
        "{}",
        "╠════════════════════════════════════════════════════════════╣".bright_green()
    );
    println!(
        "{} {}",
        "总生成数 | Total Generated:".bright_white(),
        format!("{}", total_generated).bright_cyan()
    );
    println!(
        "{} {}",
        "找到靓号 | Vanity Found:".bright_white(),
        format!("{}", total_found).bright_yellow()
    );
    println!(
        "{} {}",
        "耗时 | Time Elapsed:".bright_white(),
        format!("{:.2?}", elapsed).bright_cyan()
    );
    println!(
        "{} {}",
        "平均速率 | Average Rate:".bright_white(),
        format!(
            "{:.0} addr/s",
            total_generated as f64 / elapsed.as_secs_f64()
        )
        .bright_cyan()
    );
    println!(
        "{} {}",
        "结果保存 | Results Saved:".bright_white(),
        config.output_file.bright_yellow()
    );
    println!(
        "{}",
        "╚════════════════════════════════════════════════════════════╝".bright_green()
    );
}

fn benchmark_generation() {
    println!();
    println!(
        "{}",
        "╔════════════════════════════════════════════════════════════╗".bright_cyan()
    );
    println!(
        "{}",
        "║                  性能测试 (Benchmark)                      ║".bright_cyan()
    );
    println!(
        "{}",
        "╚════════════════════════════════════════════════════════════╝".bright_cyan()
    );

    let num_threads = num_cpus::get();
    println!(
        "{} {}",
        "检测到 CPU 核心数 | CPU Cores:".bright_yellow(),
        format!("{}", num_threads).bright_cyan()
    );
    println!();

    // 单线程测试
    println!(
        "{}",
        "► 单线程性能测试 (Single-threaded benchmark)...".bright_green()
    );
    let start = Instant::now();
    for _ in 0..100 {
        let _ = generate_tron_address();
    }
    let elapsed = start.elapsed();
    let rate = 100.0 / elapsed.as_secs_f64();
    println!(
        "{} 100 个地址生成耗时: {:.2?} | 速率: {:.0} addr/s",
        "✓".bright_green(),
        elapsed,
        rate
    );
    println!();

    // 多线程测试
    println!(
        "{}",
        "► 多线程性能测试 (Multi-threaded benchmark)...".bright_green()
    );
    let start = Instant::now();
    let _results: Vec<_> = (0..1000)
        .into_par_iter()
        .map(|_| generate_tron_address())
        .collect();
    let elapsed = start.elapsed();
    let rate = 1000.0 / elapsed.as_secs_f64();
    println!(
        "{} 1000 个地址生成耗时: {:.2?} | 速率: {:.0} addr/s",
        "✓".bright_green(),
        elapsed,
        rate
    );
    println!();

    // 超大规模测试
    println!(
        "{}",
        "► 超大规模测试 (Large scale benchmark)...".bright_green()
    );
    let start = Instant::now();
    let _results: Vec<_> = (0..10000)
        .into_par_iter()
        .map(|_| generate_tron_address())
        .collect();
    let elapsed = start.elapsed();
    let rate = 10000.0 / elapsed.as_secs_f64();
    println!(
        "{} 10000 个地址生成耗时: {:.2?} | 速率: {:.0} addr/s",
        "✓".bright_green(),
        elapsed,
        rate
    );
    println!();

    println!(
        "{}",
        "═══════════════════════════════════════════════════════════".bright_cyan()
    );
    println!(
        "{}",
        "建议: 在实际生成时使用多线程模式以获得最佳性能".bright_yellow()
    );
    println!(
        "{}",
        "Tip: Use multi-threaded mode in production for best performance".bright_yellow()
    );
}
