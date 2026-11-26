use anyhow::Result;
use args::{Cli, PluginCommand};
use clap::Parser;

mod args;

fn main() -> Result<()> {
    let args: PluginCommand = Cli::parse().into();

    match args {
        PluginCommand::Todo => todo!(),
    }
}
