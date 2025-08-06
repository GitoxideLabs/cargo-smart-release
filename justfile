#!/usr/bin/env -S just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.

default:
    {{ just_executable() }} --list

alias t := test
alias c := check

# run all tests, clippy, including journey tests, try building docs
test: clippy check doc unit-tests journey-tests

# run all tests, without clippy, including journey tests, try building docs (and clear target on CI)
ci-test: journey-tests

clear-target:
    cargo clean

# Run cargo clippy on all crates
clippy *clippy-args:
    cargo clippy

# Build all code in suitable configurations
check:
    cargo check --all

# Run cargo doc on all crates
doc $RUSTDOCFLAGS="-D warnings":
    cargo doc --all --no-deps

# run all unit tests
unit-tests:
    cargo test --all
    cargo test --features allow-emoji

cargo-smart-release := `cargo metadata --format-version 1 | jq -r .target_directory` / "debug/cargo-smart-release"

# run journey tests (cargo-smart-release)
journey-tests:
    cargo build --bin cargo-smart-release
    ./tests/journey.sh {{ cargo-smart-release }}

# run various auditing tools to assure we are legal and safe
audit:
    cargo deny check advisories bans licenses sources

# run nightly rustfmt for its extra features, but check that it won't upset stable rustfmt
fmt:
    cargo +nightly fmt --all -- --config-path rustfmt-nightly.toml
    cargo +stable fmt --all -- --check
    just --fmt --unstable
