use sysinfo::System;

pub struct CpuInfo {
    pub model: String,
    pub cores: usize,
    pub threads: usize,
}

pub fn collect() -> CpuInfo {
    let mut system = System::new();
    system.refresh_cpu_all();
    let cpus = system.cpus();
    let model = cpus
        .first()
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    let threads = cpus.len();
    let cores = System::physical_core_count().unwrap_or(threads);

    CpuInfo {
        model,
        cores,
        threads,
    }
}