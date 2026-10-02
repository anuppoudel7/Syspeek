use std::process::Command;

pub struct DevelopmentInfo {
    pub tools: Vec<DevelopmentTool>,
}

pub struct DevelopmentTool {
    pub name: String,
    pub version: Option<String>,
}

pub fn collect() -> DevelopmentInfo {
    let tools = vec![
        collect_tool("Git", "git", &["--version"]),
        collect_tool("Rust", "rustc", &["--version"]),
        collect_tool("Cargo", "cargo", &["--version"]),
        collect_tool("Python", "python", &["--version"]),
        collect_tool("pip", "pip", &["--version"]),
        collect_tool("Node.js", "node", &["--version"]),
        collect_tool("npm", "npm", &["--version"]),
        collect_tool("Java", "java", &["--version"]),
        collect_tool("GCC", "gcc", &["--version"]),
        collect_tool("Clang", "clang", &["--version"]),
    ];

    DevelopmentInfo { tools }
}

fn collect_tool(name: &str, command: &str, args: &[&str]) -> DevelopmentTool {
    let version = Command::new(command)
        .args(args)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            let output = if stdout.trim().is_empty() {
                stderr
            } else {
                stdout
            };

            output
                .lines()
                .find(|line| !line.trim().is_empty())
                .map(|line| line.trim().to_string())
        });

    DevelopmentTool {
        name: name.to_string(),
        version,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_development() {
        let development = collect();

        assert_eq!(development.tools.len(), 10);

        for tool in development.tools {
            assert!(!tool.name.is_empty());

            if let Some(version) = tool.version {
                assert!(!version.is_empty());
            }
        }
    }

    #[test]
    fn test_collect_missing_tool() {
        let tool = collect_tool(
            "Missing Tool",
            "syspeek-command-that-does-not-exist",
            &["--version"],
        );

        assert_eq!(tool.name, "Missing Tool");
        assert!(tool.version.is_none());
    }
}
