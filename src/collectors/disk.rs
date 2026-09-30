use anyhow::{Result, bail};
use sysinfo::Disks;

pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total: u64,
    pub available: u64,
}

pub fn collect() -> Result<Vec<DiskInfo>> {
    let disks = Disks::new_with_refreshed_list();

    if disks.is_empty() {
        bail!("no disk information available");
    }

    let disks = disks
        .iter()
        .map(|disk| DiskInfo {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            total: disk.total_space(),
            available: disk.available_space(),
        })
        .collect();

    Ok(disks)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_disks() {
        let result = collect();

        assert!(result.is_ok());

        let disks = result.expect("disk information should be available");

        assert!(!disks.is_empty());

        for disk in disks {
            assert!(!disk.mount_point.is_empty());
            assert!(disk.total > 0);
            assert!(disk.available <= disk.total);
        }
    }
}
