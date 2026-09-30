mod cli;
mod collectors;
mod output;

use clap::Parser;

use cli::Cli;
use collectors::{cpu, disk, memory, system};
use output::terminal;

fn main() {
    let _cli = Cli::parse();

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