use std::process::Command;

use anyhow::{Context, Result};

pub struct ServiceInfo {
    pub name: String,
    pub active_state: String,
    pub sub_state: String,
    pub failed: bool,
}

impl ServiceInfo {
    pub fn is_running(&self) -> bool {
        self.active_state == "active" && self.sub_state == "running"
    }
}

pub fn collect() -> Result<Vec<ServiceInfo>> {
    let output = Command::new("systemctl")
        .args([
            "list-units",
            "--type=service",
            "--all",
            "--no-legend",
            "--no-pager",
        ])
        .output()
        .context("failed to execute systemctl")?;

    if !output.status.success() {
        anyhow::bail!(
            "systemctl returned an error: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    let stdout =
        String::from_utf8(output.stdout).context("systemctl returned invalid UTF-8 output")?;

    Ok(stdout.lines().filter_map(parse_service).collect())
}

fn parse_service(line: &str) -> Option<ServiceInfo> {
    let fields: Vec<&str> = line.split_whitespace().collect();

    if fields.len() < 4 {
        return None;
    }

    let name = fields[0].to_string();
    let active_state = fields[2].to_string();
    let sub_state = fields[3].to_string();

    Some(ServiceInfo {
        name,
        failed: active_state == "failed" || sub_state == "failed",
        active_state,
        sub_state,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_service() {
        let line = "sshd.service loaded active running OpenSSH server daemon";

        let service = parse_service(line).expect("service should parse");

        assert_eq!(service.name, "sshd.service");
        assert_eq!(service.active_state, "active");
        assert_eq!(service.sub_state, "running");
        assert!(!service.failed);
        assert!(service.is_running());
    }

    #[test]
    fn test_parse_failed_service() {
        let line = "example.service loaded failed failed Example service";

        let service = parse_service(line).expect("service should parse");

        assert_eq!(service.name, "example.service");
        assert_eq!(service.active_state, "failed");
        assert_eq!(service.sub_state, "failed");
        assert!(service.failed);
        assert!(!service.is_running());
    }

    #[test]
    fn test_parse_invalid_service() {
        let line = "invalid";

        assert!(parse_service(line).is_none());
    }

    #[test]
    fn test_active_but_not_running_service() {
        let line = "example.service loaded active exited Example service";

        let service = parse_service(line).expect("service should parse");

        assert_eq!(service.active_state, "active");
        assert_eq!(service.sub_state, "exited");
        assert!(!service.failed);
        assert!(!service.is_running());
    }
}
