use crate::collectors::{
    cpu::CpuInfo,
    disk::DiskInfo,
    memory::MemoryInfo,
    system::SystemInfo,
};

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
    println!("Uptime:   {} seconds", system.uptime);

    println!();

    println!("CPU:      {}", cpu.model);
    println!("Cores:    {}", cpu.cores);
    println!("Threads:  {}", cpu.threads);

    println!();

    println!("Memory:");
    println!("  Total:     {} bytes", memory.total);
    println!("  Used:      {} bytes", memory.used);
    println!("  Available: {} bytes", memory.available);
    println!(
        "  Swap:      {} / {} bytes",
        memory.swap_used,
        memory.swap_total
    );

    println!();

    println!("Disk:");

    for disk in disks {
        println!(
            "  {} ({})  {} / {} bytes",
            disk.mount_point,
            disk.name,
            disk.available,
            disk.total
        );
    }
}