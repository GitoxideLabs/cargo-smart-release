# cargo-smart-release-core

Reusable API compatibility checks for Rust workspaces. The `api_check` module
provides local crate comparisons, manifest boundary audits, rustdoc dependency
audits, and the JSON release-check protocol used by `cargo smart-release`.

The library has no dependency on `gix`, the release planner, or a CLI framework.
It invokes Cargo and Git when needed. Compatibility comparisons require
Rust 1.99 (rustdoc JSON format 61) and cargo-semver-checks 0.51.0 or newer.

Command-line usage and policy configuration are documented in the repository's
[README](https://github.com/GitoxideLabs/cargo-smart-release#checking-an-api-boundary).
