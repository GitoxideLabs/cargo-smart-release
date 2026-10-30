# `cargo smart-release`

Fearlessly release workspace crates and with beautiful semi-handcrafted changelogs.

[![asciicast](https://asciinema.org/a/65uPfzoWxGac43zEZ1gQ1yVVe.svg)](https://asciinema.org/a/65uPfzoWxGac43zEZ1gQ1yVVe)

## Key Features

* **zero-configuration**
    * `cargo smart-release` needs no extra flags to _do the right thing™️_ smartly. If your intervention is needed it will let you know before it makes changes.
    * It won't do anything if there are no changes.
* **made for multi-crate workspaces**
    * "Nothing stands by itself, and everything is interconnected" - is how it sees the world, allowing it to efficiently handling complex workspace graphs.
    * _works just as well for single-crate workspaces_
* **changelogs-deluxe**
    * It maintains beautiful changelogs for you while allowing you to edit them for the final polish.
    * See your release notes via in-repository _tag objects_ and in _GitHub Releases_
    * **plays well with `cargo release`**
        * `cargo changelog` writes changelogs non-destructively, and only that, leaving the release workflow to [cargo-release].

If seeing is believing, here is [a 12 minute demonstration](https://www.youtube.com/watch?v=EOft_uMDVYE), and the same in 30 minutes is [also available](https://youtu.be/a4CzzxJ7ecE).

## Made for this Workflow

When developing various crates in a workspace, when committing changes and if the edit is breaking, a feature, or another
change I want to see in changelogs, [conventional] git messages will be used. This helps building changelog scaffolding automatically later.

When ready for releasing a particular crate or set of crates of interest, run `cargo smart-release [<crate-name> ...]` to simulate a release.
When stable crates (major version >= 1) cross a major-version boundary, a separate warning report lists their old and
planned versions, whether they will be published or only have their manifests updated, and why: breaking commits,
explicit major-bump requests, failed API checks, or conservative dependency safety bumps. This report appears in dry
runs and before execution, without `--verbose`, including with `--auto-publish-of-stable-crates`. Known published
versions are used as the baseline so an already bumped manifest still appears in the report.
To polish changelogs, run `cargo changelog --write <crate-name>` to update the scaffolding and edit it by hand until it fits.

For particularly thorough but error-prone simulations (as in false positives) one could run `cargo smart-release --dry-run-cargo-publish`.
Crates that don't compile for the host can be released by passing `--target <triple>`, which is forwarded to every `cargo publish` invocation (including dry runs).

After evaluating the release procedure and following instructions,
`cargo smart-release --execute` will cause the fully automatic release of one or more crates.

There are various other options that shouldn't be needed in the common case, use `cargo smart-release --help` to see them.

[conventional]: https://www.conventionalcommits.org

## Checking an API boundary

`cargo api-check` is installed alongside `cargo smart-release`. It audits the
supported API of a workspace crate, including uncommitted changes:

```sh
cargo api-check example
cargo api-check example --baseline HEAD
cargo api-check example --manifest-path path/to/Cargo.toml
```

The default comparison selects the newest reachable semantic-version release tag
whose manifest declares `package.metadata.api-check`: `vVERSION` for a package at
the Git root, or `CRATE-vVERSION` for workspace members. Before the first supported
release, it reports that no baseline exists and still audits the current boundary.
An explicit `--baseline` compares that revision even if it predates the policy.
Missing revisions, unreadable baseline manifests, and tool failures abort the
check. A moved crate may need a prepared baseline checkout through the `release`
subcommand below.

Declare the supported feature configurations explicitly. Crates can also opt into
the same checker during release planning without changing the release command:

```toml
[package.metadata.smart-release]
api-check = ["cargo", "api-check", "release"]
unstable-features = ["unstable"]

[package.metadata.api-check]
profiles = [
    { default-features = false, features = [] },
    { default-features = true, features = [] },
]
# Optional exceptions for deliberately supported types from 0.x dependencies:
allowed-unstable-types = ["plumbing::Handle"]
# Whole-crate exceptions use the Rust crate name:
allowed-unstable-crates = []
# Also audit supported members of reachable workspace dependency types:
recursive = true
```

Only list profiles that the crate supports, and define any named features in its
manifest. Exceptions permit particular dependency types at the boundary; they do
not automatically admit other types exposed by their members. Recursive checks
follow reachable workspace dependency members and exclude members whose docs
contain the experimental markers `**Unstable:**` or `**Unstable.**`.
Dependencies outside the workspace are checked for nominal identity but their
members are not recursively compared. The default audit is not recursive.

To see the checks **with the dependency versions proposed by smart-release**, use:

```sh
cargo api-check example --release-plan
cargo api-check example --release-plan --bump patch --bump-dependencies auto \
    --auto-publish-of-stable-crates --update-crates-index
```

This uses the same planning loop, checker requests, and index-refresh retry as
`cargo smart-release`, then reports compatibility verdicts and candidate versions.
The selected crate must have an enabled `metadata.smart-release.api-check` policy.
It starts with cached crates.io versions unless `--update-crates-index` is supplied;
smart-release additionally refreshes up front for stable auto-publishing or an
actual crates.io publish. Both commands can refresh when retrying a stale plan.
Use matching bump and stable-auto-publish options when comparing the plans.
It stops before changelog generation, manifest edits, commits, tags, and publishing.
The default local comparison uses the dependency versions currently in manifests.

The plumbing commands are also available independently:

```sh
cargo api-check boundary path/to/Cargo.toml
cargo api-check audit path/to/rustdoc.json optional_dependency,another_dependency
cargo api-check release < request.json
cargo api-check release --baseline-root path/to/baseline < request.json
```

All modes exit **0** when their checks pass, **10** for a completed incompatibility
or boundary violation, and **2** when checks cannot complete. A boundary-only
success does not establish compatibility with a prior release.

The built-in checker requires **Rust 1.99 (rustdoc JSON format 61)** and
**cargo-semver-checks 0.51.0 or newer** on `PATH`. It enables rustdoc JSON using
`RUSTC_BOOTSTRAP=1` for the rustdoc subprocess. It also needs Git for
baseline archives. It respects `CARGO_TARGET_DIR`, builds temporary consumers, and
may populate Cargo caches. Existing workspace lockfiles must be up to date:
checks preserve them with `--locked`, reporting a resolution error if they need
updating. Cargo can create a lockfile when none exists yet. Baseline source files
are read from an archive or the explicitly supplied checkout.

The checker lives in the `cargo-smart-release-core` workspace crate and is exposed as
`cargo_smart_release_core::api_check`. It has no dependency on the release planner or
`gix`, so tools such as gitoxide's `it check-api` can reuse it without building
`cargo-smart-release`. The command above uses this same library. Existing hooks
can also use `["cargo", "api-check", "release"]` once the binary is installed.
To run its regression suite, including comparisons requiring cargo-semver-checks,
use `cargo test --workspace -- --include-ignored`.

### Custom checker protocol

`api-check` is an executable and its arguments, without shell interpretation. It
runs from the workspace root, including in dry runs. `unstable-features` is an
optional array of existing feature names; an omitted array is empty. The checker
must validate the supported stable feature configurations, excluding these
features and aliases that enable them. It should combine API tooling such as
`cargo-semver-checks` with an audit of public dependency type identities: a
breaking dependency version can break callers even when the printed types look
unchanged. Source compatibility checks do not establish behavioral compatibility.

The checker receives this JSON object on standard input:

```json
{
  "package": "example",
  "manifest_path": "/workspace/example/Cargo.toml",
  "baseline_version": "1.2.3",
  "baseline_revision": "example-v1.2.3",
  "unstable_features": ["unstable"],
  "dependencies": [
    {"name": "plumbing", "current_version": "0.2.0", "baseline_version": "0.2.0", "next_version": "0.3.0", "breaking": true}
  ]
}
```

The baseline uses the latest published version from the crates index, falling
back to the manifest version when the index has no entry, and the usual release
tag naming. A missing baseline or unsupported configuration must fail the check;
returning success without performing the comparison would waive the guarantee.
`dependencies` contains **all workspace packages**, including transitive ones,
with their proposed versions. Each dependency's `baseline_version` is its latest
published version when known, or its manifest version otherwise; `breaking` compares
against that baseline so manually pre-bumped manifests are handled too.
The source manifests have not yet been rewritten,
so the checker must account for these proposed dependency identities explicitly.

Exit **0** verifies that the stable API is compatible and stops inherited breaking
bumps at this crate. Exit **10** reports incompatibility and requires its normal
breaking version bump. Any other exit status, a missing command, or invalid
metadata aborts before release mutations. Checkers must distinguish tool/build
failures from compatibility findings, and must not change tracked files. Results
are reused only for identical requests during one planning pass; changed candidate
versions trigger another check before anything is published.

Direct breaking conventional commits retain their breaking bumps. Crates without
this metadata retain conservative dependency propagation. Consumers that enable an
excluded feature, including defaults, aliases, and feature forwarding, retain
conservative bumps unless they establish their own checked API boundary. A
compatible boundary still receives a patch release when its dependencies change.
Publishing stable crates still requires `--auto-publish-of-stable-crates` unless
they were selected explicitly.

## Installation

### Cargo

Via `cargo`, which can be obtained using [rustup][rustup]

```
cargo install cargo-smart-release
```

Install with support for emoji's at the start of commit using `cargo`

```
cargo install cargo-smart-release --features allow-emoji
```

## Features

* [x] safe to use as actually performing an operation requires the `--execute` flag
* [x] avoid inconsistent states by making operations as atomic as possible, leveraging `gitoxide` technology to the fullest
* [x] handle workspace dependencies and cycles gracefully, allowing one invocation to publish multiple crates
* [x] avoid making any releases if there are no changes
* [x] avoid bumping versions if the current version isn't released, allowing you to control the version by editing the cargo manifest
* [x] [conventional] commit message drive changelog scaffolding and to automatically derive the crate version to publish
* [x] automatically release dependent workspace IDP crates along with the desired one if they changed since their last release
* [x] automatically adjust manifest versions and update manifests of crates which use those whose versions were incremented, including tracked manifests outside the workspace (such as fuzz tests)
* [x] conservatively bump downstream workspace crates in the light of breaking changes, even though these won't be published, making downstream breakage impossible
* [x] use git tags to know if a crate changed at all, skipping publishes if there is no code change at all
* [ ] it's _too eager_ to release and there should be a way to control patch releases.
* [ ] Handle pre-release versions, like 1.0.0-beta.1
* [ ] Support other remote names than 'origin' - currently the latter name is assumed. Fix by getting the remote of the currently checked out branch.
* [ ] handle version specifications correctly [(tables vs values)](https://github.com/Byron/cargo-release/blob/master/src/cargo.rs#L179:L207)
* [ ] handle all version comparators correctly (see [here](https://github.com/Byron/cargo-release/blob/master/src/version.rs#L192:L226) for how it's done)
* [ ] Automatically detect if crate changes are breaking to suggest the correct version increment
* [x] Support for conventional commits prefixed by an emoji

## Comparison to `cargo release`

`cargo-release` is the reason this tool exists, as it got me addicted to an all automatic release workflow that knows git. This works perfectly
for simple workspaces or single-crate workspaces, as of 2021-08-12, so use it: `cargo install cargo-release`.

Here is what `cargo smart-release` does differently: "It tries really hard to do what _I_ want most of the time when publishing workspace `gitoxide` crates".

- ~~be safe to execute, so it's disarmed by default~~
- specify one or more crates, and detect which crates need publishing automatically
- handle dependency cycles in such a way that increases the chances of overall success
- try really hard to not leave the workspace in an inconsistent state when something goes wrong
- be a playground for `gitoxide` to have a reason to make it much more convenient and more feasible for application authors (aka dog-fooding)
- create changelogs non-destructively, along with annotated tags and GitHub releases

## Limitations

Dependency updates discover tracked `Cargo.toml` files throughout the Git repository and match local dependencies by
package name and resolved path. Both inline and expanded dependency tables are supported, including target-specific
and workspace dependencies. Existing version requirements follow the usual release rules; path-only dependencies
remain versionless. Additional crates are not published or tagged, and their own package versions, changelogs, and
lockfiles are left alone. Unreadable extra manifests or invalid TOML are skipped with a warning; errors
updating a matched dependency stop the release before manifest changes are persisted.

* it requires tables to be used when specifying versions, i.e. `crate = { version = "1" }` instead of `crate = "1".
* it gracefully fails when encountering version requirement comparators which are not `^`, like `=`
* it's tested only by using it on `gitoxide`, there are only very few regression tests with little coverage.
* short object ids in changelogs may be ambiguous, as they are unconditionally truncated to 7 characters.
* changelog rewriting of user content will drop links if they are not of the 'inline' form
* it's very young and probably tries to eat underwear
* it needs a git repository to govern the workspace

### Changelogs

* When determining if something changed in top-level crates, only the `src/` directory is used, unless there is only a single crate in the workspace. This value is hard-coded.
* For change tracking, it will only obtain manifest values once to know where a crate lives, and
  expects it to not be moved.
* If list items populated by commit messages contain items themselves, round-tripping will fail. Ideally there was a way to parse an item on the same level only.

## Acknowledgements

Thanks to [cargo-release] for showing the way and for incredible fast response times. I'd recommend everyone to participate there instead of writing your own.

Special thanks go to [git-cliff] which gave me the nudge needed to want to write my own.

[cargo-release]: https://github.com/sunng87/cargo-release/issues/224

[git-cliff]: https://github.com/orhun/git-cliff

[rustup]: https://rustup.rs/
