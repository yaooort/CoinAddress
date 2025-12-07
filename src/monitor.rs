use sysinfo::System;

#[derive(Clone, Debug)]
pub struct SystemStats {
    pub cpu_percent: f32,
    pub memory_percent: f32,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
}

pub struct SystemMonitor {
    sys: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        SystemMonitor { sys }
    }

    pub fn get_stats(&mut self) -> SystemStats {
        self.sys.refresh_all();

        // 获取CPU占用
        let cpu_percent = self.sys.global_cpu_info().cpu_usage();

        // 获取内存占用
        let total_memory = self.sys.total_memory();
        let used_memory = self.sys.used_memory();
        let memory_percent = (used_memory as f32 / total_memory as f32) * 100.0;

        SystemStats {
            cpu_percent,
            memory_percent,
            memory_used_mb: used_memory / 1024,
            memory_total_mb: total_memory / 1024,
        }
    }
}
