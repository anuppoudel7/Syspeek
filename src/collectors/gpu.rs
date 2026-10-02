use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};

pub struct GpuInfo {
    pub name: String,
    pub vendor: Option<String>,
    pub device: Option<String>,
}

pub fn collect() -> Result<Vec<GpuInfo>> {
    let lspci_gpus = collect_lspci_gpus();

    if !lspci_gpus.is_empty() {
        return Ok(lspci_gpus
            .into_iter()
            .map(|(name, vendor, device)| GpuInfo {
                vendor: vendor.map(|value| vendor_name(&value)),
                device,
                name,
            })
            .collect());
    }

    collect_sysfs_gpus()
}

fn collect_lspci_gpus() -> Vec<(String, Option<String>, Option<String>)> {
    let output = match Command::new("lspci").arg("-nn").output() {
        Ok(output) if output.status.success() => output,
        _ => return Vec::new(),
    };

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout.lines().filter_map(parse_lspci_gpu).collect()
}

fn collect_sysfs_gpus() -> Result<Vec<GpuInfo>> {
    let drm_path = Path::new("/sys/class/drm");

    if !drm_path.exists() {
        return Ok(Vec::new());
    }

    let devices = find_gpu_devices(drm_path)?;

    let gpus = devices
        .into_iter()
        .map(|device_path| {
            let vendor = read_file(device_path.join("vendor"));
            let device = read_file(device_path.join("device"));

            let name = match (&vendor, &device) {
                (Some(vendor), Some(device)) => format!("{vendor}:{device}"),
                _ => "Unknown GPU".to_string(),
            };

            GpuInfo {
                name,
                vendor: vendor.as_deref().map(vendor_name),
                device,
            }
        })
        .collect();

    Ok(gpus)
}

fn find_gpu_devices(drm_path: &Path) -> Result<Vec<PathBuf>> {
    let entries = fs::read_dir(drm_path).context("failed to read /sys/class/drm")?;

    let mut devices = Vec::new();

    for entry in entries {
        let entry = entry.context("failed to read DRM device entry")?;

        let name = entry.file_name();
        let name = name.to_string_lossy();

        if !name.starts_with("card") || name.contains('-') {
            continue;
        }

        let device_path = entry.path().join("device");

        if device_path.exists() {
            devices.push(device_path);
        }
    }

    devices.sort();

    Ok(devices)
}

fn parse_lspci_gpu(line: &str) -> Option<(String, Option<String>, Option<String>)> {
    let lower = line.to_ascii_lowercase();

    let is_gpu = lower.contains("vga compatible controller")
        || lower.contains("3d controller")
        || lower.contains("display controller");

    if !is_gpu {
        return None;
    }

    let description = line.split_once(": ")?.1.trim();

    let name = description
        .split_once(" [")
        .map(|(name, _)| name.trim())
        .unwrap_or(description)
        .trim_end_matches([' ', ')'])
        .to_string();

    let pci_id = description
        .split('[')
        .filter_map(|part| part.split_once(']'))
        .map(|(value, _)| value)
        .find(|value| {
            let mut parts = value.split(':');
            matches!(
                (parts.next(), parts.next()),
                (Some(vendor), Some(device))
                    if vendor.len() == 4 && device.len() == 4
            )
        });

    let (vendor, device) = pci_id
        .and_then(|value| value.split_once(':'))
        .map(|(vendor, device)| (Some(vendor.to_string()), Some(device.to_string())))
        .unwrap_or((None, None));

    Some((name, vendor, device))
}

fn read_file(path: impl AsRef<Path>) -> Option<String> {
    fs::read_to_string(path)
        .ok()
        .map(|value| value.trim().to_string())
}

fn vendor_name(vendor: &str) -> String {
    let vendor = vendor.strip_prefix("0x").unwrap_or(vendor);

    match vendor {
        "10de" => "NVIDIA".to_string(),
        "1002" => "AMD".to_string(),
        "8086" => "Intel".to_string(),
        _ => vendor.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_gpu() {
        let result = collect();

        assert!(result.is_ok());

        for gpu in result.expect("GPU information should be collectable") {
            assert!(!gpu.name.is_empty());

            if let Some(vendor) = gpu.vendor {
                assert!(!vendor.is_empty());
            }

            if let Some(device) = gpu.device {
                assert!(!device.is_empty());
            }
        }
    }

    #[test]
    fn test_vendor_name() {
        assert_eq!(vendor_name("0x10de"), "NVIDIA");
        assert_eq!(vendor_name("0x1002"), "AMD");
        assert_eq!(vendor_name("0x8086"), "Intel");
        assert_eq!(vendor_name("8086"), "Intel");
        assert_eq!(vendor_name("unknown"), "unknown");
    }

    #[test]
    fn test_parse_lspci_gpu() {
        let line = "0000:00:02.0 VGA compatible controller [0300]: Intel Corporation Alder Lake-P Integrated Graphics Controller [8086:4626] (rev 0c)";

        let result = parse_lspci_gpu(line);

        assert_eq!(
            result,
            Some((
                "Intel Corporation Alder Lake-P Integrated Graphics Controller".to_string(),
                Some("8086".to_string()),
                Some("4626".to_string()),
            ))
        );
    }

    #[test]
    fn test_parse_nvidia_gpu() {
        let line = "0000:01:00.0 VGA compatible controller [0300]: NVIDIA Corporation GA107M [GeForce RTX 3050 Mobile] [10de:25a2] (rev a1)";

        let result = parse_lspci_gpu(line);

        assert_eq!(
            result,
            Some((
                "NVIDIA Corporation GA107M".to_string(),
                Some("10de".to_string()),
                Some("25a2".to_string()),
            ))
        );
    }

    #[test]
    fn test_parse_non_gpu_lspci() {
        let line = "00:14.0 USB controller: Intel Corporation USB Controller";

        assert!(parse_lspci_gpu(line).is_none());
    }

    #[test]
    fn test_read_missing_file() {
        assert!(read_file("/path/that/does/not/exist").is_none());
    }
}
