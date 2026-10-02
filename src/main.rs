mod cli;
mod collectors;
mod output;

use clap::Parser;

use cli::{Cli, Commands};
use collectors::{cpu, disk, docker, memory, network, services, system};
use output::terminal;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::System) => {
            let system_info = system::collect();
            terminal::print_system_info(&system_info);
        }

        Some(Commands::Hardware) => {
            let cpu_info = match cpu::collect() {
                Ok(info) => info,
                Err(error) => {
                    eprintln!("Error: failed to collect CPU information: {error:#}");
                    return;
                }
            };

            let memory_info = match memory::collect() {
                Ok(info) => info,
                Err(error) => {
                    eprintln!("Error: failed to collect memory information: {error:#}");
                    return;
                }
            };

            let disk_info = match disk::collect() {
                Ok(info) => info,
                Err(error) => {
                    eprintln!("Error: failed to collect disk information: {error:#}");
                    return;
                }
            };

            terminal::print_hardware_info(&cpu_info, &memory_info, &disk_info);
        }

        Some(Commands::Network) => match network::collect() {
            Ok(interfaces) => {
                terminal::print_network_info(&interfaces);
            }
            Err(error) => {
                eprintln!("Error: failed to collect network information: {error:#}");
            }
        },

        Some(Commands::Services) => match services::collect() {
            Ok(services) => {
                terminal::print_services_info(&services);
            }
            Err(error) => {
                eprintln!("Error: failed to collect system services: {error:#}");
            }
        },

        Some(Commands::Docker) => match docker::collect() {
            Ok(docker_info) => {
                terminal::print_docker_info(&docker_info);
            }
            Err(error) => {
                eprintln!("Error: failed to collect Docker information: {error:#}");
            }
        },

        Some(command) => {
            println!("{command:?} command is coming soon.");
        }

        None => {
            let system_info = system::collect();

            let cpu_info = match cpu::collect() {
                Ok(info) => info,
                Err(error) => {
                    eprintln!("Error: failed to collect CPU information: {error:#}");
                    return;
                }
            };

            let memory_info = match memory::collect() {
                Ok(info) => info,
                Err(error) => {
                    eprintln!("Error: failed to collect memory information: {error:#}");
                    return;
                }
            };

            let disk_info = match disk::collect() {
                Ok(info) => info,
                Err(error) => {
                    eprintln!("Error: failed to collect disk information: {error:#}");
                    return;
                }
            };

            terminal::print_system(&system_info, &cpu_info, &memory_info, &disk_info);
        }
    }
}
