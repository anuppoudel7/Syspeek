mod cli;
mod collectors;
mod output;

use clap::Parser;

use cli::{Cli, Commands};
use collectors::{cpu, disk, memory, system};
use output::terminal;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::System) => {
            let system_info = system::collect();

            terminal::print_system_info(&system_info);
        }

        Some(Commands::Hardware) => {
            let cpu_info = cpu::collect();
            let memory_info = memory::collect();
            let disk_info = disk::collect();

            terminal::print_hardware_info(
                &cpu_info,
                &memory_info,
                &disk_info,
            );
        }

        Some(command) => {
            println!("{command:?} command is not implemented yet.");
        }

        None => {
            let system_info = system::collect();
            let cpu_info = cpu::collect();
            let memory_info = memory::collect();
            let disk_info = disk::collect();

            terminal::print_system(
                &system_info,
                &cpu_info,
                &memory_info,
                &disk_info,
            );
        }
    }
}