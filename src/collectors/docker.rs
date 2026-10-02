use std::process::Command;

use anyhow::{Context, Result};

pub struct DockerInfo {
    pub installed: bool,
    pub daemon_running: bool,
    pub version: Option<String>,
    pub containers: Vec<DockerContainer>,
    pub images: Vec<DockerImage>,
}

pub struct DockerContainer {
    pub name: String,
    pub image: String,
    pub status: String,
}

pub struct DockerImage {
    pub repository: String,
    pub tag: String,
    pub size: String,
}

pub fn collect() -> Result<DockerInfo> {
    let version_output = match Command::new("docker").arg("--version").output() {
        Ok(output) => output,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(DockerInfo {
                installed: false,
                daemon_running: false,
                version: None,
                containers: Vec::new(),
                images: Vec::new(),
            });
        }
        Err(error) => {
            return Err(error).context("failed to execute docker");
        }
    };

    let version = if version_output.status.success() {
        String::from_utf8_lossy(&version_output.stdout)
            .trim()
            .to_string()
    } else {
        String::from_utf8_lossy(&version_output.stderr)
            .trim()
            .to_string()
    };

    let daemon_output = Command::new("docker")
        .arg("info")
        .output()
        .context("failed to execute docker info")?;

    if !daemon_output.status.success() {
        return Ok(DockerInfo {
            installed: true,
            daemon_running: false,
            version: Some(version),
            containers: Vec::new(),
            images: Vec::new(),
        });
    }

    let containers = collect_containers()?;
    let images = collect_images()?;

    Ok(DockerInfo {
        installed: true,
        daemon_running: true,
        version: Some(version),
        containers,
        images,
    })
}

fn collect_containers() -> Result<Vec<DockerContainer>> {
    let output = Command::new("docker")
        .args(["ps", "--format", "{{.Names}}\t{{.Image}}\t{{.Status}}"])
        .output()
        .context("failed to retrieve Docker containers")?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let containers = stdout
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(3, '\t');

            let name = parts.next()?.to_string();
            let image = parts.next()?.to_string();
            let status = parts.next()?.to_string();

            Some(DockerContainer {
                name,
                image,
                status,
            })
        })
        .collect();

    Ok(containers)
}

fn collect_images() -> Result<Vec<DockerImage>> {
    let output = Command::new("docker")
        .args(["images", "--format", "{{.Repository}}\t{{.Tag}}\t{{.Size}}"])
        .output()
        .context("failed to retrieve Docker images")?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let images = stdout
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(3, '\t');

            let repository = parts.next()?.to_string();
            let tag = parts.next()?.to_string();
            let size = parts.next()?.to_string();

            Some(DockerImage {
                repository,
                tag,
                size,
            })
        })
        .collect();

    Ok(images)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_docker() {
        let result = collect();

        assert!(result.is_ok());

        let docker = result.expect("Docker information should be collectable");

        if docker.installed {
            assert!(docker.version.is_some());

            if docker.daemon_running {
                assert!(docker.version.as_ref().is_some_and(|v| !v.is_empty()));
            }
        } else {
            assert!(!docker.daemon_running);
            assert!(docker.version.is_none());
            assert!(docker.containers.is_empty());
            assert!(docker.images.is_empty());
        }
    }
}
