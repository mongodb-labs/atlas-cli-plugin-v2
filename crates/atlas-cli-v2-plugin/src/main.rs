#![forbid(unsafe_code)]
#![deny(clippy::allow_attributes, clippy::allow_attributes_without_reason)]
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::indexing_slicing,
        clippy::unimplemented,
        clippy::unreachable,
        clippy::unwrap_in_result,
        clippy::panic_in_result_fn,
        clippy::get_unwrap,
        clippy::string_slice,
        clippy::let_underscore_must_use,
        clippy::let_underscore_future,
        clippy::map_err_ignore,
        clippy::dbg_macro,
        clippy::fn_params_excessive_bools,
        clippy::struct_excessive_bools,
        clippy::wildcard_enum_match_arm,
        clippy::as_conversions
    )
)]

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
