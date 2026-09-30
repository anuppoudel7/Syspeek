# syspeek

A fast and lightweight Linux system information and diagnostics CLI written in Rust.

`syspeek` is a Linux-focused command-line utility for viewing system, hardware, network, and system service information directly from the terminal.

## Overview

`syspeek` collects information from the Linux system and presents it in a clean, human-readable terminal interface.

It is designed with a simple separation between data collection, data representation, formatting, and terminal output.

## Features

### System Information

Displays:

- Operating system
- Kernel version
- Hostname
- System uptime

### Hardware Information

Displays:

- CPU model
- Physical CPU cores
- CPU threads
- Memory usage
- Available memory
- Swap usage
- Disk usage

### Network Information

Displays:

- Network interfaces
- Interface state
- MAC addresses
- IPv4 addresses
- IPv6 addresses

### System Services

Displays:

- Active services
- Running services
- Failed services
- Failed service names

### Other Features

- Human-readable terminal output
- Command-based CLI
- Error handling
- Unit tests
- Linux-focused system information
- Lightweight implementation written in Rust

## Commands

| Command | Description |
|---|---|
| `syspeek` | Display a system overview |
| `syspeek system` | Display system information |
| `syspeek hardware` | Display hardware information |
| `syspeek network` | Display network information |
| `syspeek services` | Display system service information |
| `syspeek docker` | Coming soon |
| `syspeek development` | Coming soon |
| `syspeek monitor` | Coming soon |
| `syspeek diagnose` | Coming soon |

## Example

Running:

```bash
syspeek
```
displays an overview similar to:
███████╗██╗   ██╗███████╗██████╗ ███████╗███████╗██╗  ██╗    syspeek
██╔════╝╚██╗ ██╔╝██╔════╝██╔══██╗██╔════╝██╔════╝██║ ██╔╝    ────────────────────────
███████╗ ╚████╔╝ █████╗  ██████╔╝█████╗  █████╗  █████╔╝     OS        Linux
╚════██║  ╚██╔╝  ██╔══╝  ██╔═══╝ ██╔══╝  ██╔══╝  ██╔═██╗     Kernel    Linux 7.x
███████║   ██║   ███████╗██║     ███████╗███████╗██║  ██╗    Host      my-machine
╚══════╝   ╚═╝   ╚══════╝╚═╝     ╚══════╝╚══════╝╚═╝  ╚═╝    Uptime    2h 52m

CPU       12th Gen Intel(R) Core(TM) i5
Cores     12
Threads   16

Memory    4.99 GiB / 7.46 GiB
Available 2.47 GiB
Swap      1.10 GiB / 3.73 GiB

Disk /    4.34 GiB / 91.11 GiB
Disk /home 208.37 GiB / 375.80 GiB

## Architecture
syspeek follows a simple separation-of-concerns architecture:
                 ┌──────────────┐
                 │   CLI Input  │
                 └──────┬───────┘
                        │
                        ▼
                 ┌──────────────┐
                 │  Collectors  │
                 └──────┬───────┘
                        │
                        ▼
                 ┌──────────────┐
                 │     Data     │
                 └──────┬───────┘
                        │
                        ▼
                 ┌──────────────┐
                 │  Formatters  │
                 └──────┬───────┘
                        │
                        ▼
                 ┌──────────────┐
                 │    Output    │
                 └──────────────┘

The core design principle is:
Collector → Data → Formatter → Renderer

## Collectors
Collectors are responsible for retrieving information from the Linux system.

Current collectors:
cpu — CPU information
memory — memory and swap information
disk — disk information
network — network interfaces and addresses
services — systemd service information
system — operating system, kernel, hostname, and uptime

## Output
The output layer is responsible for formatting collected data and presenting it in the terminal. This keeps system information collection independent from the presentation layer.

## Project Structure
src/
├── cli.rs
├── main.rs
├── collectors/
│   ├── cpu.rs
│   ├── disk.rs
│   ├── memory.rs
│   ├── network.rs
│   ├── services.rs
│   └── system.rs
└── output/
    ├── format.rs
    ├── logo.rs
    └── terminal.rs

## Tech Stack

Language: Rust
CLI: clap
System information: sysinfo
Error handling: anyhow
Serialization: serde / serde_json
Network interfaces: nix
Platform: Linux

## Testing

syspeek includes unit tests for its collectors and utility functions.
The current test suite covers:
Byte formatting
Uptime formatting
CPU collection
Memory collection
Disk collection
Network collection
System information collection
Systemd service parsing
Failed service detection
Service running-state detection
Invalid service input

## Code Quality
The project is checked using standard Rust development tools:
```bash
cargo fmt --check
cargo check
cargo clippy
cargo test
cargo build --release
```
current status:
```bash 
cargo fmt --check     ✓
cargo check           ✓
cargo clippy          ✓
cargo test            ✓
cargo build --release ✓
```

## Roadmap
Planned features include:

Docker information
Development environment information
System monitoring
System diagnostics
More detailed hardware information
Additional Linux system information
Improved terminal output
JSON/YAML output
Arch Linux package
AUR package

## Version
Current version: v0.1.0
v0.1.0 focuses on the core system information, hardware information, network information, system service information, error handling, testing, and CLI structure.
