use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "syspeek",
    version,
    about = "A fast and powerful Linux system information and diagnostics tool"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    System,
    Hardware,
    Network,
    Services,
    Docker,
    Development,
    Monitor,
    Diagnose,
}