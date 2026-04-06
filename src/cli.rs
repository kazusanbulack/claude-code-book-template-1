use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "task")]
#[command(about = "DevTask - A development task management CLI tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Greet the user
    Greet {
        /// Optional name to greet
        #[arg(long)]
        name: Option<String>,
    },
}