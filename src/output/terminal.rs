use crate::collectors::{
    cpu::CpuInfo, development::DevelopmentInfo, disk::DiskInfo, docker::DockerInfo,
    hardware::HardwareInfo, memory::MemoryInfo, network::NetworkInterface, services::ServiceInfo,
    system::SystemInfo,
};

use super::{
    format::{format_bytes, format_uptime},
    logo::LOGO,
};

pub fn print_network_info(interfaces: &[NetworkInterface]) {
    println!("Network Information");
    println!("───────────────────");
    for interface in interfaces {
        println!();
        println!("Interface {}", interface.name);
        println!("State     {}", interface.state);
        if let Some(mac) = &interface.mac {
            println!("MAC       {}", mac);
        }
        for ipv4 in &interface.ipv4 {
            println!("IPv4      {}", ipv4);
        }
        for ipv6 in &interface.ipv6 {
            println!("IPv6      {}", ipv6);
        }
    }
}

pub fn print_system_info(system: &SystemInfo) {
    println!("System Information");
    println!("──────────────────");
    println!("OS        {}", system.os);
    println!("Kernel    {}", system.kernel);
    println!("Host      {}", system.hostname);
    println!("Uptime    {}", format_uptime(system.uptime));
}

pub fn print_hardware_info(hardware: &HardwareInfo) {
    println!("Hardware Information");
    println!("────────────────────");

    println!();
    println!("CPU       {}", hardware.cpu.model);
    println!("Cores     {}", hardware.cpu.cores);
    println!("Threads   {}", hardware.cpu.threads);

    println!();

    if hardware.gpus.is_empty() {
        println!("GPU       Not detected");
    } else {
        for (index, gpu) in hardware.gpus.iter().enumerate() {
            let label = if hardware.gpus.len() == 1 {
                "GPU".to_string()
            } else {
                format!("GPU {}", index + 1)
            };

            println!("{:<9}{}", label, gpu.name);

            if let Some(vendor) = &gpu.vendor {
                println!("{:<9}{}", "Vendor", vendor);
            }

            if let Some(device) = &gpu.device {
                println!("{:<9}0x{}", "Device", device.trim_start_matches("0x"));
            }

            if index + 1 < hardware.gpus.len() {
                println!();
            }
        }
    }

    println!();

    println!(
        "Memory    {} / {}",
        format_bytes(hardware.memory.used),
        format_bytes(hardware.memory.total)
    );

    println!("Available {}", format_bytes(hardware.memory.available));

    println!(
        "Swap      {} / {}",
        format_bytes(hardware.memory.swap_used),
        format_bytes(hardware.memory.swap_total)
    );

    println!();

    for disk in &hardware.disks {
        println!(
            "Disk {}   {} / {} ({})",
            disk.mount_point,
            format_bytes(disk.available),
            format_bytes(disk.total),
            disk.name
        );
    }
}

pub fn print_system(system: &SystemInfo, cpu: &CpuInfo, memory: &MemoryInfo, disks: &[DiskInfo]) {
    let mut info = Vec::new();

    info.push(String::from("syspeek"));
    info.push(String::from("────────────────────────"));
    info.push(format!("OS        {}", system.os));
    info.push(format!("Kernel    {}", system.kernel));
    info.push(format!("Host      {}", system.hostname));
    info.push(format!("Uptime    {}", format_uptime(system.uptime)));

    info.push(String::new());

    info.push(format!("CPU       {}", cpu.model));
    info.push(format!("Cores     {}", cpu.cores));
    info.push(format!("Threads   {}", cpu.threads));

    info.push(String::new());

    info.push(format!(
        "Memory    {} / {}",
        format_bytes(memory.used),
        format_bytes(memory.total)
    ));

    info.push(format!("Available {}", format_bytes(memory.available)));

    info.push(format!(
        "Swap      {} / {}",
        format_bytes(memory.swap_used),
        format_bytes(memory.swap_total)
    ));

    info.push(String::new());

    for disk in disks {
        info.push(format!(
            "Disk {}   {} / {} ({})",
            disk.mount_point,
            format_bytes(disk.available),
            format_bytes(disk.total),
            disk.name
        ));
    }

    let logo_width = LOGO
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    let gap = 4;
    let height = LOGO.len().max(info.len());

    for i in 0..height {
        let logo = LOGO.get(i).copied().unwrap_or("");
        let text = info.get(i).map(String::as_str).unwrap_or("");

        if i < LOGO.len() {
            println!(
                "{:<width$}{}{}",
                logo,
                " ".repeat(gap),
                text,
                width = logo_width
            );
        } else {
            println!("{}", text);
        }
    }
}

pub fn print_services_info(services: &[ServiceInfo]) {
    let running = services
        .iter()
        .filter(|service| service.is_running())
        .count();

    let active = services
        .iter()
        .filter(|service| service.active_state == "active")
        .count();

    let failed = services.iter().filter(|service| service.failed).count();

    println!("System Services");
    println!("───────────────");
    println!();
    println!("Active      {}", active);
    println!("Running     {}", running);
    println!("Failed      {}", failed);

    if failed > 0 {
        println!();
        println!("Failed Services");
        println!("───────────────");

        for service in services.iter().filter(|service| service.failed) {
            println!("{}", service.name);
        }
    }
}

pub fn print_docker_info(docker: &DockerInfo) {
    println!("Docker Information");
    println!("──────────────────");

    if !docker.installed {
        println!();
        println!("Status      Not installed");
        return;
    }

    println!();
    println!(
        "Version     {}",
        docker.version.as_deref().unwrap_or("Unknown")
    );

    if !docker.daemon_running {
        println!("Daemon      Not running");
        return;
    }

    println!("Daemon      Running");

    println!();
    println!("Containers");
    println!("──────────");

    if docker.containers.is_empty() {
        println!("None");
    } else {
        println!("{:<20} {:<30} STATUS", "NAME", "IMAGE");
        for container in &docker.containers {
            println!(
                "{:<20} {:<30} {}",
                container.name, container.image, container.status
            );
        }
    }

    println!();
    println!("Images");
    println!("──────");

    if docker.images.is_empty() {
        println!("None");
    } else {
        println!("{:<30} {:<15} SIZE", "REPOSITORY", "TAG");
        for image in &docker.images {
            println!("{:<30} {:<15} {}", image.repository, image.tag, image.size);
        }
    }
}
pub fn print_development_info(development: &DevelopmentInfo) {
    println!("Development Environment");
    println!("───────────────────────");

    for tool in &development.tools {
        println!();

        println!("{}", tool.name);

        match &tool.version {
            Some(version) => {
                println!("    Version     {}", version);
            }
            None => {
                println!("    Status      Not installed");
            }
        }
    }
}
