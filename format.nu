#!/usr/bin/env nu

# TODO For '.toml', see https://github.com/rust-lang/rustfmt/pull/5240
cargo fmt --all
prettier --write .
