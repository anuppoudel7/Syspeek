use std::fs;
use std::net::IpAddr;
use std::path::Path;

use anyhow::{Context, Result};
use nix::ifaddrs::getifaddrs;

pub struct NetworkInterface {
    pub name: String,
    pub state: String,
    pub mac: Option<String>,
    pub ipv4: Vec<String>,
    pub ipv6: Vec<String>,
}

pub fn collect() -> Result<Vec<NetworkInterface>> {
    let mut interfaces = Vec::new();

    let net_path = Path::new("/sys/class/net");

    let entries = fs::read_dir(net_path).context("failed to read /sys/class/net")?;

    for entry in entries {
        let entry = entry.context("failed to read network interface entry")?;

        let path = entry.path();

        let name = match entry.file_name().into_string() {
            Ok(name) => name,
            Err(_) => continue,
        };

        let state = read_file(path.join("operstate")).unwrap_or_else(|| "unknown".to_string());

        let mac = read_file(path.join("address"));

        interfaces.push(NetworkInterface {
            name,
            state,
            mac,
            ipv4: Vec::new(),
            ipv6: Vec::new(),
        });
    }

    let addrs = getifaddrs().context("failed to retrieve network addresses")?;

    for address in addrs {
        let Some(interface) = interfaces
            .iter_mut()
            .find(|interface| interface.name == address.interface_name)
        else {
            continue;
        };

        let Some(sockaddr) = address.address else {
            continue;
        };

        if let Some(ip) = sockaddr.as_sockaddr_in() {
            let ip = IpAddr::V4(ip.ip());
            let value = ip.to_string();

            if !interface.ipv4.contains(&value) {
                interface.ipv4.push(value);
            }
        }

        if let Some(ip) = sockaddr.as_sockaddr_in6() {
            let ip = IpAddr::V6(ip.ip());
            let value = ip.to_string();

            if !interface.ipv6.contains(&value) {
                interface.ipv6.push(value);
            }
        }
    }

    if interfaces.is_empty() {
        anyhow::bail!("no network interfaces found");
    }

    Ok(interfaces)
}

fn read_file(path: impl AsRef<Path>) -> Option<String> {
    fs::read_to_string(path)
        .ok()
        .map(|value| value.trim().to_string())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_network() {
        let result = collect();

        assert!(result.is_ok());

        let interfaces = result.expect("network interfaces should be available");

        assert!(!interfaces.is_empty());

        for interface in interfaces {
            assert!(!interface.name.is_empty());
            assert!(!interface.state.is_empty());

            for ipv4 in &interface.ipv4 {
                assert!(ipv4.parse::<std::net::Ipv4Addr>().is_ok());
            }

            for ipv6 in &interface.ipv6 {
                assert!(ipv6.parse::<std::net::Ipv6Addr>().is_ok());
            }
        }
    }
}
