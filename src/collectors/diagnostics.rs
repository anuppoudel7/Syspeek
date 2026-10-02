use super::{disk::DiskInfo, docker::DockerInfo, memory::MemoryInfo, services::ServiceInfo};

pub struct DiagnosticResult {
    pub name: String,
    pub status: DiagnosticStatus,
    pub message: String,
}

pub enum DiagnosticStatus {
    Ok,
    Warning,
    Critical,
}

const MEMORY_WARNING: f64 = 80.0;
const MEMORY_CRITICAL: f64 = 90.0;

const SWAP_WARNING: f64 = 70.0;
const SWAP_CRITICAL: f64 = 90.0;

const DISK_WARNING: f64 = 80.0;
const DISK_CRITICAL: f64 = 90.0;

pub fn collect(
    memory: &MemoryInfo,
    disks: &[DiskInfo],
    docker: &DockerInfo,
    services: &[ServiceInfo],
) -> Vec<DiagnosticResult> {
    let mut results = Vec::new();

    results.push(check_usage(
        "Memory",
        memory.used,
        memory.total,
        MEMORY_WARNING,
        MEMORY_CRITICAL,
    ));

    if memory.swap_total > 0 {
        results.push(check_usage(
            "Swap",
            memory.swap_used,
            memory.swap_total,
            SWAP_WARNING,
            SWAP_CRITICAL,
        ));
    }

    for disk in disks {
        results.push(check_disk(disk));
    }

    results.push(check_docker(docker));
    results.push(check_services(services));
    results
}

fn check_usage(
    name: &str,
    used: u64,
    total: u64,
    warning_threshold: f64,
    critical_threshold: f64,
) -> DiagnosticResult {
    if total == 0 {
        return DiagnosticResult {
            name: name.to_string(),
            status: DiagnosticStatus::Critical,
            message: "No capacity information available".to_string(),
        };
    }

    let usage = (used as f64 / total as f64) * 100.0;

    let status = if usage >= critical_threshold {
        DiagnosticStatus::Critical
    } else if usage >= warning_threshold {
        DiagnosticStatus::Warning
    } else {
        DiagnosticStatus::Ok
    };

    DiagnosticResult {
        name: name.to_string(),
        status,
        message: format!("{usage:.1}% used"),
    }
}

fn check_disk(disk: &DiskInfo) -> DiagnosticResult {
    let used = disk.total.saturating_sub(disk.available);

    let mut result = check_usage(
        &format!("Disk {}", disk.mount_point),
        used,
        disk.total,
        DISK_WARNING,
        DISK_CRITICAL,
    );

    let usage = (used as f64 / disk.total as f64) * 100.0;

    result.message = format!(
        "{usage:.1}% used, {} available",
        format_bytes(disk.available)
    );
    result
}

fn check_docker(docker: &DockerInfo) -> DiagnosticResult {
    if !docker.installed {
        return DiagnosticResult {
            name: "Docker".to_string(),
            status: DiagnosticStatus::Ok,
            message: "Not installed".to_string(),
        };
    }

    if !docker.daemon_running {
        return DiagnosticResult {
            name: "Docker".to_string(),
            status: DiagnosticStatus::Warning,
            message: "Daemon not running".to_string(),
        };
    }

    DiagnosticResult {
        name: "Docker".to_string(),
        status: DiagnosticStatus::Ok,
        message: "Daemon running".to_string(),
    }
}

