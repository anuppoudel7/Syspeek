use sysinfo::System;

pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub swap_total: u64,
    pub swap_used: u64,
}

pub fn collect() -> MemoryInfo {
    let mut system = System::new();

    system.refresh_memory();

    MemoryInfo {
        total: system.total_memory(),
        used: system.used_memory(),
        available: system.available_memory(),
        swap_total: system.total_swap(),
        swap_used: system.used_swap(),
    }
}