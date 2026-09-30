use anyhow::{Result, bail};
use sysinfo::System;

pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub swap_total: u64,
    pub swap_used: u64,
}

pub fn collect() -> Result<MemoryInfo> {
    let mut system = System::new();

    system.refresh_memory();

    let total = system.total_memory();

    if total == 0 {
        bail!("no memory information available");
    }

    Ok(MemoryInfo {
        total,
        used: system.used_memory(),
        available: system.available_memory(),
        swap_total: system.total_swap(),
        swap_used: system.used_swap(),
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_memory() {
        let result = collect();

        assert!(result.is_ok());

        let memory = result.expect("memory information should be available");

        assert!(memory.total > 0);
        assert!(memory.used <= memory.total);
        assert!(memory.available <= memory.total);
        assert!(memory.swap_used <= memory.swap_total);
    }
}
