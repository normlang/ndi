use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(about = "Command-line interface (CLI) for managing Norm code", version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run REPL
    Repl,
}

pub struct Application {}

impl Application {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        let cli = Cli::parse();

        match cli.command {
            Commands::Repl => {
                println!("repl");
            }
        }
    }
}
