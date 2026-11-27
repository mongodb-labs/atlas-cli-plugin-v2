# atlas-cli-plugin-v2

[![CI](https://github.com/mongodb-labs/atlas-cli-plugin-v2/actions/workflows/ci.yml/badge.svg)](https://github.com/mongodb-labs/atlas-cli-plugin-v2/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/mongodb-labs/atlas-cli-plugin-v2/badge.svg?branch=main)](https://coveralls.io/github/mongodb-labs/atlas-cli-plugin-v2?branch=main)
![Status: preview](https://img.shields.io/badge/status-preview-orange)

The goal is to expose everything the [Atlas Admin API](https://www.mongodb.com/docs/api/doc/atlas-admin-api-v2/) can do under `atlas v2`, with a better command-line UX than the current Atlas CLI commands, and then to take that UX into a v2 of the [Atlas CLI](https://github.com/mongodb/mongodb-atlas-cli).

## Use

The plugin needs the [MongoDB Atlas CLI](https://www.mongodb.com/docs/atlas/cli/current/) installed.

```sh
atlas plugin install mongodb-labs/atlas-cli-plugin-v2
```

## Structure

The v2 command tree will be generated at compile time from the Atlas API spec, pinned to a commit of [mongodb/openapi](https://github.com/mongodb/openapi). None of that exists yet. The work is split over four crates:

```
atlas-cli-plugin-v2
├── crates
│   ├── atlas-cli-v2-plugin    the binary the Atlas CLI runs
│   ├── atlas-cli-v2-derive    the macro that expands the tree into clap commands
│   ├── atlas-cli-v2-codegen   turns the models crate into Rust tokens
│   └── atlas-cli-v2-models    the typed model of the spec and the CLI hierarchy
├── Cargo.toml                 workspace manifest
└── dist-workspace.toml        cargo-dist release config
```

Dependencies point one way, from the binary down to the model:

```
atlas-cli-v2-plugin ──> atlas-cli-v2-derive ──> atlas-cli-v2-codegen ──> atlas-cli-v2-models
                    └──> atlas-cli-v2-models
```

## Develop

Build the workspace:

```sh
cargo build --workspace
```

Run the unit tests:

```sh
cargo test --workspace
```

Run the end-to-end tests. They spawn the built plugin binary directly, so they need no Atlas CLI, no credentials and no network, and the command works on a fresh clone:

```sh
cargo test -p atlas-cli-v2-plugin --test '*' --features e2e-tests
```

## Releasing

See [RELEASING.md](RELEASING.md).

## Template

This repository was created from the [atlas-cli-rust-plugin](https://github.com/jeroenvervaeke/atlas-cli-rust-plugin) template.
