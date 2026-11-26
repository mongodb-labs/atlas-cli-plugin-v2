use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: PluginSubCommands,
}

#[derive(Subcommand)]
pub enum PluginSubCommands {
    /// Plugin root subcommand
    V2 {
        #[command(subcommand)]
        command: PluginCommand,
    },
}

#[derive(Subcommand)]
pub enum PluginCommand {
    /// Placeholder command
    Todo,
}

impl From<Cli> for PluginCommand {
    fn from(cli: Cli) -> Self {
        match cli.command {
            PluginSubCommands::V2 { command } => command,
        }
    }
}
