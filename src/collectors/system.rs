use sysinfo::System;
pub struct SystemInfo {
    pub os: String,
    pub kernel: String,
    pub hostname: String,
    pub uptime: u64,
}

pub fn collect() -> SystemInfo {
    SystemInfo {
        os: System::long_os_version().unwrap_or_else(|| "Unknown".to_string()),
        kernel: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
        hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
        uptime: System::uptime(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_system() {
        let system = collect();

        assert!(!system.os.is_empty());
        assert!(!system.kernel.is_empty());
        assert!(!system.hostname.is_empty());
    }
}
