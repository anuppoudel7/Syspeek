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
- GPU information
- GPU vendor and device ID
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

### Docker Information

Displays:

- Docker installation status
- Docker version
- Docker daemon status
- Running containers
- Container image and status
- Docker images and sizes

### Development Environment

Detects installed development tools and their versions:

- Git
- Rust
- Cargo
- Python
- pip
- Node.js
- npm
- Java
- GCC
- Clang

### System Diagnostics

Checks important system conditions:

- Memory usage
- Swap usage
- Disk usage
- Docker daemon status
- Failed system services

Each diagnostic check reports one of:

- `OK`
- `WARNING`
- `CRITICAL`

A summary is displayed after all checks.

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
| `syspeek docker` | Display Docker information |
| `syspeek development` | Display development environment information |
| `syspeek monitor` | Display system monitoring information |
| `syspeek diagnose` | Run system diagnostic checks |

## Examples

Running:

```text
syspeek
```
displays an overview similar to:
```bash
███████╗██╗   ██╗███████╗██████╗ ███████╗███████╗██╗  ██╗    syspeek
██╔════╝╚██╗ ██╔╝██╔════╝██╔══██╗██╔════╝██╔════╝██║ ██╔╝    ────────────────────────
███████╗ ╚████╔╝ ███████╗██████╔╝█████╗  █████╗  █████╔╝     OS        Linux
╚════██║  ╚██╔╝  ╚════██║██╔═══╝ ██╔══╝  ██╔══╝  ██╔═██╗     Kernel    Linux 7.x
███████║   ██║   ███████║██║     ███████╗███████╗██║  ██╗    Host      my-machine
╚══════╝   ╚═╝   ╚══════╝╚═╝     ╚══════╝╚══════╝╚═╝  ╚═╝    Uptime    2h 52m

CPU       12th Gen Intel(R) Core(TM) i5
Cores     12
Threads   16

Memory    4.99 GiB / 7.46 GiB
Available 2.47 GiB
Swap      1.10 GiB / 3.73 GiB

Disk /    4.34 GiB / 91.11 GiB
Disk /home 208.37 GiB / 375.80 GiB
```
```text
syspeek hardware
```
displays output similar to:
```bash
Hardware Information
────────────────────

CPU       12th Gen Intel(R) Core(TM) i5-12500H
Cores     12
Threads   16

GPU 1     Intel Corporation Alder Lake-P Integrated Graphics Controller
Vendor    Intel
Device    0x4626

GPU 2     NVIDIA Corporation GA107M
Vendor    NVIDIA
Device    0x25a2

Memory    5.67 GiB / 7.46 GiB
Available 1.78 GiB
Swap      2.62 GiB / 3.73 GiB
```

```text
syspeek docker
```
```bash
Docker Information
──────────────────

Version     Docker version 29.8.1
Daemon      Running

Containers
──────────
conatiner--info(none)

Images
──────
REPOSITORY                    TAG             SIZE
saffronmart-api              latest          744MB
saffronmart-frontend         latest          93.7MB
```

```text
syspeek development
```
Displays detected development tools and their versions.

```text
syspeek diagnose
```
```bash
Diagnostics
───────────

✓ Memory               OK        72.7% used
! Swap                 WARNING   70.5% used
✗ Disk /               CRITICAL  95.1% used, 4.45 GiB available
✓ Disk /boot           OK        18.6% used, 775.10 MiB available
✓ Disk /home           OK        45.0% used, 206.63 GiB available
✓ Docker               OK        Daemon running
✓ Services             OK        0 failed

Summary: 5 OK, 1 WARNING, 1 CRITICAL
```

## Architecture
syspeek follows a simple separation-of-concerns architecture:
```text
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
```

The core design principle is:
Collector → Data → Formatter → Renderer
Collectors retrieve system information, while the output layer is responsible for formatting and presenting that information.

## Collectors
Collectors are responsible for retrieving information from the Linux system.

Current collectors:
| Collector     | Responsibility                          |
| ------------- | --------------------------------------- |
| `cpu`         | CPU model, cores, and threads           |
| `memory`      | Memory and swap information             |
| `disk`        | Disk and filesystem information         |
| `gpu`         | GPU detection and identification        |
| `network`     | Network interfaces and addresses        |
| `services`    | systemd service information             |
| `system`      | OS, kernel, hostname, and uptime        |
| `docker`      | Docker daemon, containers, and images   |
| `development` | Development tool detection and versions |
| `hardware`    | Aggregates hardware information         |
| `diagnostics` | System health and diagnostic checks     |


## Output
The output layer is responsible for formatting collected data and presenting it in the terminal. This keeps system information collection independent from the presentation layer.

## Project Structure
```text
src/
├── cli.rs
├── main.rs
├── collectors/
│   ├── cpu.rs
│   ├── development.rs
│   ├── diagnostics.rs
│   ├── disk.rs
│   ├── docker.rs
│   ├── gpu.rs
│   ├── hardware.rs
│   ├── memory.rs
│   ├── network.rs
│   ├── services.rs
│   └── system.rs
└── output/
    ├── format.rs
    ├── logo.rs
    └── terminal.rs
```

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
- Byte formatting
- Uptime formatting
- CPU collection
- Memory collection
- Disk collection
- GPU detection and parsing
- Network collection
- System information collection
- Docker detection
- Docker daemon states
- Development tool detection
- Missing development tools
- Systemd service parsing
- Failed service detection
- Service running-state detection
- Diagnostic usage thresholds
- Disk diagnostics
- Docker diagnostics
- Service diagnostics
Run the test suite with:
```bash
cargo test
```
## Code Quality
The project is checked using standard Rust development tools:
```bash
cargo fmt --check
cargo check
cargo clippy
cargo test
cargo build --release
```

## Roadmap
Planned features include:

- System monitoring
- More detailed hardware information
- Additional Linux system information
- Improved terminal output
- JSON/YAML output
- Arch Linux package
- AUR package

## Version
Current version: v0.2.0
v0.2.0 expands syspeek with Docker information, development environment detection, enhanced hardware and GPU information, and system diagnostics.