fn check_services(services: &[ServiceInfo]) -> DiagnosticResult {
    let failed = services.iter().filter(|service| service.failed).count();

    if failed > 0 {
        return DiagnosticResult {
            name: "Services".to_string(),
            status: DiagnosticStatus::Critical,
            message: format!("{failed} failed"),
        };
    }

    DiagnosticResult {
        name: "Services".to_string(),
        status: DiagnosticStatus::Ok,
        message: "0 failed".to_string(),
    }
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];

    let mut value = bytes as f64;
    let mut unit = 0;

    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }

    format!("{value:.2} {}", UNITS[unit])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_ok() {
        let result = check_usage("Memory", 40, 100, 80.0, 90.0);

        assert!(matches!(result.status, DiagnosticStatus::Ok));
        assert_eq!(result.message, "40.0% used");
    }

    #[test]
    fn test_usage_warning() {
        let result = check_usage("Memory", 85, 100, 80.0, 90.0);

        assert!(matches!(result.status, DiagnosticStatus::Warning));
    }

    #[test]
    fn test_usage_critical() {
        let result = check_usage("Memory", 95, 100, 80.0, 90.0);

        assert!(matches!(result.status, DiagnosticStatus::Critical));
    }

    #[test]
    fn test_zero_total() {
        let result = check_usage("Memory", 0, 0, 80.0, 90.0);

        assert!(matches!(result.status, DiagnosticStatus::Critical));
        assert_eq!(result.message, "No capacity information available");
    }

    #[test]
    fn test_collect_memory_and_disks() {
        let memory = MemoryInfo {
            total: 100,
            used: 50,
            available: 50,
            swap_total: 100,
            swap_used: 20,
        };

        let disks = vec![DiskInfo {
            name: "test".to_string(),
            mount_point: "/".to_string(),
            total: 1000,
            available: 500,
        }];

        let docker = DockerInfo {
            installed: true,
            daemon_running: true,
            version: Some("Docker 29.8.1".to_string()),
            containers: Vec::new(),
            images: Vec::new(),
        };

        let services = vec![ServiceInfo {
            name: "ssh.service".to_string(),
            active_state: "active".to_string(),
            sub_state: "running".to_string(),
            failed: false,
        }];

        let results = collect(&memory, &disks, &docker, &services);

        assert_eq!(results.len(), 5);

        assert!(
            results
                .iter()
                .all(|result| { matches!(result.status, DiagnosticStatus::Ok) })
        );
    }

    #[test]
    fn test_disk_usage() {
        let disk = DiskInfo {
            name: "test".to_string(),
            mount_point: "/".to_string(),
            total: 100,
            available: 10,
        };

        let result = check_disk(&disk);

        assert!(matches!(result.status, DiagnosticStatus::Critical));
        assert_eq!(result.message, "90.0% used, 10.00 B available");
    }

    #[test]
    fn test_docker_running() {
        let docker = DockerInfo {
            installed: true,
            daemon_running: true,
            version: Some("Docker 29.8.1".to_string()),
            containers: Vec::new(),
            images: Vec::new(),
        };

        let result = check_docker(&docker);

        assert!(matches!(result.status, DiagnosticStatus::Ok));
        assert_eq!(result.message, "Daemon running");
    }

    #[test]
    fn test_docker_not_running() {
        let docker = DockerInfo {
            installed: true,
            daemon_running: false,
            version: Some("Docker 29.8.1".to_string()),
            containers: Vec::new(),
            images: Vec::new(),
        };

        let result = check_docker(&docker);

        assert!(matches!(result.status, DiagnosticStatus::Warning));
        assert_eq!(result.message, "Daemon not running");
    }

    #[test]
    fn test_docker_not_installed() {
        let docker = DockerInfo {
            installed: false,
            daemon_running: false,
            version: None,
            containers: Vec::new(),
            images: Vec::new(),
        };

        let result = check_docker(&docker);

        assert!(matches!(result.status, DiagnosticStatus::Ok));
        assert_eq!(result.message, "Not installed");
    }

    #[test]
    fn test_services_ok() {
        let services = vec![
            ServiceInfo {
                name: "ssh.service".to_string(),
                active_state: "active".to_string(),
                sub_state: "running".to_string(),
                failed: false,
            },
            ServiceInfo {
                name: "cron.service".to_string(),
                active_state: "active".to_string(),
                sub_state: "running".to_string(),
                failed: false,
            },
        ];

        let result = check_services(&services);

        assert!(matches!(result.status, DiagnosticStatus::Ok));
        assert_eq!(result.message, "0 failed");
    }

    #[test]
    fn test_services_failed() {
        let services = vec![
            ServiceInfo {
                name: "ssh.service".to_string(),
                active_state: "active".to_string(),
                sub_state: "running".to_string(),
                failed: false,
            },
            ServiceInfo {
                name: "broken.service".to_string(),
                active_state: "failed".to_string(),
                sub_state: "failed".to_string(),
                failed: true,
            },
        ];

        let result = check_services(&services);

        assert!(matches!(result.status, DiagnosticStatus::Critical));
        assert_eq!(result.message, "1 failed");
    }

    #[test]
    fn test_services_empty() {
        let result = check_services(&[]);

        assert!(matches!(result.status, DiagnosticStatus::Ok));
        assert_eq!(result.message, "0 failed");
    }
}
