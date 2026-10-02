use anyhow::{Context, Result};

use super::{cpu, disk, gpu, memory};

pub struct HardwareInfo {
    pub cpu: cpu::CpuInfo,
    pub memory: memory::MemoryInfo,
    pub disks: Vec<disk::DiskInfo>,
    pub gpus: Vec<gpu::GpuInfo>,
}

pub fn collect() -> Result<HardwareInfo> {
    let cpu = cpu::collect().context("failed to collect CPU information")?;

    let memory = memory::collect().context("failed to collect memory information")?;

    let disks = disk::collect().context("failed to collect disk information")?;

    let gpus = gpu::collect().context("failed to collect GPU information")?;

    Ok(HardwareInfo {
        cpu,
        memory,
        disks,
        gpus,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_hardware() {
        let result = collect();

        assert!(result.is_ok());

        let hardware = result.expect("hardware information should be available");

        assert!(!hardware.cpu.model.is_empty());
        assert!(hardware.cpu.cores > 0);
        assert!(hardware.cpu.threads > 0);
        assert!(hardware.cpu.threads >= hardware.cpu.cores);

        assert!(hardware.memory.total > 0);
        assert!(!hardware.disks.is_empty());
    }
}
