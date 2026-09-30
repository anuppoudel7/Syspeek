use anyhow::{Result, bail};
use sysinfo::System;

pub struct CpuInfo {
    pub model: String,
    pub cores: usize,
    pub threads: usize,
}

pub fn collect() -> Result<CpuInfo> {
    let mut system = System::new();

    system.refresh_cpu_all();

    let cpus = system.cpus();

    if cpus.is_empty() {
        bail!("no CPU information available");
    }

    let model = cpus
        .first()
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let threads = cpus.len();

    let cores = System::physical_core_count().unwrap_or(threads);

    Ok(CpuInfo {
        model,
        cores,
        threads,
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_cpu() {
        let result = collect();

        assert!(result.is_ok());

        let cpu = result.expect("CPU information should be available");

        assert!(!cpu.model.is_empty());
        assert!(cpu.cores > 0);
        assert!(cpu.threads > 0);
        assert!(cpu.threads >= cpu.cores);
    }
}
