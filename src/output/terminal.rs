use crate::collectors::{
    cpu::CpuInfo,
    disk::DiskInfo,
    memory::MemoryInfo,
    system::SystemInfo,
};

use super::format::{format_bytes, format_uptime};

pub fn print_system(
    system: &SystemInfo,
    cpu: &CpuInfo,
    memory: &MemoryInfo,
    disks: &[DiskInfo],
) {
    println!("syspeek");
    println!("--------");

    println!("OS:       {}", system.os);
    println!("Kernel:   {}", system.kernel);
    println!("Hostname: {}", system.hostname);
    println!("Uptime:   {}", format_uptime(system.uptime));
    println!();

    println!("CPU:      {}", cpu.model);
    println!("Cores:    {}", cpu.cores);
    println!("Threads:  {}", cpu.threads);

    println!();

    println!("Memory:");
    println!("  Total:     {}", format_bytes(memory.total));
    println!("  Used:      {}", format_bytes(memory.used));
    println!("  Available: {}", format_bytes(memory.available));
    println!(
        "  Swap:      {} / {}",
        format_bytes(memory.swap_used),
        format_bytes(memory.swap_total)
    );

    println!();

    println!("Disk:");
    for disk in disks {
    println!(
        "  {} ({})  {} / {}",
        disk.mount_point,
        disk.name,
        format_bytes(disk.available),
        format_bytes(disk.total)
    );
    }
}