//! End-to-end tests that exercise the plugin binary as the Atlas CLI runs it.
//!
//! Everything here is gated behind the `e2e-tests` feature so that a plain
//! `cargo test` stays hermetic.

#[cfg(feature = "e2e-tests")]
use std::process::Command;

#[cfg(feature = "e2e-tests")]
#[test]
fn help_lists_the_v2_subcommand() {
    let output = Command::new(env!("CARGO_BIN_EXE_atlas-cli-rust-plugin"))
        .arg("--help")
        .output()
        .expect("run the plugin binary");

    assert!(
        output.status.success(),
        "plugin invocation failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let help = String::from_utf8_lossy(&output.stdout);
    assert!(
        help.contains("v2"),
        "help output does not list the v2 subcommand:\n{help}"
    );
}
