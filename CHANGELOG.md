# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.21.9 (2025-11-18)

### Features

 - Allow specifying a `--target` triple when running `cargo smart-release`, forwarding it to `cargo publish` so crates that don't
   compile for the host can still be released ([#89](https://github.com/GitoxideLabs/cargo-smart-release/issues/89)).

### Style

 - <csr-id-4af7d573c5cc57fd559ea96bc8c32c66cc4f1ed5/> reorder import statements
   Triggered by justfile fmt
   - move std::io::Write to the top for standard library import consistency
 - <csr-id-9f61801b0f4bc7c62c2bd3efa1f58f51e7f1d0a6/> reorder import statements for consistency
   Triggered by justfile fmt
   - organize import statements for better readability
   - move external crates to the top and internal modules below
 - <csr-id-f1eb1e049bb7f7cdb82b194b5527210e40b5bb73/> reorder imports for better readability
   Triggered by justfile fmt
   - move `gix::traverse::commit::simple::CommitTimeOrder` import for logical grouping
   - adjust crate imports to follow standard ordering conventions

### Other

 - <csr-id-a834811e77ce1c0d7663e84cbcc3c46f13a2d2c1/> add unicode-properties library
   - add unicode-properties version 0.1.3 with emoji feature
   - enhance string handling capabilities with new library

### Bug Fixes

 - <csr-id-85cf4c751ec1b6548ec5c14d69c38af254230bd9/> enable version flag

### New Features

 - <csr-id-ecde413b6a919926928b3f8dd8da010c88c01308/> add --target passthrough to cargo
 - <csr-id-816908351b3d25c1ab1dce796fb635ed39ac67a6/> Add commit_prefix option
 - <csr-id-514f10b00c3d67bae9f08c1c09eb105a256888cd/> add emoji support feature
   - introduce `allow-emoji` feature for handling emojis
   - update Cargo.toml and justfile for conditional emoji feature testing
   
   refactor(commit): enhance message parsing with emoji support
   
   - add conditional compilation for emoji parsing in commit messages
   - implement feature flag checks to toggle emoji handling in messages
 - <csr-id-282445f6806b361cfa177b843faecc7641ad41b1/> add emoji handling for commit parsing
   - implement unicode properties to filter emojis from commit messages
   - refactor message parsing to support emoji-free strings
   - add tests for conventional commits with scope and emoji handling

### Documentation

 - <csr-id-e64ac33304d7e0060ae468ef81dda7ddc516b37e/> add emoji support installation instructions
   - update README with instructions for installing cargo-smart-release with emoji support
   - add feature list item for conventional commits prefixed by an emoji

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 34 commits contributed to the release.
 - 10 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#76](https://github.com/Byron/cargo-smart-release/issues/76)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#76](https://github.com/Byron/cargo-smart-release/issues/76)**
    - Improve `commit-prefix` argument message with anticipated usage note. ([`9d0a085`](https://github.com/Byron/cargo-smart-release/commit/9d0a085c88935d1c292a1fe335ff434c779cf10a))
 * **Uncategorized**
    - Merge branch 'cargo-target' ([`fe3f477`](https://github.com/Byron/cargo-smart-release/commit/fe3f47795cd51dbc83145ff17b9e116fcfba2e0e))
    - Refactor ([`f07ebe1`](https://github.com/Byron/cargo-smart-release/commit/f07ebe1ab75a25ecf10c7f8753dd2dca333dd53b))
    - Add --target passthrough to cargo ([`ecde413`](https://github.com/Byron/cargo-smart-release/commit/ecde413b6a919926928b3f8dd8da010c88c01308))
    - Merge branch 'HigherOrderLogic-patch-1' ([`93565bf`](https://github.com/Byron/cargo-smart-release/commit/93565bffa500fe1513f9664f2c9bf0238219c79b))
    - Refactor ([`6d82e2e`](https://github.com/Byron/cargo-smart-release/commit/6d82e2e7324ee1c29bb2bbca998dfbdca2ce228c))
    - Enable version flag ([`85cf4c7`](https://github.com/Byron/cargo-smart-release/commit/85cf4c751ec1b6548ec5c14d69c38af254230bd9))
    - Merge pull request #87 from GitoxideLabs/dependabot/cargo/cargo-04ef004206 ([`00ca99f`](https://github.com/Byron/cargo-smart-release/commit/00ca99f7ca97d1ed907835204ef5b50c1e14b3c4))
    - Bump the cargo group across 1 directory with 62 updates ([`b54064f`](https://github.com/Byron/cargo-smart-release/commit/b54064fc22957ce7d2696fab9679a97e0fbe1a25))
    - Merge pull request #86 from EliahKagan/ungroup-pulldown-cmark ([`d0de1a8`](https://github.com/Byron/cargo-smart-release/commit/d0de1a8740a6820e60c3478a7df44c0bd63c32cf))
    - Don't include `pulldown-cmark` in grouped version updates ([`f5b0637`](https://github.com/Byron/cargo-smart-release/commit/f5b06378f0f2550eb8895506ee969141a869a514))
    - Merge pull request #84 from GitoxideLabs/dependabot/github_actions/github-actions-1893dd32ff ([`0baa405`](https://github.com/Byron/cargo-smart-release/commit/0baa40522c8d8b8b6943b487acd4fdbed9bfb60c))
    - Bump github/codeql-action from 3 to 4 in the github-actions group ([`13d50b4`](https://github.com/Byron/cargo-smart-release/commit/13d50b4bffc2255b5c7d8e3cc235889341052aa0))
    - Merge pull request #83 from GitoxideLabs/dependabot/cargo/cargo-470f7772c1 ([`8001b48`](https://github.com/Byron/cargo-smart-release/commit/8001b4808881971cc34a81e839653d54b561e7ee))
    - Bump the cargo group across 1 directory with 41 updates ([`0276691`](https://github.com/Byron/cargo-smart-release/commit/02766913af2ff4d89dcdc80d6900082b32389857))
    - Merge pull request #82 from GitoxideLabs/dependabot/cargo/serde-untagged-0.1.9 ([`bf9960e`](https://github.com/Byron/cargo-smart-release/commit/bf9960e08daa808d99b35ae9bb19cea7e81ee7a7))
    - Merge pull request #81 from GitoxideLabs/dependabot/cargo/erased-serde-0.4.8 ([`7f13245`](https://github.com/Byron/cargo-smart-release/commit/7f13245cfdcdb2aa5ae736bb1c5c6ce92628f0f0))
    - Bump serde-untagged from 0.1.8 to 0.1.9 ([`8573eb1`](https://github.com/Byron/cargo-smart-release/commit/8573eb10b9cfeec8a3538fd8a8bf445f01f1d594))
    - Bump erased-serde from 0.4.6 to 0.4.8 ([`a300ef5`](https://github.com/Byron/cargo-smart-release/commit/a300ef5826e28a836e49f0158a384edc0d1bb5ec))
    - Merge pull request #79 from GitoxideLabs/dependabot/github_actions/github-actions-a331d3ec2d ([`2b13036`](https://github.com/Byron/cargo-smart-release/commit/2b1303669bcfff93273fe388ffdb18630b704306))
    - Bump actions/checkout from 4 to 5 in the github-actions group ([`dab64a6`](https://github.com/Byron/cargo-smart-release/commit/dab64a6d6afded7fd5f78b280909c7749430dd59))
    - Merge pull request #78 from GitoxideLabs/dependabot/cargo/cargo-3aa5a0942d ([`eaf5638`](https://github.com/Byron/cargo-smart-release/commit/eaf5638ea2d6b004d44f8c55034025d43d57c385))
    - Bump the cargo group with 44 updates ([`170b1e9`](https://github.com/Byron/cargo-smart-release/commit/170b1e9704385cd5c04b52b232fdc0aac76b1f28))
    - Merge branch 'simoncozens/main' ([`3bb8880`](https://github.com/Byron/cargo-smart-release/commit/3bb8880ff70c231aae11d4dbc9a8f5b7ec376b0e))
    - Add commit_prefix option ([`8169083`](https://github.com/Byron/cargo-smart-release/commit/816908351b3d25c1ab1dce796fb635ed39ac67a6))
    - Refactor ([`560e924`](https://github.com/Byron/cargo-smart-release/commit/560e924b1130bf17ee0000d7dc3a8b059cdd5980))
    - Add emoji support installation instructions ([`e64ac33`](https://github.com/Byron/cargo-smart-release/commit/e64ac33304d7e0060ae468ef81dda7ddc516b37e))
    - Add emoji support feature ([`514f10b`](https://github.com/Byron/cargo-smart-release/commit/514f10b00c3d67bae9f08c1c09eb105a256888cd))
    - Reorder import statements ([`4af7d57`](https://github.com/Byron/cargo-smart-release/commit/4af7d573c5cc57fd559ea96bc8c32c66cc4f1ed5))
    - Reorder import statements for consistency ([`9f61801`](https://github.com/Byron/cargo-smart-release/commit/9f61801b0f4bc7c62c2bd3efa1f58f51e7f1d0a6))
    - Reorder imports for better readability ([`f1eb1e0`](https://github.com/Byron/cargo-smart-release/commit/f1eb1e049bb7f7cdb82b194b5527210e40b5bb73))
    - Add unicode-properties library ([`a834811`](https://github.com/Byron/cargo-smart-release/commit/a834811e77ce1c0d7663e84cbcc3c46f13a2d2c1))
    - Add emoji handling for commit parsing ([`282445f`](https://github.com/Byron/cargo-smart-release/commit/282445f6806b361cfa177b843faecc7641ad41b1))
    - Merge pull request #72 from GitoxideLabs/fix ([`82d1d24`](https://github.com/Byron/cargo-smart-release/commit/82d1d24d0e76a5e48948b69758864c51839d7424))
</details>

## 0.21.8 (2025-08-03)

### Bug Fixes

 - <csr-id-71f3ff862bf403ce30850461edfe447a9bf5ed23/> Allow remote-URLs to be used when pushing.
   That way this works when branches are checked out with URLs in stead
   of symbolic remote names.
   This happens by default when using `gh pr checkout`.
   
   Previously, it would panic, which now also won't happen anymore
   in favor of a more descriptive error.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release.
 - 98 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-smart-release v0.21.8 ([`1ffea62`](https://github.com/Byron/cargo-smart-release/commit/1ffea62845507b80662c94e1247efbb03efc4845))
    - Allow remote-URLs to be used when pushing. ([`71f3ff8`](https://github.com/Byron/cargo-smart-release/commit/71f3ff862bf403ce30850461edfe447a9bf5ed23))
    - Merge pull request #71 from GitoxideLabs/dependabot/cargo/cargo-431d3b394f ([`2919096`](https://github.com/Byron/cargo-smart-release/commit/2919096b1c65a75fb83440d69ede05f6e6ec4989))
    - Bump the cargo group with 11 updates ([`ade7955`](https://github.com/Byron/cargo-smart-release/commit/ade79553bede6005c610d52223ff3f185b5c56d3))
    - Merge pull request #70 from GitoxideLabs/dependabot/cargo/cargo-81d39f0e3a ([`fa63be7`](https://github.com/Byron/cargo-smart-release/commit/fa63be786391b497aba82c44f11589e80d2ab9b1))
    - Bump the cargo group with 32 updates ([`7502019`](https://github.com/Byron/cargo-smart-release/commit/75020193cb282cff0b875e33122ea35f6010c0a1))
    - Merge pull request #69 from EliahKagan/thanks-clippy ([`3af51d7`](https://github.com/Byron/cargo-smart-release/commit/3af51d78ed511895436261d78cc1ea4600dc758a))
    - Replace another use of `.then()` with `if`-`then` ([`92c4374`](https://github.com/Byron/cargo-smart-release/commit/92c43745ac0cd6f28d8a276c28db0842b8b2826b))
    - Thanks clippy ([`344f250`](https://github.com/Byron/cargo-smart-release/commit/344f25047162a8a0eb6fc0065aa8ba7040e3bbac))
    - Remove unrelated dependencies by avoiding `pulldown-cmark` default dependencies. ([`bee3e4c`](https://github.com/Byron/cargo-smart-release/commit/bee3e4c9438de9c5e795b5a16078a307beeb961d))
    - Merge pull request #68 from GitoxideLabs/dependabot/cargo/cargo-b47431ca3c ([`6533376`](https://github.com/Byron/cargo-smart-release/commit/653337672f1cf85725c94eae64be168aa7c3ed09))
    - Bump the cargo group with 47 updates ([`02b3d6d`](https://github.com/Byron/cargo-smart-release/commit/02b3d6dff8c0d00d4ed39e31b68408425035146d))
    - Merge pull request #65 from GitoxideLabs/dependabot/cargo/cargo-1c3b7ce06e ([`ffbffae`](https://github.com/Byron/cargo-smart-release/commit/ffbffaeef3c82ea9995a412f07ccccf43823c19b))
    - Thanks clippy ([`f8d2f30`](https://github.com/Byron/cargo-smart-release/commit/f8d2f3030acd532174360e8a04c55d0af45fdf9a))
    - Adapt to changes in `cargo_metadata` ([`aac2e4b`](https://github.com/Byron/cargo-smart-release/commit/aac2e4b6fded326da062bb6c909b99c73a334cce))
    - Bump the cargo group with 38 updates ([`ff64e06`](https://github.com/Byron/cargo-smart-release/commit/ff64e066648453f92b760b9bde6bcfde4fd1e2dc))
    - Merge pull request #64 from GitoxideLabs/dependabot/cargo/cargo-bf709c9f14 ([`50a3266`](https://github.com/Byron/cargo-smart-release/commit/50a3266fc1885755a2389c9e8f28c65637dca0ce))
    - Bump the cargo group with 8 updates ([`7734556`](https://github.com/Byron/cargo-smart-release/commit/77345560e9cca9bfc6cb9678b52c109d776a866e))
</details>

## 0.21.7 (2025-04-27)

<csr-id-919acc2000c694a3f2399fa6992a56803653d003/>
<csr-id-202be0927a17cf20c458fab9cc6eb957d3a81584/>
<csr-id-9e3257c19dcd12ed556fd173c7211b729a69d1ff/>

### Chore

 - <csr-id-919acc2000c694a3f2399fa6992a56803653d003/> bump the cargo group across 1 directory with 5 updates
   Bumps the cargo group with 5 updates in the / directory:
   
   | Package | From | To |
   | --- | --- | --- |
   | [clap](https://github.com/clap-rs/clap) | `4.5.36` | `4.5.37` |
   | [cargo_toml](https://gitlab.com/lib.rs/cargo_toml) | `0.21.0` | `0.22.1` |
   | [winnow](https://github.com/winnow-rs/winnow) | `0.6.26` | `0.7.6` |
   | [jiff](https://github.com/BurntSushi/jiff) | `0.1.29` | `0.2.8` |
   | [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) | `0.9.6` | `0.13.0` |
   
   
   
   Updates `clap` from 4.5.36 to 4.5.37
   - [Release notes](https://github.com/clap-rs/clap/releases)
   - [Changelog](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md)
   - [Commits](https://github.com/clap-rs/clap/compare/clap_complete-v4.5.36...clap_complete-v4.5.37)
   
   Updates `cargo_toml` from 0.21.0 to 0.22.1
   - [Commits](https://gitlab.com/lib.rs/cargo_toml/commits/v0.22.1)
   
   Updates `winnow` from 0.6.26 to 0.7.6
   - [Changelog](https://github.com/winnow-rs/winnow/blob/main/CHANGELOG.md)
   - [Commits](https://github.com/winnow-rs/winnow/compare/v0.6.26...v0.7.6)
   
   Updates `jiff` from 0.1.29 to 0.2.8
   - [Release notes](https://github.com/BurntSushi/jiff/releases)
   - [Changelog](https://github.com/BurntSushi/jiff/blob/master/CHANGELOG.md)
   - [Commits](https://github.com/BurntSushi/jiff/compare/0.1.29...jiff-static-0.2.8)
   
   Updates `pulldown-cmark` from 0.9.6 to 0.13.0
   - [Release notes](https://github.com/raphlinus/pulldown-cmark/releases)
   - [Commits](https://github.com/raphlinus/pulldown-cmark/compare/v0.9.6...v0.13.0)
   
   ---
   updated-dependencies:
   - dependency-name: clap
     dependency-version: 4.5.37
     dependency-type: direct:production
     update-type: version-update:semver-patch
     dependency-group: cargo
   - dependency-name: cargo_toml
     dependency-version: 0.22.1
     dependency-type: direct:production
     update-type: version-update:semver-minor
     dependency-group: cargo
   - dependency-name: winnow
     dependency-version: 0.7.6
     dependency-type: direct:production
     update-type: version-update:semver-minor
     dependency-group: cargo
   - dependency-name: jiff
     dependency-version: 0.2.8
     dependency-type: direct:production
     update-type: version-update:semver-minor
     dependency-group: cargo
   - dependency-name: pulldown-cmark
     dependency-version: 0.13.0
     dependency-type: direct:production
     update-type: version-update:semver-minor
     dependency-group: cargo
   ...
 - <csr-id-202be0927a17cf20c458fab9cc6eb957d3a81584/> bump crossbeam-channel from 0.5.14 to 0.5.15
   Bumps [crossbeam-channel](https://github.com/crossbeam-rs/crossbeam) from 0.5.14 to 0.5.15.
   - [Release notes](https://github.com/crossbeam-rs/crossbeam/releases)
   - [Changelog](https://github.com/crossbeam-rs/crossbeam/blob/master/CHANGELOG.md)
   - [Commits](https://github.com/crossbeam-rs/crossbeam/compare/crossbeam-channel-0.5.14...crossbeam-channel-0.5.15)
   
   ---
   updated-dependencies:
   - dependency-name: crossbeam-channel
     dependency-version: 0.5.15
     dependency-type: indirect
   ...
 - <csr-id-9e3257c19dcd12ed556fd173c7211b729a69d1ff/> bump the github-actions group with 4 updates
   Bumps the github-actions group with 4 updates: [actions/checkout](https://github.com/actions/checkout), [EmbarkStudios/cargo-deny-action](https://github.com/embarkstudios/cargo-deny-action), [extractions/setup-just](https://github.com/extractions/setup-just) and [github/codeql-action](https://github.com/github/codeql-action).
   
   
   Updates `actions/checkout` from 3 to 4
   - [Release notes](https://github.com/actions/checkout/releases)
   - [Changelog](https://github.com/actions/checkout/blob/main/CHANGELOG.md)
   - [Commits](https://github.com/actions/checkout/compare/v3...v4)
   
   Updates `EmbarkStudios/cargo-deny-action` from 1 to 2
   - [Release notes](https://github.com/embarkstudios/cargo-deny-action/releases)
   - [Commits](https://github.com/embarkstudios/cargo-deny-action/compare/v1...v2)
   
   Updates `extractions/setup-just` from 1 to 3
   - [Release notes](https://github.com/extractions/setup-just/releases)
   - [Commits](https://github.com/extractions/setup-just/compare/v1...v3)
   
   Updates `github/codeql-action` from 2 to 3
   - [Release notes](https://github.com/github/codeql-action/releases)
   - [Changelog](https://github.com/github/codeql-action/blob/main/CHANGELOG.md)
   - [Commits](https://github.com/github/codeql-action/compare/v2...v3)
   
   ---
   updated-dependencies:
   - dependency-name: actions/checkout
     dependency-version: '4'
     dependency-type: direct:production
     update-type: version-update:semver-major
     dependency-group: github-actions
   - dependency-name: EmbarkStudios/cargo-deny-action
     dependency-version: '2'
     dependency-type: direct:production
     update-type: version-update:semver-major
     dependency-group: github-actions
   - dependency-name: extractions/setup-just
     dependency-version: '3'
     dependency-type: direct:production
     update-type: version-update:semver-major
     dependency-group: github-actions
   - dependency-name: github/codeql-action
     dependency-version: '3'
     dependency-type: direct:production
     update-type: version-update:semver-major
     dependency-group: github-actions
   ...

### Documentation

 - <csr-id-3267581a7f69bbc2c630552d3daf08fdaa4c7992/> Update project URL and add security policy
   - Update the URL in `Cargo.toml` to point to
   `GitoxideLabs/cargo-smart-release` since the repo was moved
   (though the old URL will still work as a redirect).

### New Features

 - <csr-id-2efb737d16dec82d1e8a00d05fac06142a4b2718/> add `--allow-empty-release-message` actually ignore empty changelogs
 - <csr-id-ca7d486d48ee37ba70cbcf583b3479a97f1899d5/> Recognize variants of "thanks clippy"
   The commit message title (or title prefix) "thanks clippy" was
   recognized, but case variants such as "Thanks clippy" were not.
   Less importantly, the plausible "thanks, clippy" variant (and its
   case variants) were also not recognized.
   
   This downcases the title before checking for "thanks clippy" as a
   prefix, and it adds a check for the prefix "thanks, clippy".

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 49 commits contributed to the release.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#40](https://github.com/Byron/cargo-smart-release/issues/40)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#40](https://github.com/Byron/cargo-smart-release/issues/40)**
    - Update Contributing guide ([`52f8824`](https://github.com/Byron/cargo-smart-release/commit/52f8824f9ffdff316b36a9270d89ab3e45994478))
 * **Uncategorized**
    - Release cargo-smart-release v0.21.7 ([`da00838`](https://github.com/Byron/cargo-smart-release/commit/da00838c5a789c79ed47b99b202f88af33b21ab0))
    - Merge pull request #63 from GitoxideLabs/dependabot/cargo/cargo-cfb4711de7 ([`008bfec`](https://github.com/Byron/cargo-smart-release/commit/008bfec919b0d5e55a7a71d024f733815febef2e))
    - Upgrade `crates-index` and `gix-testtools` ([`350b950`](https://github.com/Byron/cargo-smart-release/commit/350b9508d2a22c62343806f98fad3bd028b42d03))
    - Refactor ([`385159e`](https://github.com/Byron/cargo-smart-release/commit/385159e8cad0f79ba84553007b9c139480b82bcd))
    - Adapt to changes in `gix` ([`698bb7a`](https://github.com/Byron/cargo-smart-release/commit/698bb7a96bdbc100268e79992c485729777f26e9))
    - Bump the cargo group across 1 directory with 11 updates ([`4da21c0`](https://github.com/Byron/cargo-smart-release/commit/4da21c0bea2612341d3aed3e47b2426a9aa1a031))
    - Merge pull request #62 from EliahKagan/deps ([`5386251`](https://github.com/Byron/cargo-smart-release/commit/5386251d277e20772c330e21a94aeb8860b1738c))
    - Get only patch version updates to `pulldown-cmark` ([`aeb91ee`](https://github.com/Byron/cargo-smart-release/commit/aeb91ee22bcd8aadd64a3261e3de2ed985c99485))
    - Omit conventional prefix in Dependabot commits ([`d71b7b7`](https://github.com/Byron/cargo-smart-release/commit/d71b7b7e30cd6659c5a762f0d9d4bbfeb6de1d0b))
    - Merge pull request #61 from GitoxideLabs/allow-generated ([`c583e0a`](https://github.com/Byron/cargo-smart-release/commit/c583e0a3b3e034a3f891056d57a108caf2c970a0))
    - Add `--allow-empty-release-message` actually ignore empty changelogs ([`2efb737`](https://github.com/Byron/cargo-smart-release/commit/2efb737d16dec82d1e8a00d05fac06142a4b2718))
    - Merge pull request #58 from EliahKagan/dependency-type ([`bd30507`](https://github.com/Byron/cargo-smart-release/commit/bd30507aa857ec179c9b6b874d67abe548194a42))
    - Always let Dependabot propose `Cargo.lock` updates ([`93c47c2`](https://github.com/Byron/cargo-smart-release/commit/93c47c209428621873ec25e99e98c07055341bd7))
    - Merge pull request #54 from GitoxideLabs/dependabot/cargo/cargo-fb4565de19 ([`76ec21b`](https://github.com/Byron/cargo-smart-release/commit/76ec21bced0da7200e5b2306514740262f0d9edc))
    - Downgrade `pulldown-cmark` to 0.9.6; bump some others ([`694c34f`](https://github.com/Byron/cargo-smart-release/commit/694c34f52430cbdec0a0f921e27330bfe76fbdcb))
    - Bump the cargo group across 1 directory with 5 updates ([`919acc2`](https://github.com/Byron/cargo-smart-release/commit/919acc2000c694a3f2399fa6992a56803653d003))
    - Merge pull request #57 from EliahKagan/installation ([`acc104a`](https://github.com/Byron/cargo-smart-release/commit/acc104a75dac8478ffafec4378a558289cf034f7))
    - Improve event triggers for crates.io install test ([`33bd24d`](https://github.com/Byron/cargo-smart-release/commit/33bd24d94d2cabd4151095e905f7ed3a90d66f68))
    - Merge pull request #55 from EliahKagan/doc ([`d59c889`](https://github.com/Byron/cargo-smart-release/commit/d59c889b5cd931ac22c27e02ef2af063834f6b11))
    - Update project URL and add security policy ([`3267581`](https://github.com/Byron/cargo-smart-release/commit/3267581a7f69bbc2c630552d3daf08fdaa4c7992))
    - Merge pull request #52 from EliahKagan/version-updates ([`49e50f7`](https://github.com/Byron/cargo-smart-release/commit/49e50f7d6a133d275792f783789b9d8b4c1a092a))
    - Merge pull request #51 from EliahKagan/run-ci/cargo-update ([`dfa7a85`](https://github.com/Byron/cargo-smart-release/commit/dfa7a85e016f9de526961cfd20dca39f8e55bcd4))
    - Thanks clippy ([`5386a41`](https://github.com/Byron/cargo-smart-release/commit/5386a41f048a744e7cbabb131256d08b15bc4d2e))
    - Don't `--allow deprecated` for `clippy` on CI ([`1fa2b68`](https://github.com/Byron/cargo-smart-release/commit/1fa2b68b32455401b4540fa07b0bfae88f8ea817))
    - Thanks clippy ([`90afecb`](https://github.com/Byron/cargo-smart-release/commit/90afecbb4d5fe58cfb559cc2c47711db6b32f1e0))
    - Bump locked dependencies ([`fe4de40`](https://github.com/Byron/cargo-smart-release/commit/fe4de4067ad2559284b54f0349a538f4732a9a1d))
    - Enable Dependabot version updates for Rust ([`24dc8df`](https://github.com/Byron/cargo-smart-release/commit/24dc8df5f6abcb2da382b2a6399dcb57094c0d74))
    - Merge pull request #49 from GitoxideLabs/dependabot/cargo/crossbeam-channel-0.5.15 ([`502682d`](https://github.com/Byron/cargo-smart-release/commit/502682d5b61e9dc5b98872d9273094a38486acff))
    - Bump crossbeam-channel from 0.5.14 to 0.5.15 ([`202be09`](https://github.com/Byron/cargo-smart-release/commit/202be0927a17cf20c458fab9cc6eb957d3a81584))
    - Merge pull request #48 from EliahKagan/run-ci/security ([`c254144`](https://github.com/Byron/cargo-smart-release/commit/c2541442e3597330aeb38bddd2689912de37abe8))
    - Merge pull request #47 from EliahKagan/contrib ([`262f202`](https://github.com/Byron/cargo-smart-release/commit/262f202fed524a8554b345295b665ac24a528a53))
    - Fix GHA warning syntax ([`ae23c95`](https://github.com/Byron/cargo-smart-release/commit/ae23c954066c880862cc9f2111dd41bc3300d8c8))
    - Find all applicable RUSTSEC advisories in `security_audit` job ([`3f243c1`](https://github.com/Byron/cargo-smart-release/commit/3f243c1a74f582de816c0447b3e96fe0f9fc19ed))
    - Preinstall `cargo-audit` binary in `security_audit` job ([`f34e7b6`](https://github.com/Byron/cargo-smart-release/commit/f34e7b66d64164494dff644ce7c369a01a4b0ff2))
    - Add some convenient and fairly unobtrustive CI triggers ([`f9880fa`](https://github.com/Byron/cargo-smart-release/commit/f9880fa55f46363e1b82d4c052dd9cf13bf07be6))
    - Improve title in the contributing guidelines ([`e2b3f17`](https://github.com/Byron/cargo-smart-release/commit/e2b3f175b562c1c2f1bc71b4a5ba7badb9c2fb29))
    - Merge pull request #45 from Byron/dependabot/github_actions/github-actions-d3e65476f6 ([`1ebc174`](https://github.com/Byron/cargo-smart-release/commit/1ebc174eb62d656fc3359744bdbfeb689f46fa2d))
    - Merge pull request #44 from EliahKagan/bump-gix ([`28def59`](https://github.com/Byron/cargo-smart-release/commit/28def59a9e58edac81d2e9d5df2c0cad94db03da))
    - Bump the github-actions group with 4 updates ([`9e3257c`](https://github.com/Byron/cargo-smart-release/commit/9e3257c19dcd12ed556fd173c7211b729a69d1ff))
    - Merge pull request #43 from EliahKagan/run-ci/gha ([`9c9a8dc`](https://github.com/Byron/cargo-smart-release/commit/9c9a8dc002153f9dc91fc36ee0fc6a86cebd7e95))
    - Merge pull request #42 from EliahKagan/thanks-clippy-detection ([`0c307d3`](https://github.com/Byron/cargo-smart-release/commit/0c307d38581450efa63145c2d3e1293e14f7e746))
    - Bump `gix` from 0.69.1 to 0.71.0 ([`0f1bc82`](https://github.com/Byron/cargo-smart-release/commit/0f1bc82b130684d1e4348c7015c456013d5b6e36))
    - Enable Dependabot version updates for actions ([`ed59e97`](https://github.com/Byron/cargo-smart-release/commit/ed59e97eab4202f010bc2d7fac6aad12b7fe866f))
    - Make the main CI workflow easier to run in forks ([`74e3475`](https://github.com/Byron/cargo-smart-release/commit/74e3475249f035d8f17f6e3ab9cf0043c068c3fb))
    - Use more consistent style in CI workflows ([`bfbaa1e`](https://github.com/Byron/cargo-smart-release/commit/bfbaa1ef58f409c5554c585038e6ac5323dcbd9d))
    - Recognize variants of "thanks clippy" ([`ca7d486`](https://github.com/Byron/cargo-smart-release/commit/ca7d486d48ee37ba70cbcf583b3479a97f1899d5))
    - Merge pull request #41 from EliahKagan/clippy-fixes ([`67b2147`](https://github.com/Byron/cargo-smart-release/commit/67b2147ac69db4c03bfe41540e1ff0d165481724))
    - Thanks clippy ([`c286b5a`](https://github.com/Byron/cargo-smart-release/commit/c286b5a077548cb428dba91eb389fdea0283c7e9))
</details>

<csr-unknown>
Add a SECURITY.md file with the same wording as in gitoxide,but where the hyperlink is for drafting a cargo-smart-releaseadvisory (rather than a gitoxide advisory).<csr-unknown/>

## 0.21.6 (2024-12-24)

### New Features

 - <csr-id-32283ca6b868e18736ecfad0354219e6142d0932/> Add option to signoff git commits
   Adds `cargo smart-release --signoff` to sign off git
   commits.

### Bug Fixes

 - <csr-id-869b0bd5202465a891bae8830e817e07a6133df6/> don't crash when run on a detached `HEAD`
   Instead, there will be a helpful error message.
 - <csr-id-5a4a5227635168e1ace0ed4b8b163c6b29423c7b/> assure we only edit dependencies of local path dependencies.
   Otherwise it would have been possible to set a crate-io dependency
   to a version that wasn't yet published.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#34](https://github.com/Byron/cargo-smart-release/issues/34)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#34](https://github.com/Byron/cargo-smart-release/issues/34)**
    - Don't crash when run on a detached `HEAD` ([`869b0bd`](https://github.com/Byron/cargo-smart-release/commit/869b0bd5202465a891bae8830e817e07a6133df6))
 * **Uncategorized**
    - Release cargo-smart-release v0.21.6 ([`7db489e`](https://github.com/Byron/cargo-smart-release/commit/7db489e9d3df4f111d59f695b66c241955666509))
    - Upgrade all dependencies ([`8f3f794`](https://github.com/Byron/cargo-smart-release/commit/8f3f794658fe9b63b399e437894ca923896d0084))
    - Update dependencies ([`b5c8259`](https://github.com/Byron/cargo-smart-release/commit/b5c825950ba2fa1497f92f5bb8481b0e03ab2c1e))
    - Adjust test expectations, which are dependent on the Rust version now ([`f8c753d`](https://github.com/Byron/cargo-smart-release/commit/f8c753d8da3254427bb05bc3242fdca9b88cd777))
    - Thanks clippy ([`4365aa5`](https://github.com/Byron/cargo-smart-release/commit/4365aa5928d25b5498e95ec89e994e58cd994a4b))
    - Assure we only edit dependencies of local path dependencies. ([`5a4a522`](https://github.com/Byron/cargo-smart-release/commit/5a4a5227635168e1ace0ed4b8b163c6b29423c7b))
    - Ignore new tests that are now failing on CI. ([`e9e5fbf`](https://github.com/Byron/cargo-smart-release/commit/e9e5fbfb86c0e8f02eebf9f80ff7aff54f3a4b31))
    - Merge pull request #32 from dave-tucker/signoff ([`c498d29`](https://github.com/Byron/cargo-smart-release/commit/c498d29169457e57fc74b8234da2bbbcdecbdd10))
    - Add option to signoff git commits ([`32283ca`](https://github.com/Byron/cargo-smart-release/commit/32283ca6b868e18736ecfad0354219e6142d0932))
    - Merge branch 'ag/fix-day-span' ([`557f555`](https://github.com/Byron/cargo-smart-release/commit/557f55534c63da5d84f6e7557f4656d44e6bebdb))
    - Fix use of span to get number of days ([`a108b79`](https://github.com/Byron/cargo-smart-release/commit/a108b7977d44fd697ad8668f56179287c26d452d))
</details>

## 0.21.5 (2024-08-31)

### Bug Fixes

 - <csr-id-6c587768acc26e98ac845bdd8d6283a0331a48fc/> Compute amount of passed days between releases properly
   Previously the amount of days passed would always be zero, which
   would make this information disappear everywhere.
   
   Now it shows like before.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 8 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-smart-release v0.21.5 ([`44f2371`](https://github.com/Byron/cargo-smart-release/commit/44f2371d29603739572608aecd23f4c1654cf986))
    - Merge branch 'fix-27' ([`80244ed`](https://github.com/Byron/cargo-smart-release/commit/80244ed94c58e588253b2b67e995c42543bda2c0))
    - Compute amount of passed days between releases properly ([`6c58776`](https://github.com/Byron/cargo-smart-release/commit/6c587768acc26e98ac845bdd8d6283a0331a48fc))
</details>

## 0.21.4 (2024-08-23)

<csr-id-056d3065c49de523a1e06b845c270c1d175814d6/>

### Chore

 - <csr-id-056d3065c49de523a1e06b845c270c1d175814d6/> upgrade `gix` to 0.66
   This removes `time` from the dependency list and maybe makes installations work again.

### New Features

 - <csr-id-91120a312eb047c315f7d47f0fdeaf32e63d4a52/> allow changelog generation for moved crates, emit warning message
 - <csr-id-8c45160f3bdf928ebe9549dad9f8d51f25b876f3/> improve error messages when path is not found in commit

### Bug Fixes

 - <csr-id-57f6e2b9d1239af965d0c4ad8a34bd5b52983dcd/> don't crash on detached heads

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#14](https://github.com/Byron/cargo-smart-release/issues/14)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#14](https://github.com/Byron/cargo-smart-release/issues/14)**
    - Don't crash on detached heads ([`57f6e2b`](https://github.com/Byron/cargo-smart-release/commit/57f6e2b9d1239af965d0c4ad8a34bd5b52983dcd))
 * **Uncategorized**
    - Release cargo-smart-release v0.21.4 ([`c2398ff`](https://github.com/Byron/cargo-smart-release/commit/c2398ffbc4e0557d95103d39fa9517c330287b1a))
    - Merge branch 'upgrade-dependencies' ([`2d24694`](https://github.com/Byron/cargo-smart-release/commit/2d24694555438e0f5d17195167a17c1fa665f1cb))
    - Replace `time` with `jiff` ([`6d318f8`](https://github.com/Byron/cargo-smart-release/commit/6d318f81735953892e43ecd17ab5ceb057493449))
    - Upgrade `gix` to 0.66 ([`056d306`](https://github.com/Byron/cargo-smart-release/commit/056d3065c49de523a1e06b845c270c1d175814d6))
    - Thanks clippy ([`46ea567`](https://github.com/Byron/cargo-smart-release/commit/46ea5671edc67c0d36a5f6a67446d2c9bba8a725))
    - Adjust journey test expectations, hopefully cross-platform ([`cd2443d`](https://github.com/Byron/cargo-smart-release/commit/cd2443dd038658be84fbfc8264aa2abd066af34f))
    - `just fmt` ([`459c9ce`](https://github.com/Byron/cargo-smart-release/commit/459c9ce3ce4085f80dd97aac401c9b2fbcf27442))
    - Fix clippy ([`d00758b`](https://github.com/Byron/cargo-smart-release/commit/d00758ba0488530d7497ec56f508fff118805be1))
    - Upgrade to latest `gix` ([`779ccca`](https://github.com/Byron/cargo-smart-release/commit/779ccca06293fcfb7d6801b1605c694d66bce964))
    - Update `Cargo.lock` to fix breakage ([`58362a6`](https://github.com/Byron/cargo-smart-release/commit/58362a64761fed02d398df199b4e3429b9311f1e))
    - Merge branch 'allow-moved-stuff' ([`d472a8d`](https://github.com/Byron/cargo-smart-release/commit/d472a8db86003c9e6caa10a53c7e021f7f062b60))
    - Refactor ([`54e0a84`](https://github.com/Byron/cargo-smart-release/commit/54e0a84bdfdc4ed01973880c566fcd54d6419fd0))
    - Allow changelog generation for moved crates, emit warning message ([`91120a3`](https://github.com/Byron/cargo-smart-release/commit/91120a312eb047c315f7d47f0fdeaf32e63d4a52))
    - Merge pull request #13 from hydro-project/err-msg ([`b684fcb`](https://github.com/Byron/cargo-smart-release/commit/b684fcb7f01ad822d5a9ec26f255db1623f756e7))
    - Improve error messages when path is not found in commit ([`8c45160`](https://github.com/Byron/cargo-smart-release/commit/8c45160f3bdf928ebe9549dad9f8d51f25b876f3))
</details>

## 0.21.3 (2024-01-04)

### New Features

 - <csr-id-4c7b6d20399480da6797fb42b954d40d316c85f3/> Support for custom registries with the `--registry <REG-URL>` flag.
   Please note that this information is
   merely passed to `cargo publish --registry <REG-URL>`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-smart-release v0.21.3 ([`7892013`](https://github.com/Byron/cargo-smart-release/commit/7892013f41557d8df5390a6f2855bb5e79632fe6))
    - Support for custom registries with the `--registry <REG-URL>` flag. ([`4c7b6d2`](https://github.com/Byron/cargo-smart-release/commit/4c7b6d20399480da6797fb42b954d40d316c85f3))
    - Upgrade gix to latest version ([`86113bf`](https://github.com/Byron/cargo-smart-release/commit/86113bf58f9303f113d0baba41fb58e6a459e861))
    - Update dependencies ([`168cf48`](https://github.com/Byron/cargo-smart-release/commit/168cf484a2ebd23c174536a59de2dc6172b0e0ce))
    - Upgrade dependencies ([`1071dba`](https://github.com/Byron/cargo-smart-release/commit/1071dbaac051f47ed17d8fa1843fd194ce839a68))
    - Refactor ([`f9df11f`](https://github.com/Byron/cargo-smart-release/commit/f9df11f681f634d566ff3c106e20a142f242a91a))
    - Thanks clippy ([`40eae8d`](https://github.com/Byron/cargo-smart-release/commit/40eae8d4e9ecbb54bfd282bf06a17a57837d9b9d))
    - 🍮 Support for custom registries ([`3abb6df`](https://github.com/Byron/cargo-smart-release/commit/3abb6dfd48b0b5c6c898b44b47b3dfbb62f15b00))
</details>

## 0.21.2 (2023-10-12)

### Bug Fixes

 - <csr-id-1236efa69e313ea22352397424eaafa8f630c5df/> assure we find the highest available version when picking the next version.
   Otherwise it can be that higher versions 'hide' behind lower ones which can
   happen if a backport is made, for example.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 51 calendar days.
 - 51 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-smart-release v0.21.2 ([`82c39db`](https://github.com/Byron/cargo-smart-release/commit/82c39dbc57b755978ec37997642513992a72e292))
    - Assure we find the highest available version when picking the next version. ([`1236efa`](https://github.com/Byron/cargo-smart-release/commit/1236efa69e313ea22352397424eaafa8f630c5df))
    - Thanks clippy ([`2ee59e1`](https://github.com/Byron/cargo-smart-release/commit/2ee59e1843d59f1c4ed75e968d5e2409c9996e9a))
    - Upgrade cargo-metadata to latest version ([`e6e6f75`](https://github.com/Byron/cargo-smart-release/commit/e6e6f75b1b31109577e0d6407f605f9629fbb6cd))
    - Update crates-index to latest version so `gix` is the same ([`58fa999`](https://github.com/Byron/cargo-smart-release/commit/58fa999519dc4b72aaf6fa9d48a47c94ee8e2d6e))
    - Upgrade to `gix` 0.54 ([`54af21a`](https://github.com/Byron/cargo-smart-release/commit/54af21a2f255350ad833a259807a4bdec2fd88aa))
    - Try once more to get the if: syntax right ([`77c921a`](https://github.com/Byron/cargo-smart-release/commit/77c921ae523161f43b8ce34402866e51b9dd5818))
    - Try to speed-up CI by removing rust installation steps in favor of the one included in image ([`df61a57`](https://github.com/Byron/cargo-smart-release/commit/df61a5778e8c4ca6fc01c6055737207ad9d760cc))
    - Prevent journey tests from running (and failing) on windows ([`b5e9634`](https://github.com/Byron/cargo-smart-release/commit/b5e963408ba1e6def18e1a05cf178c46a82d7c9b))
    - Remove last mention of MSRV ([`77bcb37`](https://github.com/Byron/cargo-smart-release/commit/77bcb3793eb8fb0b34d33c457d6cbf9f195389c1))
    - Rename workflow: Test -> eInstallation ([`e7e0666`](https://github.com/Byron/cargo-smart-release/commit/e7e0666ed490a3f8bbea848305eff39bba320d4b))
    - Fix CI workflow ([`ed09a0c`](https://github.com/Byron/cargo-smart-release/commit/ed09a0cfcf59cd4ba19c02b507e285411ce14b83))
    - Add test for installation of binary ([`df23b2c`](https://github.com/Byron/cargo-smart-release/commit/df23b2c23a689f9c3993af3fc96d0fb5fe7c9194))
</details>

## 0.21.1 (2023-08-22)

<csr-id-431cd880e06fcd9e194700739bd7c7a93575c4a0/>

### Chore

 - <csr-id-431cd880e06fcd9e194700739bd7c7a93575c4a0/> update repository to not point at `gitoxide` anymore

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-smart-release v0.21.1 ([`7454750`](https://github.com/Byron/cargo-smart-release/commit/7454750aae2745aac37786f13d9401e591f9c83b))
    - Update repository to not point at `gitoxide` anymore ([`431cd88`](https://github.com/Byron/cargo-smart-release/commit/431cd880e06fcd9e194700739bd7c7a93575c4a0))
</details>

## 0.21.0 (2023-08-22)

<csr-id-528638729492300730aebee283d2a837325b4a62/>
<csr-id-ba76b8bd911b98ab78fec3cf6c8e7ee679721a6f/>
<csr-id-67eb1d9e3d396cc7f786d767e287d7e946ed3118/>
<csr-id-d6075a44bff9073c811510e86d73216baa844a69/>
<csr-id-c829ffcf262464b9d963f3415f44fcb3c53d7a47/>
<csr-id-462b3f878b63a5d483094a8a827f740cb59a96f8/>
<csr-id-7b960c061fda7d371245c52fa94f6957689bbb6a/>
<csr-id-00ad37542abf250ffda04d92cdfbb2db945e58a9/>
<csr-id-7638bb5f5a3e81f7cf62913ad5bd1aac7cff6cbe/>
<csr-id-9658f32181c1c3b3b836eadca5274d38f3dd57c4/>
<csr-id-3ccdb43562a35f2556a0ade9fb1beb8d5e33a2eb/>
<csr-id-64801454750b21f0e72b7b571d1318256b11f4a3/>
<csr-id-f2ad685e56caf4612378649b8c68c99ac3225ccc/>
<csr-id-80cbe46e6710d51a5aa0369bc397ae211eb70dbb/>
<csr-id-bd44a1a34f1f68eb35b043bbf9f3f9bb616e5705/>

This is the first release of from [its own repository](https://github.com/Byron/cargo-smart-release) which for the first time comes without `git2` dependency,
and is solely powered by `gix`.

Please note that due to the rewritten history and tag-name changes, this changelog also looks quite a bit different in its historical parts.

### New Features (BREAKING)

 - <csr-id-0183d55c92dfeaff82f16d13e52fa23427d75e19/> use `crates-index` with https://github.com/frewsxcv/rust-crates-index/pull/129 applied.
   This means `git2` is now fully removed from the tools used by `gitoxide`.
   
   Note that this also means that the `vendored-ssl` feature has been removed as there is no equivalent.
   It might be worth to add feature toggles that change to another backend though.

### Bug Fixes

 - <csr-id-186ab8af0023b419e2df6aa86a102adf10c931b6/> remove `git2` dependency thanks to upgrade to latest version of `crates-index`
   Note that this also removes the `vendored-openssl` feature as it doesn't exist in `crates-index`
   anymore.

### Refactor

 - <csr-id-c829ffcf262464b9d963f3415f44fcb3c53d7a47/> Upgrade to winnow 0.5
 - <csr-id-462b3f878b63a5d483094a8a827f740cb59a96f8/> Move off of quietly deprecated names
   Prep for 0.5
 - <csr-id-7b960c061fda7d371245c52fa94f6957689bbb6a/> Move off deprecated parsers
 - <csr-id-00ad37542abf250ffda04d92cdfbb2db945e58a9/> Switch to ranged take_while
 - <csr-id-7638bb5f5a3e81f7cf62913ad5bd1aac7cff6cbe/> Upgrade to winnow 0.4
 - <csr-id-9658f32181c1c3b3b836eadca5274d38f3dd57c4/> Explicitly parse_next
   This is prep for 0.4
 - <csr-id-3ccdb43562a35f2556a0ade9fb1beb8d5e33a2eb/> Move off of explicitly complete parsers
 - <csr-id-64801454750b21f0e72b7b571d1318256b11f4a3/> Move to FinishIResilt::finish
 - <csr-id-f2ad685e56caf4612378649b8c68c99ac3225ccc/> Move to Parser inherent functions
 - <csr-id-80cbe46e6710d51a5aa0369bc397ae211eb70dbb/> Move off of tuple/tag
 - <csr-id-bd44a1a34f1f68eb35b043bbf9f3f9bb616e5705/> Switch from nom to winnow 0.3

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 33 commits contributed to the release over the course of 31 calendar days.
 - 34 days passed between releases.
 - 17 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-smart-release v0.21.0 ([`72267e0`](https://github.com/Byron/cargo-smart-release/commit/72267e07c3bd78008cccc5ca550e73a0b4cc0061))
    - Prepare changelog ([`4f6afe2`](https://github.com/Byron/cargo-smart-release/commit/4f6afe2e3a5b8903bd0b70a4cc16e6333958ead3))
    - Remove msrv check as it's not needed for a binary ([`abaad04`](https://github.com/Byron/cargo-smart-release/commit/abaad04ac2b72e9a763eea484ac6f547a132a9ef))
    - Provide justfile and improve automation ([`7e2a6ca`](https://github.com/Byron/cargo-smart-release/commit/7e2a6cab25de699f395daabf98e15eefa8b2aa23))
    - Update dependencies ([`aea1a51`](https://github.com/Byron/cargo-smart-release/commit/aea1a519ceeffae53b563e3ff30314db01a73649))
    - Fix license check ([`a9c135c`](https://github.com/Byron/cargo-smart-release/commit/a9c135c26a9f77c2dc0e10019f1ffc5202722c54))
    - Adjust template to better fit preferences ([`8afc4ec`](https://github.com/Byron/cargo-smart-release/commit/8afc4ec3cf75ce9da2f4eddc35580af4d3be3349))
    - Merge remote-tracking branch 'template/main' into gix-submodule ([`7a62b5b`](https://github.com/Byron/cargo-smart-release/commit/7a62b5b694111529d4b511d08cad6bb5c409c79b))
    - Upgrade to Winnow 0.5 ([`c146a4c`](https://github.com/Byron/cargo-smart-release/commit/c146a4cdd59d6aaf1fe7016b4ebfeea1bc50aa8e))
    - Update pre-commit hooks ([`5286387`](https://github.com/Byron/cargo-smart-release/commit/528638729492300730aebee283d2a837325b4a62))
    - Ensure latest deps are good ([`ba76b8b`](https://github.com/Byron/cargo-smart-release/commit/ba76b8bd911b98ab78fec3cf6c8e7ee679721a6f))
    - Only run certain journey tests if the `gh` program is installed. ([`edc8442`](https://github.com/Byron/cargo-smart-release/commit/edc8442ac8a66fef2a55d4915188b90bc9b9116e))
    - Ensure lockfile isn't stale ([`67eb1d9`](https://github.com/Byron/cargo-smart-release/commit/67eb1d9e3d396cc7f786d767e287d7e946ed3118))
    - Update `time` crate explicitly in Cargo.toml to latest version ([`600d0e0`](https://github.com/Byron/cargo-smart-release/commit/600d0e0ce41c87f26baaff7927f7ee7f969a42cf))
    - Expand update window so more likely to be hit ([`d6075a4`](https://github.com/Byron/cargo-smart-release/commit/d6075a44bff9073c811510e86d73216baa844a69))
    - Upgrade smart-release to the latest `crates-index` ([`9bcd57f`](https://github.com/Byron/cargo-smart-release/commit/9bcd57ff6ba2641a78a5659dbce44701b6590120))
    - Remove hardcoded target dir from justfile ([`2011ba6`](https://github.com/Byron/cargo-smart-release/commit/2011ba6af0016029d381fba41a17327565202ba0))
    - Remove `git2` dependency thanks to upgrade to latest version of `crates-index` ([`186ab8a`](https://github.com/Byron/cargo-smart-release/commit/186ab8af0023b419e2df6aa86a102adf10c931b6))
    - Upgrade to winnow 0.5 ([`c829ffc`](https://github.com/Byron/cargo-smart-release/commit/c829ffcf262464b9d963f3415f44fcb3c53d7a47))
    - Move off of quietly deprecated names ([`462b3f8`](https://github.com/Byron/cargo-smart-release/commit/462b3f878b63a5d483094a8a827f740cb59a96f8))
    - Move off deprecated parsers ([`7b960c0`](https://github.com/Byron/cargo-smart-release/commit/7b960c061fda7d371245c52fa94f6957689bbb6a))
    - Switch to ranged take_while ([`00ad375`](https://github.com/Byron/cargo-smart-release/commit/00ad37542abf250ffda04d92cdfbb2db945e58a9))
    - Upgrade to winnow 0.4 ([`7638bb5`](https://github.com/Byron/cargo-smart-release/commit/7638bb5f5a3e81f7cf62913ad5bd1aac7cff6cbe))
    - Explicitly parse_next ([`9658f32`](https://github.com/Byron/cargo-smart-release/commit/9658f32181c1c3b3b836eadca5274d38f3dd57c4))
    - Move off of explicitly complete parsers ([`3ccdb43`](https://github.com/Byron/cargo-smart-release/commit/3ccdb43562a35f2556a0ade9fb1beb8d5e33a2eb))
    - Move to FinishIResilt::finish ([`6480145`](https://github.com/Byron/cargo-smart-release/commit/64801454750b21f0e72b7b571d1318256b11f4a3))
    - Move to Parser inherent functions ([`f2ad685`](https://github.com/Byron/cargo-smart-release/commit/f2ad685e56caf4612378649b8c68c99ac3225ccc))
    - Move off of tuple/tag ([`80cbe46`](https://github.com/Byron/cargo-smart-release/commit/80cbe46e6710d51a5aa0369bc397ae211eb70dbb))
    - Switch from nom to winnow 0.3 ([`bd44a1a`](https://github.com/Byron/cargo-smart-release/commit/bd44a1a34f1f68eb35b043bbf9f3f9bb616e5705))
    - Use `crates-index` with https://github.com/frewsxcv/rust-crates-index/pull/129 applied. ([`0183d55`](https://github.com/Byron/cargo-smart-release/commit/0183d55c92dfeaff82f16d13e52fa23427d75e19))
    - Upgrade `crates-index` to 1.0 ([`a292362`](https://github.com/Byron/cargo-smart-release/commit/a2923626fa40f8d4c35c667dba892525981ea8a2))
    - Release gix-features v0.32.1, gix-actor v0.24.1, gix-validate v0.7.7, gix-object v0.33.1, gix-path v0.8.4, gix-glob v0.10.1, gix-quote v0.4.6, gix-attributes v0.16.0, gix-command v0.2.8, gix-packetline-blocking v0.16.4, gix-filter v0.2.0, gix-fs v0.4.1, gix-chunk v0.4.4, gix-commitgraph v0.18.1, gix-hashtable v0.2.4, gix-revwalk v0.4.1, gix-traverse v0.30.1, gix-worktree-stream v0.2.0, gix-archive v0.2.0, gix-config-value v0.12.5, gix-tempfile v7.0.1, gix-utils v0.1.5, gix-lock v7.0.2, gix-ref v0.33.1, gix-sec v0.8.4, gix-prompt v0.5.4, gix-url v0.21.1, gix-credentials v0.17.1, gix-diff v0.33.1, gix-discover v0.22.1, gix-ignore v0.5.1, gix-bitmap v0.2.6, gix-index v0.21.1, gix-mailmap v0.16.1, gix-negotiate v0.5.1, gix-pack v0.40.1, gix-odb v0.50.1, gix-packetline v0.16.4, gix-transport v0.34.1, gix-protocol v0.36.1, gix-revision v0.18.1, gix-refspec v0.14.1, gix-worktree v0.23.0, gix v0.50.0, safety bump 5 crates ([`9ce302f`](https://github.com/Byron/cargo-smart-release/commit/9ce302f3ec2e26eeedc0976e0a4e8f662e7888a7))
    - Adapt to changes in `gix` ([`88dfdfb`](https://github.com/Byron/cargo-smart-release/commit/88dfdfb374e1044ec558f1f9520ba8a628b7b93d))
</details>

## 0.20.0 (2023-07-19)

<csr-id-bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d/>
<csr-id-dbc6cbb4363c2532f81b0bd6e351c4577bb9e9a3/>
<csr-id-03c57aeb4586059a5065c56ff56044f24880af90/>
<csr-id-feb40e0f068c299ff3a5204caad0c3834bf0b9c9/>
<csr-id-79e37f5a53802f4f2499d426856d0ea0d316599d/>
<csr-id-96fd2c1b2db3ba7959a97d212f64ac0ed7184132/>
<csr-id-d99db2e632b25a8b020491c3e1d40bf2efd3472a/>
<csr-id-4163ad78c72df3a993bea6084fc05c6a2a44b9c2/>
<csr-id-4452ea424cf11a56e48ceb0e2764d31f7e2c242e/>
<csr-id-45f23a30c715e3da15deeb5c4b808c99e3b80033/>
<csr-id-ee8a2b5fa6dc2e586767d60d141f1abade64fe8f/>
<csr-id-f2d2bfad7c65c6d8a961bbbb0970f27de5a5c33e/>
<csr-id-17173c85ac4806e1ecdbd06bacc70b37d5647779/>
<csr-id-33a2d9f1ebbfb078649dad5ce07fae0aa1e857b4/>
<csr-id-8ab83d5a84f341c34cc059e27e45019390ec25d5/>
<csr-id-85bd41841d2721e857d605f42ac021bd37285ede/>
<csr-id-e746ee9791ea086ff98a79efb522d177c64f4846/>
<csr-id-4f3808d76a5a732970c5ef744a8375e7ab729357/>
<csr-id-489c035a0366e98a0654b824df577ca6287ab964/>
<csr-id-4d2e66b2593a1360f4405fd319a290d98e336987/>
<csr-id-6d4ba38f7c61860c068bb351c521fdaec4cdcbc6/>
<csr-id-0bb7f48f2b2c3cdaf4dadac5163f03b923489cf5/>
<csr-id-245c84673a59e3569d0ab628a3051959c5c48b7a/>
<csr-id-a6e2c581cfcf2e8395b7ead3e4d82a0a5a47d25f/>
<csr-id-4f0c9fb70630abccd83a43beb338d504f62381c2/>
<csr-id-689150655fe6b9bd83cb1d6a3c6c461f6bca38a3/>
<csr-id-bfe6febccc3f3631566e6d24b32b5d92e73d49f9/>
<csr-id-bfc2733b28276e916eabfb2137c983fb07cc6eed/>
<csr-id-a2a750c35df6faaeda50db2a61e7b5bfcc21f9be/>
<csr-id-5a4c43abb465c24a37ebc2697aa2d4675e3bef96/>
<csr-id-721c8f7acdade982f617299077ef3958c379f55b/>
<csr-id-d847ff8af4d2f82b6aeb8e02bbed17570fff760b/>
<csr-id-b0f93567029381414eea9e7752ebaa251da11502/>
<csr-id-aac36685b88075553241fc55e83982da4b4d7e82/>
<csr-id-af8eeeb31f33484f84b8e37958802a4e24f58edc/>
<csr-id-571c98ed65eb162d6e4d227653bc41e2fa053eb1/>
<csr-id-59e477a7dbe738594e00dcba53d685e099933adf/>
<csr-id-505c94854a00fc135cd037ef848ed4eb8a6ade2e/>
<csr-id-12c4b49d956999e9b9151a26bded7f103bea73e1/>
<csr-id-11b7ce634faeb68709cf9ee4b36aefed846e3cce/>
<csr-id-9d29488b124a37a252db60b9e84ad2911484b44a/>
<csr-id-df9204e93d735329a012bd2256945a63980a5590/>
<csr-id-62705deb6f8119e30b92af83616292628551f050/>
<csr-id-4dfdc898f5930bab82bbfb8c871aa1df40b03e88/>
<csr-id-0d2cb64ac5d66328449793683fd9bb866b851f02/>
<csr-id-ed56f26e4f09432f4eca4c05638313edb25cd493/>
<csr-id-da676e5732eec0b633748eb640009a3c64f2e7f7/>
<csr-id-ba1ba749b4fa9df368771b64eeed4625d4479dab/>
<csr-id-f6f441479cde179d92b095889797adeda3bce379/>
<csr-id-a1db0f1900e465b8bfb521a82691f2567bf357fd/>
<csr-id-4f9f1f94d950b22d730eaa4d31d91cab7ca5890a/>
<csr-id-9fd83a0e020cf4c45b1874290dbd7ebdba69d738/>
<csr-id-151f10e895145241b91d29e1b9db723409798684/>
<csr-id-8b96c9d7373fc31ba068aefad51c25a4cab3b87c/>
<csr-id-de1155bb13e02fcc67eaee6cbdc3f9cf1334e537/>
<csr-id-9f2900c62a182de085869263289eb06c6624c8a0/>
<csr-id-69473e9274034c4f7844fcba6859ae9e3ff84547/>
<csr-id-cc08f344687221476d87318b76ded2b0ded488de/>
<csr-id-654d64604a455c43547c40bf003eaf19289376fb/>
<csr-id-6c067c73fd73914d4a135f7072065b9123059d3f/>
<csr-id-eab816ae90b83940d549f110d00b69671f699fdb/>
<csr-id-307bf1b8f11b9d7bc787d7cadf151a6310a15765/>
<csr-id-1cea98086a6676594c099162f72f7c910127f755/>
<csr-id-b196db6d12a353fbf0b5d5dd61c98099f997866e/>
<csr-id-4d44cd7ca51f05fb06185677642d73c0ff0da079/>
<csr-id-2b6bb28cd18916a6244a2632a6abcba9362b9fd0/>
<csr-id-80d4cdd688e88b897f384b770f9c13268ecb3793/>
<csr-id-716170eaa853ddf3032baa9b107eb3e44d6a4124/>
<csr-id-96297f038d8d931bb9d5ba4dfcdced18d7c81061/>
<csr-id-60a8ec89e3f97baad0dbe097e03dc0cd30899e02/>
<csr-id-afaba35d39c75d13138e2928cddeb0b93601cee3/>
<csr-id-62401b8eafb71d8a928137f6f8dfc25340e39bbf/>
<csr-id-2c4a7f574f6fed6655e8b2f25916c22d7bf08ad1/>
<csr-id-563de12d25e777e7244a73308090adcfb8b90014/>
<csr-id-6c8df60dc4015279cef303cab8f4760efb5ebea8/>
<csr-id-d1dd4ae94067be2f3158fa46b0e78504705dfb26/>
<csr-id-037f37906dad6d39f9fad371bc9a8ab76e8bd5c4/>
<csr-id-afd6a45ef73201bf5d5f3d4f0317f432b17c60d0/>
<csr-id-083884043cc08394c6f91df81e6407721b2dc19e/>
<csr-id-2768727452315929d88dda7d0686440d8e668736/>
<csr-id-afeff23549a05cd0e5997f129e5d7a564ec41866/>
<csr-id-fbaab420b9e4e01e60522f87e89e2e0a28250c73/>
<csr-id-e7b7555d1516d0b274e7269961fce9ec9b30bc98/>
<csr-id-19b6df4e9d25d502ec4e21cb950a186f8b4300ce/>
<csr-id-38a07f2ad061f2eba143532c94d23b4254aec438/>
<csr-id-38e3835b1b1fb90a7596b3cb08dd18c903a5acce/>

### Chore

 - <csr-id-bcad5c22049d56a25ef69d6c7a3344e78f9a1d4d/> Add `clippy::redundant-closure-for-method-calls` lint
 - <csr-id-dbc6cbb4363c2532f81b0bd6e351c4577bb9e9a3/> inline format args

### Style (BREAKING)

 - <csr-id-03c57aeb4586059a5065c56ff56044f24880af90/> rename --skip-dependencies to --no-dependencies…
   …to be more inline with existing terminology of other flags.

### Refactor (BREAKING)

 - <csr-id-feb40e0f068c299ff3a5204caad0c3834bf0b9c9/> clarify different repository types much better
 - <csr-id-79e37f5a53802f4f2499d426856d0ea0d316599d/> Remove --no-multi-crate-release support entirely
   As the default is to do multi-crate releases and now having to deal
   with single-create releases reduces maintenance burden.
   
   The solution to this problem is to not specify versions in
   dev-dependencies to workspace crates.
   
   We also don't check for this anymore, which might be re-added
   at some point if there is demand.
 - <csr-id-96fd2c1b2db3ba7959a97d212f64ac0ed7184132/> Use 'to_*' when converting `easy::Object` to specific object kind
   This also makes the API more consistent while being more idiomatic.

### New Features (BREAKING)

 - <csr-id-963798af8339ee3d1278aea3c2abca26e1a674c1/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `git-features` and `git-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.
 - <csr-id-afa0a50d51f3e15186c5fac7ee817d169fc989cb/> upgrade to crates-index 0.18
   It now assumes that the crates-index must exist, which migth not always
   be the case and rightfully so. Now we wrap it to get back to the
   original behavior.
 - <csr-id-338eb4b53da0c00d810a6a154d4ea7fe48536a5c/> rename --skip-* flags to --no-* for consistency

### Changed (BREAKING)

 - <csr-id-9bc4049fc3babeef1a4a5b3804dcb8eb4877000e/> Remove easy::borrow::Error entirely; support for multiple objects per handle
   This massive simplification finally allows any amounts of objects to be
   created while adding support for reusing their data buffers thanks
   to a simple free-list stored with the handle.
 - <csr-id-d000e81d054fba5d75d5e6cdc338d295c6553725/> rename `easy::Object` to `OwnedObject`; remove `Ref` suffix from `ObjectRef` and `TreeRef`
 - <csr-id-4d4cb23503e9bc0e8c9a0ac80217e23e87afbd0e/> 'changelog' subcommand change --dependencies to --no-dependencies
   This makes dependency resolution similar to cargo smart-release by
   default and is less surprising.
 - <csr-id-eaca66c77fbdf2f0db7eaf5e03edb316b1a9ae95/> Remove `--only` alias and invert `--no-dependencies` to `--dependencies`
   This leads to easier usage in the common case and helps avoid confusion
   when --dependencies is used and it's not picking up safety bumps on the
   way, like `smart-release` would.
 - <csr-id-5a3bbcb4496f5bfcc624b6f9e967af0d63ac2ed3/> rename short name for `--execute` to `-e` from `-n` for consistency
 - <csr-id-092fa282ad95b4b2493711883e2978442ebd38a4/> rename `ObjectAccessExt::tag(…)` to `*::tag_reference(…)`, add `easy::Object::try_to_tag()`
   This one also contains the first and probably only test for tag object
   creation.

### Style

 - <csr-id-d99db2e632b25a8b020491c3e1d40bf2efd3472a/> Match auto-generated style
 - <csr-id-4163ad78c72df3a993bea6084fc05c6a2a44b9c2/> Match auto-generated style
   This will make reviewing auto-update PRs easier

### Refactor

 - <csr-id-4452ea424cf11a56e48ceb0e2764d31f7e2c242e/> split data::output::count::objects into files

### Other

 - <csr-id-45f23a30c715e3da15deeb5c4b808c99e3b80033/> try to auto-udpate crates index with lifetime crazyness
   Even though it could work, it's too complicated.
 - <csr-id-ee8a2b5fa6dc2e586767d60d141f1abade64fe8f/> try to assure that breaking changes are always published in correct order
   The problem here is that even though we can turn non-publishable breaks
   into publishable ones without loosing information, they will not be in
   the correct order.
   
   The solution is to merge dependency trees instead of clearing them with
   weird logic.
 - <csr-id-f2d2bfad7c65c6d8a961bbbb0970f27de5a5c33e/> :remote_url() is now optional
   Otherwise it wouldn't work on repos that don't have a remote set yet.
   Instead of failing, we don't create links.
 - <csr-id-17173c85ac4806e1ecdbd06bacc70b37d5647779/> add git-conventional
 - <csr-id-33a2d9f1ebbfb078649dad5ce07fae0aa1e857b4/> consider nom for custom parsing, but…
   …realize that the easiest way is definitely the excellent
   git-conventional crate.
   
   This also means we have to stop specifying crates in commit messages
   or find another way to do that.
 - <csr-id-8ab83d5a84f341c34cc059e27e45019390ec25d5/> refactor
 - <csr-id-85bd41841d2721e857d605f42ac021bd37285ede/> refactor
 - <csr-id-e746ee9791ea086ff98a79efb522d177c64f4846/> refactor
 - <csr-id-4f3808d76a5a732970c5ef744a8375e7ab729357/> a seemingly slow version of path lookup, but…
   …in debug mode it's faster than the fast path, despite doing more
   and being the same when it comes to searching path components.
 - <csr-id-489c035a0366e98a0654b824df577ca6287ab964/> fast filter by single-component path
 - <csr-id-4d2e66b2593a1360f4405fd319a290d98e336987/> prepare for fast lookup of paths
 - <csr-id-6d4ba38f7c61860c068bb351c521fdaec4cdcbc6/> configure caches with env vars using `apply_environment()`
 - <csr-id-0bb7f48f2b2c3cdaf4dadac5163f03b923489cf5/> refactor
 - <csr-id-245c84673a59e3569d0ab628a3051959c5c48b7a/> set package cache via RepositoryAccessExt
 - <csr-id-a6e2c581cfcf2e8395b7ead3e4d82a0a5a47d25f/> object-cache to allow for a speed boost…
   …by avoiding duplicate accesses to hit the object database.
   However, the cost for the cache are relatively high and involve some
   memory copying, so hit rates of about 50% is certainly what is needed
   to get any speed boost at all.
 - <csr-id-4f0c9fb70630abccd83a43beb338d504f62381c2/> actually build the segment vec, without pruning for now
 - <csr-id-689150655fe6b9bd83cb1d6a3c6c461f6bca38a3/> build commit history for later use in changelog generation
 - <csr-id-bfe6febccc3f3631566e6d24b32b5d92e73d49f9/> sketch history acquisition
 - <csr-id-bfc2733b28276e916eabfb2137c983fb07cc6eed/> add 'Head::peeled()' method
 - <csr-id-a2a750c35df6faaeda50db2a61e7b5bfcc21f9be/> some performance logging
 - <csr-id-5a4c43abb465c24a37ebc2697aa2d4675e3bef96/> build ref lookup table
 - <csr-id-721c8f7acdade982f617299077ef3958c379f55b/> loose reference iteration with non-dir prefixes…
   Previously it was expected for the prefix `Path` to always exist for
   the prefix to be valid. This, however, is not similar to packed
   prefixes, which allow non-dir prefixes as well.
   
   Now we will check if the prefix is actually a directory, and if not
   split it into its parent directory and the filename portion. The latter
   is then used for prefix matching file names within that directory.
 - <csr-id-d847ff8af4d2f82b6aeb8e02bbed17570fff760b/> Add 'references().all().peeled().'…
   …to not only make typical usage of iterated references more convenient
   but also work around a double-borrow error one would see otherwise.
 - <csr-id-b0f93567029381414eea9e7752ebaa251da11502/> filter refs correctly, but…
   …it needs a way to peel references right away without trying
   to double-borrow. This means the Iterator needs to implement this.
 - <csr-id-aac36685b88075553241fc55e83982da4b4d7e82/> find tag references by name…
   …even though it's clear that loose refs won't be found with prefixes
   that aren't directories, but contain a partial file.
   
   This is more like a bug to be fixed, as that works naturally for
   packed-refs for instance.
 - <csr-id-af8eeeb31f33484f84b8e37958802a4e24f58edc/> improve changelog format
 - <csr-id-571c98ed65eb162d6e4d227653bc41e2fa053eb1/> sketch first step of info generation
 - <csr-id-59e477a7dbe738594e00dcba53d685e099933adf/> changelog gets crates to work on
 - <csr-id-505c94854a00fc135cd037ef848ed4eb8a6ade2e/> handle unborn heads
 - <csr-id-12c4b49d956999e9b9151a26bded7f103bea73e1/> fmt
 - <csr-id-11b7ce634faeb68709cf9ee4b36aefed846e3cce/> refactor
 - <csr-id-9d29488b124a37a252db60b9e84ad2911484b44a/> refactor
 - <csr-id-df9204e93d735329a012bd2256945a63980a5590/> refactor
 - <csr-id-62705deb6f8119e30b92af83616292628551f050/> initial test for changelog
   Which doesn't test that much.
 - <csr-id-4dfdc898f5930bab82bbfb8c871aa1df40b03e88/> very basic support for changelog command…
   …which shows that it probably just wants to be separate for now before
   being integrated?
 - <csr-id-0d2cb64ac5d66328449793683fd9bb866b851f02/> add 'cargo changelog' sub-command binary
 - <csr-id-ed56f26e4f09432f4eca4c05638313edb25cd493/> add changelog to most tests
 - <csr-id-da676e5732eec0b633748eb640009a3c64f2e7f7/> assure the current package version is actually breaking
 - <csr-id-ba1ba749b4fa9df368771b64eeed4625d4479dab/> better verbosity handling when comparing to crates-index
 - <csr-id-f6f441479cde179d92b095889797adeda3bce379/> turn off safety bump with its own flag
 - <csr-id-a1db0f1900e465b8bfb521a82691f2567bf357fd/> improved safety bump log message
 - <csr-id-4f9f1f94d950b22d730eaa4d31d91cab7ca5890a/> commit message reveals safety bumps
 - <csr-id-9fd83a0e020cf4c45b1874290dbd7ebdba69d738/> released crates only receive minor bumps…
   …which signals a change while allowing decendents to pin themselves to
   patch updates only.
   
   This would be users of "unstable" git-repository features for example.
   which then also don't want to see new minor versions automatically
   as it may cause breakage.
 - <csr-id-151f10e895145241b91d29e1b9db723409798684/> update changelog
 - <csr-id-8b96c9d7373fc31ba068aefad51c25a4cab3b87c/> way more tests to nail current log output
   This is the basis for adjusting the output verbosity or information
   where it matters.
 - <csr-id-de1155bb13e02fcc67eaee6cbdc3f9cf1334e537/> dependency upgrade works
 - <csr-id-9f2900c62a182de085869263289eb06c6624c8a0/> calculate new version of dependent
 - <csr-id-69473e9274034c4f7844fcba6859ae9e3ff84547/> don't claim "conservative" updates for major version change
 - <csr-id-cc08f344687221476d87318b76ded2b0ded488de/> assure we can find non-sequential connections
 - <csr-id-654d64604a455c43547c40bf003eaf19289376fb/> all logic to calculate dependent version bumps
 - <csr-id-6c067c73fd73914d4a135f7072065b9123059d3f/> an algorithm to collect dependencies by 'growing'
 - <csr-id-eab816ae90b83940d549f110d00b69671f699fdb/> foundation for bumping versions
   The idea is that the dependency traversal may also produce a new version
   number, which is when it will naturally be set for all dependents later.
 - <csr-id-307bf1b8f11b9d7bc787d7cadf151a6310a15765/> 

### Documentation

 - <csr-id-614b0a2376b9ae6d95a1b768b93d06057f4b82d6/> Remove reference to travis
 - <csr-id-d6b4446cd761d82313a0e69cf0da82ebfc4084cb/> Set changelog base
 - <csr-id-e866c4d97dfbdc6dde52c750f7c3d34c0be43709/> fix minor typos
 - <csr-id-09d6986a15f67e48a0184cbca927dc05c51be604/> fix typos

### Chore

 - <csr-id-1cea98086a6676594c099162f72f7c910127f755/> Add `clippy::redundant-closure-for-method-calls` lint
 - <csr-id-b196db6d12a353fbf0b5d5dd61c98099f997866e/> inline format args
 - <csr-id-4d44cd7ca51f05fb06185677642d73c0ff0da079/> Update precommit hooks
 - <csr-id-2b6bb28cd18916a6244a2632a6abcba9362b9fd0/> Catch clippy config failures
 - <csr-id-80d4cdd688e88b897f384b770f9c13268ecb3793/> Remove clippy lint past MSRV (needs 1.67)
 - <csr-id-716170eaa853ddf3032baa9b107eb3e44d6a4124/> Ban rebase merges
 - <csr-id-96297f038d8d931bb9d5ba4dfcdced18d7c81061/> Clarify why map_or is banned
 - <csr-id-60a8ec89e3f97baad0dbe097e03dc0cd30899e02/> Ban for_each
 - <csr-id-afaba35d39c75d13138e2928cddeb0b93601cee3/> Use new minimumReleaseAge field
 - <csr-id-62401b8eafb71d8a928137f6f8dfc25340e39bbf/> Lower the MSRV churn for template
 - <csr-id-2c4a7f574f6fed6655e8b2f25916c22d7bf08ad1/> Delay Renovate PRs until ready
 - <csr-id-563de12d25e777e7244a73308090adcfb8b90014/> Update stabilidyDays to new syntax
 - <csr-id-6c8df60dc4015279cef303cab8f4760efb5ebea8/> Include Cargo.lock
 - <csr-id-d1dd4ae94067be2f3158fa46b0e78504705dfb26/> Expand approved licenses
 - <csr-id-037f37906dad6d39f9fad371bc9a8ab76e8bd5c4/> Remove rustfmt/clippy next jobs
 - <csr-id-afd6a45ef73201bf5d5f3d4f0317f432b17c60d0/> Use workspace inheritance
 - <csr-id-083884043cc08394c6f91df81e6407721b2dc19e/> Update release process
 - <csr-id-2768727452315929d88dda7d0686440d8e668736/> Don't set rustflags by default
   Doing so can cause unnecessary recompilation
 - <csr-id-afeff23549a05cd0e5997f129e5d7a564ec41866/> Quote strings in yaml
 - <csr-id-fbaab420b9e4e01e60522f87e89e2e0a28250c73/> update msrv to v1.65.0
 - <csr-id-e7b7555d1516d0b274e7269961fce9ec9b30bc98/> First step
 - <csr-id-19b6df4e9d25d502ec4e21cb950a186f8b4300ce/> upgrade to clap 4.1
 - <csr-id-38a07f2ad061f2eba143532c94d23b4254aec438/> remove default link to cargo doc everywhere
 - <csr-id-38e3835b1b1fb90a7596b3cb08dd18c903a5acce/> upgrade all dependencies

### New Features

 - <csr-id-4a244a2c9d1007fa10afa6f5078747bd2b001eb5/> add --capitalize-commit option to capitalize commit message in cargo-smart-release
 - <csr-id-09aae21add04b2edbaa1489f1b47cc0886da0062/> add `vendored-openssl` feature toggle.
   This should help the build on some platforms.
 - <csr-id-b9312a1e405544517c6ef147f8e42142327a3bc9/> add --capitalize-commit option to capitalize commit message in cargo-smart-release
 - <csr-id-f5649e2cbfc6c369354ea8aff02420d1748693ce/> avoid panics in favor of error handling. That way more information can be provided which helps with a fix.
 - <csr-id-84aeb51fd6099d8cfa6ec4db08b78e06728d0664/> rename tracking for crates in the crate-root.
   Now it's possible to rename crates if they are directly at the crate root
   without loosing their history.
 - <csr-id-9ee78e7426d26ca010ed935f92566ed2f1fc3201/> highlight (non-fatal) errors when losslessly parsing changelogs
 - <csr-id-14af24b24977d7bfcd3297760eb2080a3b403ed2/> Commit statistics reveal the days passes between releases
 - <csr-id-659d02a0661deb18db2271bc89c66c1609c83502/> auto-update crates-index if there is an indication
   There is the possibility of false-positives triggering such an update
   if manifests are edited by hand, which is not the common case.
   
   If it is, please let us know.
 - <csr-id-0d203345367e28823f05ed1f8dbf26d47d04d6d6/> 'changelog' understands '-e/--execute' as well.
   This makes writing changelogs before release easier as the command-line
   has to change less.
 - <csr-id-48a2dcca9de7ef243d8d7990ca0e7f7cdd3360a8/> Support for no prefixes in version headers
   These are also inherited so once set by a single versioned release
   section, fully generated sections will inherit their prefix from
   that one.
 - <csr-id-eab7fbe00a99f70c1af048278464f6416543d485/> Add `-d` short flag for `--allow-dirty` in `changelog`
 - <csr-id-9e9f3b61b9c507915d7af9738cb6295e40671df8/> `changelog` subcommand fails if there is nothing to do
 - <csr-id-71e2dc9d7db48c7a6dd7eb07600a0b145a412b72/> Respect publish=false in cargo manifest
 - <csr-id-f12956dc716dffe1a0d13e7cc1a29be70ffc53f0/> Perform safety bumps without forcing a publish
   Which is what's required to assure that future publishes of such
   transitively dependent crates won't cause downstream breakage.
 - <csr-id-77f787711407e847102bfad66cb72e4044e180de/> Inform about safety bumps more explicitly
 - <csr-id-6c613be2dd4e97d61a6f529e4bde0d7283a99033/> `--no-dependencies` now has `--only` as alias
 - <csr-id-6b5f7b39fbaf79b2c1746cb52fdcb26e8a147c1f/> add easy::ext::ObjectAccessExt::tag(…) to create tag objects
   It's a quick sketch on how tag object creation could work.
   
   Note the duplication the method name using traits, which seems like a good solution
   to the problem of differentiating tag objects and tag references while
   keeping the method name short.
   
   Most will only ever need one, right?
   
   Even in my example that's not the case, so maybe we have to rename it.
 - <csr-id-1fbef37ee6234f59fa694adc1baa1c828da9d836/> Make `git_url::Url` available under `git_repository::Url`
 - <csr-id-ec34ead31b57a8e7a0f117dc77f6ce4f74324ca5/> smart-release with --changelog-without option…
   …to allow disabling various changelog segments like clippy, or commit
   statistics.
   
   Note that it's always possible to delete individual sections afterwards.
 - <csr-id-afe692af21a53f02e9ecefca64ba2caad8b21791/> changelog command learns the --without <section> option
   With it one can omit auto-generated sections of the given name.

### Bug Fixes

 - <csr-id-3d8d9f36206c7e8dcd2d38a085ad93a6295d3231/> fix docs generation
   URLs need to be escaped. Plus added doc build to CI without deps
 - <csr-id-147cfe1e13dc60bfefe6b5299b8c01550f1577db/> fix docs generation
   URLs need to be escaped. Plus added doc build to CI without deps
 - <csr-id-f7b990b803a4aa448e81a323df3a54e66d2d8df4/> Fix Renovate regexes
 - <csr-id-1a9f4b9027acfcacbbc81dbdafe4b198db0e510f/> $HOME detection on windows
 - <csr-id-a327b8aec3d00c568630121c1af267accf833c5d/> enable local-offset support in the `time` crate and opt-in to it.
   This should allow proper times for release dates like before as they
   respect the local time, instead of defaulting to UTC-time.
 - <csr-id-3ebf12e76678db9c1c6ebd964f6ef4e6811ba111/> assure we can track dependencies correctly.
   Previously, if worktree crates would also be used as crates.io crates,
   the dependency traversal would fail to find packages that come in from crates.io
   as opposed to the workspace, and discard them, causing dependencies to be missed.
   
   Now we correctly ignore workspace dependencies from crates.io.
 - <csr-id-ce965582a2e914190a30982559cfa4e5a98a578a/> handle worktree members which are also used as dependencies from crates.io.
   Previously there would be an assertion error if worktree members
   are not used only by path, but also by dependency to crates.io.
 - <csr-id-83e647f427ba959d19036fe0f9a35b54f1d5ce1b/> Disable tag.gpgSign in test scripts
   This is done for the same reason that commit.gpgsign is disabled for test
   scripts. It prevents test failures if the user has tag.gpgsign enabled in
   their global git config when invoking tests.
 - <csr-id-2ac7c93d9698dc1227984c3427bf221b047982f8/> `where -> were` typo fix.
 - <csr-id-39f2c7dfaae72057784091a1fbeae524c7691d0f/> build complete history information to match with `did crate changed` queries
   Previously it was possible see a crate was changed, but didn't receive a
   version bump which would in turn halt the release process.
   
   The issue was an algorithm inabuility to find changes in the commitgraph
   because it would not look at the correct tree, causing trees to be
   missed entirely. This in turn caused it to not see changes that were
   present and the mismatch in question.
 - <csr-id-83f9c09430cae18d69bb57eedcb8b4822af6f39a/> log errors if these log messages cause stopping the release process.
   Previously it was possible see `log::warn` but have the process abort
   with proclaimed errors which weren't obvious. Now they are `log::error`
   as one would expect.
 - <csr-id-7f422f264d11586b80540e8d1014a8ebf7000ff4/> Assure `git@github.com/user/repo` urls transform into https urls correctly.
 - <csr-id-be95bcb34086e4de77626f5232fa562dd2f03c9d/> allow dependency edits to apply to `target.<cfg>.*dependencies`.
   Previously these would be skipped, which would cause the publish to
   abort due to invalid manifests - some dependencies would still refer
   to an outdated but incompatible version.
 - <csr-id-8647f724ef5fa00edcd886e44c039f80d72211c4/> List any dependency update that is caused by other crates in preview.
   Previously it was possible that crates there were about to be published
   didn't show up in the list of crates that received a safety version
   bump.
 - <csr-id-3be525aea817a0a1c217456c9f1fa9e4e39b847f/> Avoid running into the `default-members` trap with 'cargo publish'.
   Default-members in a cargo workspace can override what's actually
   published, so we have to be explicit about what to publish.
   
   This is only the case when there is more than one members in the
   workspace, even though it would probably work as well if the package
   would be specified with a single-crate workspace.
 - <csr-id-e34556be1f0fed4c6413681ae2b936d8de19cb1e/> Correctly determine top-level crate name.
   Previously it was possible to think the crate is part of a multi-crate
   worktree even though it wasn't, causing changelogs to not pick up their
   history as it would look for different tag names.
 - <csr-id-b8b7345e3a7c07746921313f8933fe658f284065/> Don't assume crates are non-breaking just because they are in the user selection.
   Crates shownig up 'early' in our list could cause the entire
   breakage-propagation to fail which led the crate to be ignored entirely
   even when their dependees changed their version. This led to
   inconsistent version requirements which would abort any cargo call.
 - <csr-id-7634f69f76480a0b76a58e3565f27d925d8fd844/> improve headline parsing for git-conventional messages.
   It is now case-insensitive, which prevents it from getting tripped
   up in some cases.
 - <csr-id-ff553150c9a0195e0e561c3f75e96b5d4c27651a/> Don't pass judgement on usefulness of certain kinds of git-conventional messages
   Previously we would intentionally avoid writing out information about
   refactors or chores as they are not deemed useful in a changelog.
   
   However, this can be confusing for anyone but the original author.
   
   We now write them as seen.
   
   Future iterations on this may consider adding more options
   to configure which sections should go into the changelog.
 - <csr-id-f90a34eb131a7a56f0c7d513b13ae85fab69dcac/> more prominent message if 'bat' wasn't found in PATH
 - <csr-id-66c33aa691d8de288fe01355398390cb72d46be6/> don't claim missing user edits if there are some
 - <csr-id-57316bb060571ab41ecdc99854091676b3bcd3f9/> don't mistake prefixed tags for versions
   Previously we would be too generious when accepting version tags, now
   we accept the prefixes 'v' and 'vers' and no prefix at all.
 - <csr-id-1829e1ce2998d1e625b589554db463f6fef1d0d2/> don't panic if there is a version requirement without version
 - <csr-id-bfb6b5fe9ff34c6882eb57c47099bb729f941b85/> Don't let dev-dependencies participate in traversal unless they have a version specified.
   This prevents safety bumps due to breaking changes in dev dependencies,
   which are generally ignored if there is no version specified.
 - <csr-id-d078b9a7f51c1665c67ab728a4180d1df7798c76/> nicer previews thanks to added newline
 - <csr-id-a53631473201cd7914da78c037cfbf2767c0c1fd/> Assume manifests cannot necessarily be read by `cargo_toml::Manifest` and fallback
 - <csr-id-510cd12ab2001f4c1ef6fadd669a3d1416039f92/> create github release only after tags were created and pushed
 - <csr-id-ffd0561cab8fcdb8b36bd5d88bed4054c14727b5/> strip `.git` suffix from repository paths when using it in urls
 - <csr-id-0b077c0e12cf2edb7e5a3b49116ccbeee1fa18ec/> remove extra '/' after https://github.com/ based URLs
 - <csr-id-d1d4961e840cc60dfbf8553e57fd2410d0238374/> push all available tags even if an error occurred
   That way, tags don't remain unpushed despite having been created
   successfully, just because one crate later in the publishing
   process fails.
 - <csr-id-7dec0c7969c1afb5203fcab036e23e972ff59dc7/> create GitHub release right after publishing succeeds
   This is more atomic and prevents loosing all github releases if one
   publish fails later on.
 - <csr-id-2d234c5794b4d730d1eb12d9921653213100f84d/> src/ dir of root packages is only used if there is multiple workspace members
 - <csr-id-7f3d3890264a0425c8e9221b16e9e048fef05fb5/> correct reporting of manifest changes
   Previously even unchanged crates would trigger workspace crates
   to be recorded for manifest changes.
   
   Now only crates that are to receive manifest changes will be triggering
   this.
 - <csr-id-0cbceee0f53aa10f06a1d23951be3c7e863f568d/> panic due to unexpected internal state
   When there was no change in the src/ directory of the top-level crate,
   the dependency resolution would not be able to auto-bump the version
   as no change occurred, but another part would usually detect a change
   as it wasn't confined to the top-level src/ directory.
   
   This could lead to a panic as an invariant wasn't upheld.
   
   This was fixed by letting both parts agree to use the src/ directory
   to determine changes of the top-level directory, and by making panics
   impossible while improving the messaging around this state should it
   still occur. The latter is rough, probably rare, but usable.
 - <csr-id-f63303d41db48a858252e9a395b82019dad8a883/> dependency resolution
   Previously the ordering of crate for release might not have been
   correct due to this issue that is now fixed.
   
   We need depth-first traversals and previously it would extend skipped
   dependencies, effectively putting them into their own ordering.
   
   Previously it would restore that ordering, but not anymore, causing
   this bug that was entirely unnecessary.
 - <csr-id-aa5fbb849d23826eda484900dc110b55b8a29c8b/> --no-changelog during smart-release is now actually working
   Previously the flag had no effect and changelogs would always be
   generated, possibly stopping the release as at least one of them
   needed manual work.
 - <csr-id-730bcf7b64b9efa4b6871752cc389363132ebec1/> pin version of clap to beta 5
   This assures we don't get broken automatically in future.
   Previously that wasn't possible as the dependency of `clap`,
   `clap-derive` was also using a beta verion and wasn't constrained,
   hence it would be updated and cause breaking changes with pinned
   versions of consumers of `clap`.
 - <csr-id-44aff86eaaf481c7b3079a12782db817e19848f0/> ! breaking changes cause intermediate (otherwise skipped) crates to be published.
   This assures that about-to-be-released crates that have breaking changes
   anywhere in their dependency graph will cause all crates leading up to,
   and including, a breaking change to be published as well.
 - <csr-id-d504df3b8ed0439226b659ca9bfdc55adc0506b2/> don't claim to change manifest version if it's the same one

### Bug Fixes (BREAKING)

 - <csr-id-39bcb1d1b2ea4cc9a75f97bc05db642b0a556f8d/> don't auto-publish stable crates by inverting `no-auto-publish-of-stable-crates` (to `auto-publish...`).
   It turned out that I was happily publishing stable crates even without user-facing changes
   as this was the default.
   This will now stop, and is fine if stable crates are not exposing API of unstable crates.
 - <csr-id-018bb93d253d1649c736d54609aa9b2c1f33296d/> don't auto-publish stable crates by inverting `no-auto-publish-of-stable-crates` (to `auto-publish...`).
   It turned out that I was happily publishing stable crates even without user-facing changes
   as this was the default.
   This will now stop, and is fine if stable crates are not exposing API of unstable crates.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 688 commits contributed to the release.
 - 154 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 36 unique issues were worked on: [#192](https://github.com/Byron/cargo-smart-release/issues/192), [#197](https://github.com/Byron/cargo-smart-release/issues/197), [#198](https://github.com/Byron/cargo-smart-release/issues/198), [#200](https://github.com/Byron/cargo-smart-release/issues/200), [#213](https://github.com/Byron/cargo-smart-release/issues/213), [#221](https://github.com/Byron/cargo-smart-release/issues/221), [#222](https://github.com/Byron/cargo-smart-release/issues/222), [#224](https://github.com/Byron/cargo-smart-release/issues/224), [#227](https://github.com/Byron/cargo-smart-release/issues/227), [#228](https://github.com/Byron/cargo-smart-release/issues/228), [#234](https://github.com/Byron/cargo-smart-release/issues/234), [#241](https://github.com/Byron/cargo-smart-release/issues/241), [#259](https://github.com/Byron/cargo-smart-release/issues/259), [#262](https://github.com/Byron/cargo-smart-release/issues/262), [#266](https://github.com/Byron/cargo-smart-release/issues/266), [#270](https://github.com/Byron/cargo-smart-release/issues/270), [#274](https://github.com/Byron/cargo-smart-release/issues/274), [#279](https://github.com/Byron/cargo-smart-release/issues/279), [#287](https://github.com/Byron/cargo-smart-release/issues/287), [#298](https://github.com/Byron/cargo-smart-release/issues/298), [#301](https://github.com/Byron/cargo-smart-release/issues/301), [#308](https://github.com/Byron/cargo-smart-release/issues/308), [#317](https://github.com/Byron/cargo-smart-release/issues/317), [#318](https://github.com/Byron/cargo-smart-release/issues/318), [#331](https://github.com/Byron/cargo-smart-release/issues/331), [#364](https://github.com/Byron/cargo-smart-release/issues/364), [#422](https://github.com/Byron/cargo-smart-release/issues/422), [#427](https://github.com/Byron/cargo-smart-release/issues/427), [#429](https://github.com/Byron/cargo-smart-release/issues/429), [#450](https://github.com/Byron/cargo-smart-release/issues/450), [#470](https://github.com/Byron/cargo-smart-release/issues/470), [#512](https://github.com/Byron/cargo-smart-release/issues/512), [#513](https://github.com/Byron/cargo-smart-release/issues/513), [#560](https://github.com/Byron/cargo-smart-release/issues/560), [#67](https://github.com/Byron/cargo-smart-release/issues/67), [#711](https://github.com/Byron/cargo-smart-release/issues/711)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#192](https://github.com/Byron/cargo-smart-release/issues/192)**
    - Assure the current package version is actually breaking ([`da676e5`](https://github.com/Byron/cargo-smart-release/commit/da676e5732eec0b633748eb640009a3c64f2e7f7))
    - Better verbosity handling when comparing to crates-index ([`ba1ba74`](https://github.com/Byron/cargo-smart-release/commit/ba1ba749b4fa9df368771b64eeed4625d4479dab))
    - Turn off safety bump with its own flag ([`f6f4414`](https://github.com/Byron/cargo-smart-release/commit/f6f441479cde179d92b095889797adeda3bce379))
    -  ([`307bf1b`](https://github.com/Byron/cargo-smart-release/commit/307bf1b8f11b9d7bc787d7cadf151a6310a15765))
 * **[#197](https://github.com/Byron/cargo-smart-release/issues/197)**
    - Improved safety bump log message ([`a1db0f1`](https://github.com/Byron/cargo-smart-release/commit/a1db0f1900e465b8bfb521a82691f2567bf357fd))
    - Commit message reveals safety bumps ([`4f9f1f9`](https://github.com/Byron/cargo-smart-release/commit/4f9f1f94d950b22d730eaa4d31d91cab7ca5890a))
    - Released crates only receive minor bumps… ([`9fd83a0`](https://github.com/Byron/cargo-smart-release/commit/9fd83a0e020cf4c45b1874290dbd7ebdba69d738))
    - Update changelog ([`151f10e`](https://github.com/Byron/cargo-smart-release/commit/151f10e895145241b91d29e1b9db723409798684))
    - Way more tests to nail current log output ([`8b96c9d`](https://github.com/Byron/cargo-smart-release/commit/8b96c9d7373fc31ba068aefad51c25a4cab3b87c))
    - Dependency upgrade works ([`de1155b`](https://github.com/Byron/cargo-smart-release/commit/de1155bb13e02fcc67eaee6cbdc3f9cf1334e537))
    - Calculate new version of dependent ([`9f2900c`](https://github.com/Byron/cargo-smart-release/commit/9f2900c62a182de085869263289eb06c6624c8a0))
    - Don't claim "conservative" updates for major version change ([`69473e9`](https://github.com/Byron/cargo-smart-release/commit/69473e9274034c4f7844fcba6859ae9e3ff84547))
    - Assure we can find non-sequential connections ([`cc08f34`](https://github.com/Byron/cargo-smart-release/commit/cc08f344687221476d87318b76ded2b0ded488de))
    - All logic to calculate dependent version bumps ([`654d646`](https://github.com/Byron/cargo-smart-release/commit/654d64604a455c43547c40bf003eaf19289376fb))
    - An algorithm to collect dependencies by 'growing' ([`6c067c7`](https://github.com/Byron/cargo-smart-release/commit/6c067c73fd73914d4a135f7072065b9123059d3f))
    - Foundation for bumping versions ([`eab816a`](https://github.com/Byron/cargo-smart-release/commit/eab816ae90b83940d549f110d00b69671f699fdb))
 * **[#198](https://github.com/Byron/cargo-smart-release/issues/198)**
    - Add yet another video ([`c63addb`](https://github.com/Byron/cargo-smart-release/commit/c63addbd591e5a0f0a1de718dbe759361de1c6dd))
    - Update Asciinema link in readme ([`e6a975f`](https://github.com/Byron/cargo-smart-release/commit/e6a975fef7c63135d4c90e5d07bc1284dd7c78bd))
    - Polish README a little more ([`76b7e1c`](https://github.com/Byron/cargo-smart-release/commit/76b7e1c20ce1d17bbcc6f0b2f5ae2f806e0d9966))
    - First version of updated README ([`1e160ba`](https://github.com/Byron/cargo-smart-release/commit/1e160ba1f46bf1dfd920fac36584645bd879f10d))
    - Finish changelog ([`4a40cc3`](https://github.com/Byron/cargo-smart-release/commit/4a40cc3c95444cc4aa5e648e5d11ff16398729b3))
    - Enforce an empty line after user sections ([`17117fd`](https://github.com/Byron/cargo-smart-release/commit/17117fdb68202d59399c62c7464a8f4c2fa0f5e8))
    - Respect release-level removed-id list even when inserting sections ([`35dbef7`](https://github.com/Byron/cargo-smart-release/commit/35dbef7e3787d46f6cf2fce6e6add1a47e5f2b9c))
    - Rename short name for `--execute` to `-e` from `-n` for consistency ([`5a3bbcb`](https://github.com/Byron/cargo-smart-release/commit/5a3bbcb4496f5bfcc624b6f9e967af0d63ac2ed3))
    - `--no-dependencies` now has `--only` as alias ([`6c613be`](https://github.com/Byron/cargo-smart-release/commit/6c613be2dd4e97d61a6f529e4bde0d7283a99033))
    - Write more of the smart-release changelog to learn --no-dependencies needs an alias ([`6b6556e`](https://github.com/Byron/cargo-smart-release/commit/6b6556e6e0a2936544ef089a09fbfa9d74712343))
    - Show how many more changelogs are going to be previewed… ([`1908726`](https://github.com/Byron/cargo-smart-release/commit/19087262effe65ae25675e400e3a56d7fc28c3fd))
    - Start writing the 0.4 changelog ([`90e4d36`](https://github.com/Byron/cargo-smart-release/commit/90e4d364dd9ce9bd7b9045940a1107be75b7a025))
    - Only use src/ directory for top-level crate change tracking… ([`4daa08c`](https://github.com/Byron/cargo-smart-release/commit/4daa08ce76c65216b693fa4e122a45867390f2e4))
    - Refactor ([`13d5e3f`](https://github.com/Byron/cargo-smart-release/commit/13d5e3fbd6640c0c60891f3299b4f299fc437e63))
    - Don't show previews in dry-run mode; provide help on how to fix this before release ([`7cf1ff6`](https://github.com/Byron/cargo-smart-release/commit/7cf1ff6bd5e64d5782e63169b6ce8ae641f01c03))
    - Fix naughty issue that isn't even reproducible… ([`682f7a9`](https://github.com/Byron/cargo-smart-release/commit/682f7a96372bfcb1553e49229d02c0964ddbcae6))
    - Correctly parse back single-word conventional messages ([`8b696e5`](https://github.com/Byron/cargo-smart-release/commit/8b696e5d58451e85fcd5508e97619dbc9eb61153))
    - Fix logic to determine if breaking changes are already handled by package version ([`1aa4a89`](https://github.com/Byron/cargo-smart-release/commit/1aa4a8938b0b7af975cfe9569818d28e25fc8bc0))
    - Greatly simplify dry-run preview for clear visuals ([`beadc46`](https://github.com/Byron/cargo-smart-release/commit/beadc46f897e348d144ac4f57a9b5b95c52ed397))
    - Update expectations for log messages ([`d41f483`](https://github.com/Byron/cargo-smart-release/commit/d41f4839b01fb2680611ec4f6037569adb52ab7b))
    - Use correct title for github release to match name of tag ([`9030e3e`](https://github.com/Byron/cargo-smart-release/commit/9030e3e9b99ebaf3bc256aba0506d18ed3d71852))
    - Fix logic to determine if links should be used… ([`9dddfcc`](https://github.com/Byron/cargo-smart-release/commit/9dddfccb7f8c54df003be86ca4fe02e15b073ba4))
    - Fix logic to determine if there are conventional headlines to fix - ignore non-breaking ([`58f9175`](https://github.com/Byron/cargo-smart-release/commit/58f9175cd041f424ac708dfa77309b537bfd8cb1))
    - Fix commit subject line when release would stop due changelog ([`292c38c`](https://github.com/Byron/cargo-smart-release/commit/292c38cb2681407a97b878600466da17570a155e))
    - Fix github release invocation ([`fc4dad0`](https://github.com/Byron/cargo-smart-release/commit/fc4dad03d8f8c2ae4714710a7ad3ae5ff2fa955d))
    - Less surprising location of the 'prepare release' message ([`deb2423`](https://github.com/Byron/cargo-smart-release/commit/deb2423c918a433a3d4a4f8233912523a09bf3f0))
    - Much better preview titles ([`cad444a`](https://github.com/Byron/cargo-smart-release/commit/cad444a9b5b677aa8e512a6b525445b7777bbea4))
    - Use --file-name flag to shorten displayed path ([`a56ee25`](https://github.com/Byron/cargo-smart-release/commit/a56ee252d5961c6c4b4b7bb24b7541440b8ddb8f))
    - Fix crate name and version for --version flag ([`6b41bb6`](https://github.com/Byron/cargo-smart-release/commit/6b41bb6b77dcbb94ac3a441850d40f54778de98d))
    - Clap second pass with arg headlines and better help messages ([`9fc4330`](https://github.com/Byron/cargo-smart-release/commit/9fc433026eaf27d4e0f2779e0fec1d02ea9779c6))
    - First pass of using clap instead of argh ([`f2418f4`](https://github.com/Byron/cargo-smart-release/commit/f2418f48a8d413ba0323920c93f177f526c5c028))
    - Use fmt::Display instead of io::Write when creating markdown docs… ([`1006017`](https://github.com/Byron/cargo-smart-release/commit/1006017ca1f6ff1145995e05b10b8aad2a2635cb))
    - Even cleaner release text, just with detail tags… ([`0aad53f`](https://github.com/Byron/cargo-smart-release/commit/0aad53f4a27ea22f4745735844ea16bad378e973))
    - Less verbose gh tool logging in dry-run mode ([`94c92a1`](https://github.com/Byron/cargo-smart-release/commit/94c92a17b19d0a19e139b39b92b8a9f6c1f010f5))
    - Try to do github releases for multi-crate releases, too ([`0bc59fb`](https://github.com/Byron/cargo-smart-release/commit/0bc59fb7f3a8bef5d89521323241bfff822adebe))
    - Improve commit message titles and simplify tag-name logic ([`54883bb`](https://github.com/Byron/cargo-smart-release/commit/54883bb0cfb8a1fa1e76d05fd2bc0dbc77bba85c))
    - Refactor ([`7884d3e`](https://github.com/Byron/cargo-smart-release/commit/7884d3ede0b3dcc2035fa5b3538fa5ac1c64ba82))
    - First sketch of running gh tool to create releases ([`162a72a`](https://github.com/Byron/cargo-smart-release/commit/162a72a62b13b061f0fd51015e75d00a4e9d9cd8))
    - Support for ssh->https github urls; more robustness in general ([`645e6e8`](https://github.com/Byron/cargo-smart-release/commit/645e6e807bbae527f03d2ffb1db062a4546699ca))
    - Add flag to allow disabling github releases ([`955d964`](https://github.com/Byron/cargo-smart-release/commit/955d964635704be949e130e549800847bb3165f8))
    - Sketch incorporation of github CLI support ([`5ae4aa1`](https://github.com/Byron/cargo-smart-release/commit/5ae4aa1cf39d9516e9822345b71a1222afa925cf))
    - :remote_url() is now optional ([`f2d2bfa`](https://github.com/Byron/cargo-smart-release/commit/f2d2bfad7c65c6d8a961bbbb0970f27de5a5c33e))
    - Inform about the difference between tag objects and references in verbose logs ([`f3e8eef`](https://github.com/Byron/cargo-smart-release/commit/f3e8eefd85b141d6583750e1ed1be235de6c722f))
    - Rename `ObjectAccessExt::tag(…)` to `*::tag_reference(…)`, add `easy::Object::try_to_tag()` ([`092fa28`](https://github.com/Byron/cargo-smart-release/commit/092fa282ad95b4b2493711883e2978442ebd38a4))
    - Add easy::ext::ObjectAccessExt::tag(…) to create tag objects ([`6b5f7b3`](https://github.com/Byron/cargo-smart-release/commit/6b5f7b39fbaf79b2c1746cb52fdcb26e8a147c1f))
    - Allow to skip writing section titles and html tags ([`efcb56c`](https://github.com/Byron/cargo-smart-release/commit/efcb56cd1bf3fed06540dec959264bb6cba08798))
    - Allow to turn off changelog links ([`a126d71`](https://github.com/Byron/cargo-smart-release/commit/a126d71aa57bd0a04ab1b3bdd0fec1b8454e58f5))
    - Pass release section text to function soon creating a tag object ([`37a053c`](https://github.com/Byron/cargo-smart-release/commit/37a053cfb8e4bec252203f8d4aeb657aab84fce6))
    - Precise change tracking for changelogs ([`a19dab8`](https://github.com/Byron/cargo-smart-release/commit/a19dab8fc95d9b8a4929e7e51b1d638ea4de5a07))
    - Fix stop-release-for-changelog logic and fix all affected changelogs ([`3897efd`](https://github.com/Byron/cargo-smart-release/commit/3897efdbf9167c370ea4f9272740b321f6bf94c3))
    - Less verbose changelog and smart-release sub-commands related to changelogs ([`523850f`](https://github.com/Byron/cargo-smart-release/commit/523850fd47389a9f82c11319872f097e6de047c7))
    - Adjust all changelogs to fulfil requirements for publishing ([`a3edb09`](https://github.com/Byron/cargo-smart-release/commit/a3edb09a88215bd78dc82e17790e8381248c8fb4))
    - Handle changelogs with upcoming version section if they were left for editing ([`bef8b28`](https://github.com/Byron/cargo-smart-release/commit/bef8b28f4a2b86b62a01e66c4bd4313128f773a9))
    - Refactor ([`9a806c6`](https://github.com/Byron/cargo-smart-release/commit/9a806c63e066e19fa7b44c6881ce5d5e295ba54e))
    - Automatically stop releases if changelogs are fully generated, and a flag to disable that ([`26828f5`](https://github.com/Byron/cargo-smart-release/commit/26828f5338b75e5587176af2b82c08b53e54dd80))
    - Check for changelog sections which are purely generated and warn about those ([`2653418`](https://github.com/Byron/cargo-smart-release/commit/26534186d1b1ecdc0aa4ba1bbdf323d56ee7243f))
    - See how it deals with major versions and auto-bumping in journey tests ([`6e3723b`](https://github.com/Byron/cargo-smart-release/commit/6e3723b4715ffbd1fa36ce9111ec4deff2fa9433))
    - More consistent log messages pertaining crate names ([`308be17`](https://github.com/Byron/cargo-smart-release/commit/308be176527fec5c004a5d15aab349cfc97030e8))
    - First working version of version auto-bumping based on changelog ([`74a170a`](https://github.com/Byron/cargo-smart-release/commit/74a170a253831f1e3e1d85793fb546a85603c304))
    - Issue links for category headlines ([`73476d6`](https://github.com/Byron/cargo-smart-release/commit/73476d620f50976621723251316ea23e4ac472cb))
    - Prepare for arrival of 'auto' bump mode ([`6e30606`](https://github.com/Byron/cargo-smart-release/commit/6e30606e0b0ae6f30d41c88a95882af9fbf8ca1f))
    - Fix git-url re-export to respect feature flags ([`c3d9c09`](https://github.com/Byron/cargo-smart-release/commit/c3d9c09518d11845d2bf1a3949d0abc009b076f4))
    - Deduplicate conventional message ids ([`2ddf4c5`](https://github.com/Byron/cargo-smart-release/commit/2ddf4c5319ea7057294be367b748823b819d9228))
    - Regenerate all changelogs to get links ([`9d654db`](https://github.com/Byron/cargo-smart-release/commit/9d654db859fbbedae238a4477497b7dd1325b414))
    - Link up github issue ids in statistics ([`d298590`](https://github.com/Byron/cargo-smart-release/commit/d29859005b0088f78ffad644688fb6adfeabdb5b))
    - Format links for commit ids ([`14251fa`](https://github.com/Byron/cargo-smart-release/commit/14251fa8d64bcc4a0885933fc99cbbe4cf382859))
    - Pass actual repository url down from commands ([`defe422`](https://github.com/Byron/cargo-smart-release/commit/defe4224cae0944e6572d021f972049d445e05fb))
    - Make `git_url::Url` available under `git_repository::Url` ([`1fbef37`](https://github.com/Byron/cargo-smart-release/commit/1fbef37ee6234f59fa694adc1baa1c828da9d836))
    - Foundation for rendering links if needed ([`6502ae0`](https://github.com/Byron/cargo-smart-release/commit/6502ae08309d43b666939f663aa7409b3b884e1b))
    - Rename title for "Fixed" to "Bug Fixes" ([`18f09df`](https://github.com/Byron/cargo-smart-release/commit/18f09df3fb4c14aa6b9602d201071521d3edcb21))
    - Mention actual issues that where worked on ([`ee11cec`](https://github.com/Byron/cargo-smart-release/commit/ee11cec8180a8814b0cd7ffb0b53a9d6f7eb1bfb))
    - Also parse 'style' if there are breaking changes ([`bcf5011`](https://github.com/Byron/cargo-smart-release/commit/bcf5011803dc2e9302f60720f041587fd112e28a))
    - Allow 'refactor' and 'other' in conventional messages if they have breaking changes ([`0e22b3e`](https://github.com/Byron/cargo-smart-release/commit/0e22b3ef72af592bf005c720a08f039f24b7cac3))
    - Support writing whole bodies in conventional messages… ([`c24b14f`](https://github.com/Byron/cargo-smart-release/commit/c24b14f4f52e4bce3cd045ec33eb1ddbf3c2c516))
    - Support for paragraphs in conventional items ([`3b5c963`](https://github.com/Byron/cargo-smart-release/commit/3b5c9632a045fe79db31cad6d88366ef972f0970))
    - Respect release-wide ignore list to allow removing entire conventional headlines ([`6d2ada8`](https://github.com/Byron/cargo-smart-release/commit/6d2ada8cffa0e2679a432694a9f6de8f15f3947b))
    - Only write headlines that we can parse back… ([`5a2afb4`](https://github.com/Byron/cargo-smart-release/commit/5a2afb43eb97aeb79bb2d959560b61dd94e51efa))
    - Handle all possible changelog headlines and add roundtrip tests ([`e6c302f`](https://github.com/Byron/cargo-smart-release/commit/e6c302f3d0813532bbd59ef689ed7354a93fff97))
    - First basic parsing of conventional user and generated messages ([`ea7e2bf`](https://github.com/Byron/cargo-smart-release/commit/ea7e2bf34cd3d4cd2a791d734b4f3911ad38dc9b))
    - Parsing of removed conventional messages from changelogs ([`101a3f7`](https://github.com/Byron/cargo-smart-release/commit/101a3f7634f543d08131862ce486eb1959878a4b))
    - First basic merging of conventional messages… ([`97b932e`](https://github.com/Byron/cargo-smart-release/commit/97b932e5daef81720ef141a19b467b920b1f340b))
    - Trivially emulate gits way of handling commit dates… ([`396234a`](https://github.com/Byron/cargo-smart-release/commit/396234a845219a305db937e71c2bbaaf3f57c098))
    - Also consider changes of changelogs themselves… ([`b456388`](https://github.com/Byron/cargo-smart-release/commit/b456388f66ee34e847ad703bb646c0e9c6165328))
    - Adjust date of upcoming version as well ([`b99ee1d`](https://github.com/Byron/cargo-smart-release/commit/b99ee1dcfec0be08ee3403cb59672b7e3b0a2014))
    - Assure git-conventional is treated like user generated content for statistics ([`0175068`](https://github.com/Byron/cargo-smart-release/commit/017506855b7e73662954c92e77db7052cfd28762))
    - Merge doesn't consider user generated sections, only the ones it would want to add ([`6ff0ac2`](https://github.com/Byron/cargo-smart-release/commit/6ff0ac27564b8ae4560e8f280a27aacacbe647d7))
    - Quick and dirty writing of conventional messages… ([`0dd950d`](https://github.com/Byron/cargo-smart-release/commit/0dd950d3927598f31d0c26c8f6fd906cef2d8abd))
    - Basic generation of git-conventional information ([`0c2414b`](https://github.com/Byron/cargo-smart-release/commit/0c2414b7c83193675bc2e8e52a5fe4a734604364))
    - Sketch out data structure for git-conventional segments ([`b99774c`](https://github.com/Byron/cargo-smart-release/commit/b99774c440ac87447a9d32f3ab6bb327c2d7c414))
    - Refactor ([`ff499f3`](https://github.com/Byron/cargo-smart-release/commit/ff499f30fc5e9656b198eee7321482eef01c83b7))
    - Smart-release with --changelog-without option… ([`ec34ead`](https://github.com/Byron/cargo-smart-release/commit/ec34ead31b57a8e7a0f117dc77f6ce4f74324ca5))
    - Changelog command learns the --without <section> option ([`afe692a`](https://github.com/Byron/cargo-smart-release/commit/afe692af21a53f02e9ecefca64ba2caad8b21791))
    - Easy removal of statistical sections, by just removing them… ([`fc780c9`](https://github.com/Byron/cargo-smart-release/commit/fc780c9a8fc3433f31cd4cf2f45e0cea1a778d17))
    - Rebuild all changelogs to assure properly ordered headlines ([`d0a454d`](https://github.com/Byron/cargo-smart-release/commit/d0a454d25df14ca1951a797a9417f39f89c7178c))
    - Reorder headlines according to version ordering… ([`d5015cb`](https://github.com/Byron/cargo-smart-release/commit/d5015cbdc57acd2dbafdc0d7971e1e8a05941d6a))
    - Sort all commits by time, descending… ([`4a6cf34`](https://github.com/Byron/cargo-smart-release/commit/4a6cf34ef60342a6c837d527a83b4f9d9dc3634f))
    - Greatly reduce changelog size now that the traversal fix is applied ([`8253e7d`](https://github.com/Byron/cargo-smart-release/commit/8253e7d384f2a801e863f86883cd3120681a4519))
    - Use most relevant parent tree for change comparison… ([`d2c1fc6`](https://github.com/Byron/cargo-smart-release/commit/d2c1fc60b952e2e76a251f96b38718230277fbde))
    - Use hashmap based lookup for trees… ([`4dbdfa3`](https://github.com/Byron/cargo-smart-release/commit/4dbdfa3a740faa11d04e4abefa4f40423d382fdd))
    - Refactor and improve path filtering to find relevant commits… ([`eee6131`](https://github.com/Byron/cargo-smart-release/commit/eee6131309691cdec3a8d0ff5b7e6ad1c8fef9e1))
    - The first headline level controls all the other ones ([`7ae9243`](https://github.com/Byron/cargo-smart-release/commit/7ae92431937911a9540f7e7309a7f72b64a0cbaf))
    - Adapt to git-hash refactor ([`c3452ee`](https://github.com/Byron/cargo-smart-release/commit/c3452ee4df64c3d6a1531d5eee69bfb4d02f63ec))
    - Fixup remaining changelogs… ([`b00e472`](https://github.com/Byron/cargo-smart-release/commit/b00e4723d7e02219e81bbffc9b930966b1d7840a))
    - Generate changelogs with details ([`68d08cc`](https://github.com/Byron/cargo-smart-release/commit/68d08ccc11104d3e2299feaf0af198c0e064308d))
    - Only use short hashes for logs, without detecting ambiguity for now ([`64b3a96`](https://github.com/Byron/cargo-smart-release/commit/64b3a965b9ea2f5bcfd415c29480b96de20e05fe))
    - Boost allowed package sizes… ([`b48a1a4`](https://github.com/Byron/cargo-smart-release/commit/b48a1a4d578786b1ca8ae5dac41768389ec4bac3))
    - Stable smart-release journey tests… ([`de28347`](https://github.com/Byron/cargo-smart-release/commit/de28347258de64e6832e9ec63fc1c4cab5575510))
    - Update all changelogs with details ([`beeb22b`](https://github.com/Byron/cargo-smart-release/commit/beeb22bc4a64caab28d1c0fbeca2020fed42e5d8))
    - Put commit details to the end of generated segments ([`47bc58b`](https://github.com/Byron/cargo-smart-release/commit/47bc58bb541e5f4f965ef7f2aee96bbd0b18a4fd))
    - Use message commit id instead of body… ([`f48af09`](https://github.com/Byron/cargo-smart-release/commit/f48af097e032e75d3039c03b44ed1cfb95631e36))
    - Fix md formatting on github ([`8d148e4`](https://github.com/Byron/cargo-smart-release/commit/8d148e4bd9394a09775cb73c0799bb32edc8b56b))
    - Create details headline based on log message ([`137f59e`](https://github.com/Byron/cargo-smart-release/commit/137f59e2219fcae82a96012aefde47ebd8c54760))
    - Add details behind a fold, but… ([`3afd35e`](https://github.com/Byron/cargo-smart-release/commit/3afd35e39389108affa3408c33daf2554fd6b74b))
    - Use the notion of 'changes after merge' only to drive previews… ([`6c3e583`](https://github.com/Byron/cargo-smart-release/commit/6c3e5834c7d2301268aba51c96fd78906b43e39b))
    - Update changelogs ([`37a72e2`](https://github.com/Byron/cargo-smart-release/commit/37a72e2f1005e29d6d0b84ca72b8bd2ff973700e))
    - Refactor ([`a603526`](https://github.com/Byron/cargo-smart-release/commit/a603526116602faf61a1caad2debeb7409e5cbe6))
    - Also provide a duration in days for preparing a release as part of statistics ([`7b27fea`](https://github.com/Byron/cargo-smart-release/commit/7b27feaafb14ac91781740505b1dc7562f60da82))
    - Fix tests ([`476210e`](https://github.com/Byron/cargo-smart-release/commit/476210e813f87f05ca2c9ec4e00aeddd3edd976e))
    - Refactor ([`0664b7e`](https://github.com/Byron/cargo-smart-release/commit/0664b7e18385669221a9b69b012234729332e36e))
    - More commit statistics ([`f8f8962`](https://github.com/Byron/cargo-smart-release/commit/f8f89624e0309af87aa5bee056683cf05e3e2a41))
    - Basic commit statistics with round-trip, more actual information to come ([`a841296`](https://github.com/Byron/cargo-smart-release/commit/a841296c86132916e031de8881c008ac10b146a0))
    - Refactor… ([`972b96f`](https://github.com/Byron/cargo-smart-release/commit/972b96fd4e41ff61d334a5dd9003ee706071268a))
    - More robust parsing of read-only sections ([`e7c9476`](https://github.com/Byron/cargo-smart-release/commit/e7c94762f8207252f2a139fb50bf4793eceacb25))
    - Treat clippy as generated statistical section… ([`86897f3`](https://github.com/Byron/cargo-smart-release/commit/86897f3ba14d2f0e7a6f09da102482b44b9a5ee5))
    - Add new section type and write it out: clippy ([`08f99d4`](https://github.com/Byron/cargo-smart-release/commit/08f99d45b98dafd1eccc3bdd0f6a0e15f946cbad))
    - Introduce notion of essential sections in a changelog… ([`df4c267`](https://github.com/Byron/cargo-smart-release/commit/df4c267402a75a68c79b889c8e6ea1c34d655e8f))
    - Preview changelog support for smart-release as well ([`d833552`](https://github.com/Byron/cargo-smart-release/commit/d8335527744369fec146ae65d66df7b87cbefbad))
    - Detect changes after merge; add flag for controlling changelog preview ([`f89a980`](https://github.com/Byron/cargo-smart-release/commit/f89a980fc7b428f25ad07410921d46a371cb8d4d))
    - A lot of logic to handle messaging around changelog generation and halting… ([`f43e848`](https://github.com/Byron/cargo-smart-release/commit/f43e84837d0ff932bbbdaf7568af17cb42cb624a))
    - Unconditional changelog creation in smart-release ([`1b0a1e9`](https://github.com/Byron/cargo-smart-release/commit/1b0a1e9cf09739726c4879df1e57bfa77640854a))
    - Rename --skip-* flags to --no-* for consistency ([`338eb4b`](https://github.com/Byron/cargo-smart-release/commit/338eb4b53da0c00d810a6a154d4ea7fe48536a5c))
    - Fix windows tests by transforming line endings ([`9e956d0`](https://github.com/Byron/cargo-smart-release/commit/9e956d044aa7b65b7bf754505b1f75a513734a60))
    - Avoid adding newlines which make writing unstable ([`97f3d26`](https://github.com/Byron/cargo-smart-release/commit/97f3d267bc97ad61a4343fd0c36dc0644cf4c617))
    - Fix section headline level ([`e411da2`](https://github.com/Byron/cargo-smart-release/commit/e411da27ef76afb1c916374b36c4dd844ba395d1))
    - Write first version of changlogs thus far… ([`92e8f8f`](https://github.com/Byron/cargo-smart-release/commit/92e8f8f622b14c12c5db4502e494cb091e6500e4))
    - Implement --write actually ([`2788115`](https://github.com/Byron/cargo-smart-release/commit/2788115dead74e1330298a900b028fa4775d1932))
    - Parse more user generated section content, adapt existing changelogs to work correctly ([`bbd1874`](https://github.com/Byron/cargo-smart-release/commit/bbd1874baa84761e971b65167c3b980d0f6b4f92))
    - A test case showing that headlines are currently ignored, and links too ([`ddee0d1`](https://github.com/Byron/cargo-smart-release/commit/ddee0d108842cb57e6cd2608c5d6eeb2685b3f8e))
    - Don't try to run tests in binaries that have none… ([`8aa83c4`](https://github.com/Byron/cargo-smart-release/commit/8aa83c44091c8818c990dc47e1c5afa87bcc2812))
    - It's already getting there, even though a few parts are completely missing ([`557ff8b`](https://github.com/Byron/cargo-smart-release/commit/557ff8bc9e7937ac96b400d2b793c8e6b82e31a0))
    - Only parse into 'unknown' catch all in special cases… ([`94973d3`](https://github.com/Byron/cargo-smart-release/commit/94973d382005853e2470e9b26343e1d1aa08fa8b))
    - First basic parsing of unknown parts as segments in sections ([`adf90e4`](https://github.com/Byron/cargo-smart-release/commit/adf90e40e33255a2ca6fc592a4348a9e97766df4))
    - Quick and dirty switch to getting access to a range of parsed input… ([`c84f34d`](https://github.com/Byron/cargo-smart-release/commit/c84f34df291bd88bd77e7febd279a2b7e46dcbbc))
    - Setup test for old method of parsing unknown text… ([`7588bce`](https://github.com/Byron/cargo-smart-release/commit/7588bce1cae4568f8abb150ff9df5c5cb81c53db))
    - Refactor tests: unit to integration level ([`3c44db4`](https://github.com/Byron/cargo-smart-release/commit/3c44db412cd1bdf3ca4952a420fcbddea85009aa))
    - Don't add a date to unreleased versions ([`4e93272`](https://github.com/Byron/cargo-smart-release/commit/4e93272d236835207e962ba26620556d3f2091d5))
    - Actually integrated generated changelog with existing ones… ([`e82df94`](https://github.com/Byron/cargo-smart-release/commit/e82df945e09a2a59bf9fa83244250b05a861f7a9))
    - Inform about 'bat's  absence ([`09d9cff`](https://github.com/Byron/cargo-smart-release/commit/09d9cff2d229db76bdfd1fabf16f3c7bd9ddecb7))
    - Rename --no-bat to --no-preview… ([`91e87e3`](https://github.com/Byron/cargo-smart-release/commit/91e87e38782475a0ccbeccc25e0d36601198369c))
    - Basic merging now works ([`463fab1`](https://github.com/Byron/cargo-smart-release/commit/463fab1455d3777be8692b31d8bf41e509dde19d))
    - Sketch for finding insertion points and merging sections ([`e560752`](https://github.com/Byron/cargo-smart-release/commit/e5607527294bdaee97e82e24a3335e1f74cd5c06))
    - Sketch merging logic… ([`fc10d0b`](https://github.com/Byron/cargo-smart-release/commit/fc10d0be87c953fcc2156c1ed74d8580cc165fdc))
    - Prepare test for basic merging… ([`abca51e`](https://github.com/Byron/cargo-smart-release/commit/abca51eece2faf919279bf986c3bf0cad0bd0d37))
    - Nicer 'thanks clippy' message ([`ba1b2fc`](https://github.com/Byron/cargo-smart-release/commit/ba1b2fc28a85108dafc156cf394d2ec8b39ec61d))
    - Show with simple example how the round-tripping works, neat ([`076f74d`](https://github.com/Byron/cargo-smart-release/commit/076f74d4bcb8b3ff9b7004a8ab5ea180585a2021))
    - Collect unknown text so things don't get lost entirely… ([`fe98d53`](https://github.com/Byron/cargo-smart-release/commit/fe98d531955db58d20c01b21911830d7a6f9240c))
    - Parse back what we write out, perfectly… ([`116977f`](https://github.com/Byron/cargo-smart-release/commit/116977fbe777fad856f4cc6eb3787a4fa1fe063e))
    - Fix journey test ([`a616aae`](https://github.com/Byron/cargo-smart-release/commit/a616aae277341c4730b00e78562f00e1c0ba17f1))
    - Write new changelogs with bat if available ([`4349069`](https://github.com/Byron/cargo-smart-release/commit/43490694c70347a43f56b63ed146ac9ce73b10b3))
    - Use `cargo diet` to reduce package size ([`3bd8454`](https://github.com/Byron/cargo-smart-release/commit/3bd8454aed005861bd7d731ac5b7d70c79fc9eb4))
    - Write markdown changelog to lock file ([`5da55b3`](https://github.com/Byron/cargo-smart-release/commit/5da55b3b949ec952b7b29ecf3ff72977fce7c4d7))
    - Refactor ([`2841bf1`](https://github.com/Byron/cargo-smart-release/commit/2841bf1e9503ddb8f3b87c2c399459b5bb82fb4d))
    - Basic serialization of ChangeLog ([`a1cf210`](https://github.com/Byron/cargo-smart-release/commit/a1cf2100c8c7318cafa7ed5a882c0ac261c5e1a6))
    - Support for generated headers ([`9bcd092`](https://github.com/Byron/cargo-smart-release/commit/9bcd09217edf99f2c6e9bb909ea07c25b054fc09))
    - Refactor ([`d35bdd9`](https://github.com/Byron/cargo-smart-release/commit/d35bdd953085b53ce9fae76423bc99c1acc51c7f))
    - Use 'to_*' when converting `easy::Object` to specific object kind ([`96fd2c1`](https://github.com/Byron/cargo-smart-release/commit/96fd2c1b2db3ba7959a97d212f64ac0ed7184132))
    - Transform history segments into changelog parts ([`c11a308`](https://github.com/Byron/cargo-smart-release/commit/c11a3089e7ae47a97cf33a46f162f18a6e1f6b63))
    - Layout structure for ChangeLog generation from history items ([`0810797`](https://github.com/Byron/cargo-smart-release/commit/081079745428794337fb436a5c0293882ec8e9a7))
    - More general commit history ([`4af3892`](https://github.com/Byron/cargo-smart-release/commit/4af38922993b0638536488e3a6abc2dafc80e47e))
    - Invert meaning of changelog's --dependencies flag… ([`3ee1527`](https://github.com/Byron/cargo-smart-release/commit/3ee15270aaf4642a07ff165762d4792749ddb424))
    - Rename --skip-dependencies to --no-dependencies… ([`03c57ae`](https://github.com/Byron/cargo-smart-release/commit/03c57aeb4586059a5065c56ff56044f24880af90))
    - Remove strong-weak typing for conventional type ([`82fcb9b`](https://github.com/Byron/cargo-smart-release/commit/82fcb9b69d0086d5455dab96cc95d9d467bcd25b))
    - Fix panic related to incorrect handling of character boundaries ([`07ed7bf`](https://github.com/Byron/cargo-smart-release/commit/07ed7bff681367032c886e46a5f2df836a2d4791))
    - Parse message fully (and own it) to allow markdown generation ([`a83bae7`](https://github.com/Byron/cargo-smart-release/commit/a83bae7e90bfbaa79e00179438b123fe6774da59))
    - Tests for conventional and unconventional description parsing ([`f8dcd2a`](https://github.com/Byron/cargo-smart-release/commit/f8dcd2a92928d112a7a3ab324a55e7bbbfb26e16))
    - Make use of fixed git-conventional ([`f9a2a12`](https://github.com/Byron/cargo-smart-release/commit/f9a2a12caacd43eec212fc6705c31b32cc7f304b))
    - Update git-conventional dependency ([`b6aef25`](https://github.com/Byron/cargo-smart-release/commit/b6aef2584d6ebd857f590835c07144bcb1c38f43))
    - First test and sketch for stripping of additional title values ([`c7f86d1`](https://github.com/Byron/cargo-smart-release/commit/c7f86d139ad0b615eb3a85df95c1d7e34364217b))
    - Basic message parsing, either conventional or not, without additions ([`106b7d7`](https://github.com/Byron/cargo-smart-release/commit/106b7d7a47fdc900c16204b05e3571b315c70c0f))
    - Sketch Message fields from which change logs can be built ([`d926850`](https://github.com/Byron/cargo-smart-release/commit/d926850f3dcc083c57bd81acee1e0062af5f50f0))
    - Fix build ([`5419e57`](https://github.com/Byron/cargo-smart-release/commit/5419e57a116b2560c2dca1326e00bfc8a4cd465f))
    - More message parsing tests, now with legit failure… ([`84f3ef3`](https://github.com/Byron/cargo-smart-release/commit/84f3ef38dbdcbf831ec3e59f25940782fe47e426))
    - Sketch data for parsed messages ([`e84f720`](https://github.com/Byron/cargo-smart-release/commit/e84f720be28fad0b63e74b2b25c725cf13e517f9))
    - Add git-conventional ([`17173c8`](https://github.com/Byron/cargo-smart-release/commit/17173c85ac4806e1ecdbd06bacc70b37d5647779))
    - Consider nom for custom parsing, but… ([`33a2d9f`](https://github.com/Byron/cargo-smart-release/commit/33a2d9f1ebbfb078649dad5ce07fae0aa1e857b4))
    - Refactor ([`8ab83d5`](https://github.com/Byron/cargo-smart-release/commit/8ab83d5a84f341c34cc059e27e45019390ec25d5))
    - Refactor ([`85bd418`](https://github.com/Byron/cargo-smart-release/commit/85bd41841d2721e857d605f42ac021bd37285ede))
    - Refactor ([`e746ee9`](https://github.com/Byron/cargo-smart-release/commit/e746ee9791ea086ff98a79efb522d177c64f4846))
    - A seemingly slow version of path lookup, but… ([`4f3808d`](https://github.com/Byron/cargo-smart-release/commit/4f3808d76a5a732970c5ef744a8375e7ab729357))
    - Fast filter by single-component path ([`489c035`](https://github.com/Byron/cargo-smart-release/commit/489c035a0366e98a0654b824df577ca6287ab964))
    - Prepare for fast lookup of paths ([`4d2e66b`](https://github.com/Byron/cargo-smart-release/commit/4d2e66b2593a1360f4405fd319a290d98e336987))
    - Configure caches with env vars using `apply_environment()` ([`6d4ba38`](https://github.com/Byron/cargo-smart-release/commit/6d4ba38f7c61860c068bb351c521fdaec4cdcbc6))
    - Refactor ([`0bb7f48`](https://github.com/Byron/cargo-smart-release/commit/0bb7f48f2b2c3cdaf4dadac5163f03b923489cf5))
    - Set package cache via RepositoryAccessExt ([`245c846`](https://github.com/Byron/cargo-smart-release/commit/245c84673a59e3569d0ab628a3051959c5c48b7a))
    - Object-cache to allow for a speed boost… ([`a6e2c58`](https://github.com/Byron/cargo-smart-release/commit/a6e2c581cfcf2e8395b7ead3e4d82a0a5a47d25f))
    - Actually build the segment vec, without pruning for now ([`4f0c9fb`](https://github.com/Byron/cargo-smart-release/commit/4f0c9fb70630abccd83a43beb338d504f62381c2))
    - Build commit history for later use in changelog generation ([`6891506`](https://github.com/Byron/cargo-smart-release/commit/689150655fe6b9bd83cb1d6a3c6c461f6bca38a3))
    - Sketch history acquisition ([`bfe6feb`](https://github.com/Byron/cargo-smart-release/commit/bfe6febccc3f3631566e6d24b32b5d92e73d49f9))
    - Add 'Head::peeled()' method ([`bfc2733`](https://github.com/Byron/cargo-smart-release/commit/bfc2733b28276e916eabfb2137c983fb07cc6eed))
    - Some performance logging ([`a2a750c`](https://github.com/Byron/cargo-smart-release/commit/a2a750c35df6faaeda50db2a61e7b5bfcc21f9be))
    - Build ref lookup table ([`5a4c43a`](https://github.com/Byron/cargo-smart-release/commit/5a4c43abb465c24a37ebc2697aa2d4675e3bef96))
    - Loose reference iteration with non-dir prefixes… ([`721c8f7`](https://github.com/Byron/cargo-smart-release/commit/721c8f7acdade982f617299077ef3958c379f55b))
    - Add 'references().all().peeled().'… ([`d847ff8`](https://github.com/Byron/cargo-smart-release/commit/d847ff8af4d2f82b6aeb8e02bbed17570fff760b))
    - Filter refs correctly, but… ([`b0f9356`](https://github.com/Byron/cargo-smart-release/commit/b0f93567029381414eea9e7752ebaa251da11502))
    - Find tag references by name… ([`aac3668`](https://github.com/Byron/cargo-smart-release/commit/aac36685b88075553241fc55e83982da4b4d7e82))
    - Improve changelog format ([`af8eeeb`](https://github.com/Byron/cargo-smart-release/commit/af8eeeb31f33484f84b8e37958802a4e24f58edc))
    - Sketch first step of info generation ([`571c98e`](https://github.com/Byron/cargo-smart-release/commit/571c98ed65eb162d6e4d227653bc41e2fa053eb1))
    - Changelog gets crates to work on ([`59e477a`](https://github.com/Byron/cargo-smart-release/commit/59e477a7dbe738594e00dcba53d685e099933adf))
    - Handle unborn heads ([`505c948`](https://github.com/Byron/cargo-smart-release/commit/505c94854a00fc135cd037ef848ed4eb8a6ade2e))
    - Fmt ([`12c4b49`](https://github.com/Byron/cargo-smart-release/commit/12c4b49d956999e9b9151a26bded7f103bea73e1))
    - Refactor ([`11b7ce6`](https://github.com/Byron/cargo-smart-release/commit/11b7ce634faeb68709cf9ee4b36aefed846e3cce))
    - Refactor ([`9d29488`](https://github.com/Byron/cargo-smart-release/commit/9d29488b124a37a252db60b9e84ad2911484b44a))
    - Refactor ([`df9204e`](https://github.com/Byron/cargo-smart-release/commit/df9204e93d735329a012bd2256945a63980a5590))
    - Initial test for changelog ([`62705de`](https://github.com/Byron/cargo-smart-release/commit/62705deb6f8119e30b92af83616292628551f050))
    - Very basic support for changelog command… ([`4dfdc89`](https://github.com/Byron/cargo-smart-release/commit/4dfdc898f5930bab82bbfb8c871aa1df40b03e88))
    - Add 'cargo changelog' sub-command binary ([`0d2cb64`](https://github.com/Byron/cargo-smart-release/commit/0d2cb64ac5d66328449793683fd9bb866b851f02))
    - Add changelog to most tests ([`ed56f26`](https://github.com/Byron/cargo-smart-release/commit/ed56f26e4f09432f4eca4c05638313edb25cd493))
 * **[#200](https://github.com/Byron/cargo-smart-release/issues/200)**
    - Parse issue numbers from description and clean it up ([`8dd1543`](https://github.com/Byron/cargo-smart-release/commit/8dd15433e06c82d656ef72af456b24bda71fa298))
 * **[#213](https://github.com/Byron/cargo-smart-release/issues/213)**
    - Fix version logic to handle breaking version updates correctly ([`7a2573d`](https://github.com/Byron/cargo-smart-release/commit/7a2573d9ae235e18b0489aa72cacb465fdf9d38a))
 * **[#221](https://github.com/Byron/cargo-smart-release/issues/221)**
    - Add tests which indicate the problem: safety-bump not applied to auto-publishes… ([`6586d44`](https://github.com/Byron/cargo-smart-release/commit/6586d449e6746a767c1465264dc2d3f72486c827))
    - --no-changelog-preview isn't needed anymore in dry-run mode ([`3d41b13`](https://github.com/Byron/cargo-smart-release/commit/3d41b1355a8b20e9e1e53784db0295233a59b226))
    - Refactor ([`0bd1b82`](https://github.com/Byron/cargo-smart-release/commit/0bd1b82cc0b0acf37333ed36490424a2139f5cf0))
    - Inform about safety bumps more explicitly ([`77f7877`](https://github.com/Byron/cargo-smart-release/commit/77f787711407e847102bfad66cb72e4044e180de))
    - Refactor ([`b172a93`](https://github.com/Byron/cargo-smart-release/commit/b172a938321be7cc5edb00f8a26a3012bbc0ae93))
 * **[#222](https://github.com/Byron/cargo-smart-release/issues/222)**
    - Fix smart-release journey test expecations ([`7a8c276`](https://github.com/Byron/cargo-smart-release/commit/7a8c276849277bc835e589454db4d6f22720301c))
    - Adjust changelog ([`71fef26`](https://github.com/Byron/cargo-smart-release/commit/71fef26e628883b4c6b815659e7c2d51b2d61b64))
    - Add `-d` short flag for `--allow-dirty` in `changelog` ([`eab7fbe`](https://github.com/Byron/cargo-smart-release/commit/eab7fbe00a99f70c1af048278464f6416543d485))
    - Adjust changelog ([`286a214`](https://github.com/Byron/cargo-smart-release/commit/286a21481831406cd422555ec15b4aeb0cd7ea05))
    - Push all available tags even if an error occurred ([`d1d4961`](https://github.com/Byron/cargo-smart-release/commit/d1d4961e840cc60dfbf8553e57fd2410d0238374))
    - Create GitHub release right after publishing succeeds ([`7dec0c7`](https://github.com/Byron/cargo-smart-release/commit/7dec0c7969c1afb5203fcab036e23e972ff59dc7))
    - Src/ dir of root packages is only used if there is multiple workspace members ([`2d234c5`](https://github.com/Byron/cargo-smart-release/commit/2d234c5794b4d730d1eb12d9921653213100f84d))
    - Adjust changelog for release, now definitely ([`96a2399`](https://github.com/Byron/cargo-smart-release/commit/96a239998b699ba2e02f74520aaa7325a566d6c2))
    - Correct reporting of manifest changes ([`7f3d389`](https://github.com/Byron/cargo-smart-release/commit/7f3d3890264a0425c8e9221b16e9e048fef05fb5))
    - Adjust changelog for smart-release release ([`c9d2fa3`](https://github.com/Byron/cargo-smart-release/commit/c9d2fa3406c17e2bcc6e52b40b42f3f23ef2a6f9))
    - `changelog` subcommand fails if there is nothing to do ([`9e9f3b6`](https://github.com/Byron/cargo-smart-release/commit/9e9f3b61b9c507915d7af9738cb6295e40671df8))
    - Panic due to unexpected internal state ([`0cbceee`](https://github.com/Byron/cargo-smart-release/commit/0cbceee0f53aa10f06a1d23951be3c7e863f568d))
    - Crude fix to avoid version related invariants to kick in during dependency resolution ([`2746efc`](https://github.com/Byron/cargo-smart-release/commit/2746efca082e91f590f08462e8541f93d98564cb))
    - Refactor ([`3037193`](https://github.com/Byron/cargo-smart-release/commit/30371933609ca37cc42f62ad619f3c6d8ae15301))
    - Refactor ([`8f4db64`](https://github.com/Byron/cargo-smart-release/commit/8f4db647922695245a92a5d500ea25ea108a5fd9))
    - Refactor ([`eceeb98`](https://github.com/Byron/cargo-smart-release/commit/eceeb9898a653f2428aca8c0510baea2d5c1f8fc))
    - Fix merging of dependency graphs for multiple crates ([`faf7bec`](https://github.com/Byron/cargo-smart-release/commit/faf7bec3b9132b71740f74d461f4f5ceea222992))
    - Revert "FAIL: try to assure that breaking changes are always published in correct order" ([`2054909`](https://github.com/Byron/cargo-smart-release/commit/2054909c2da663257b7252575d2b5fed5648e932))
    - Try to assure that breaking changes are always published in correct order ([`ee8a2b5`](https://github.com/Byron/cargo-smart-release/commit/ee8a2b5fa6dc2e586767d60d141f1abade64fe8f))
    - Update changelogs prior to release ([`cb3bfd7`](https://github.com/Byron/cargo-smart-release/commit/cb3bfd76b9d7ef2f3113cb3497ad7d974922bc0e))
    - Respect user selection when re-adding crates for manifest change ([`9e4c8cb`](https://github.com/Byron/cargo-smart-release/commit/9e4c8cb8bb2671277c5b2aa3d83abcf987914d5f))
    - Dependency resolution ([`f63303d`](https://github.com/Byron/cargo-smart-release/commit/f63303d41db48a858252e9a395b82019dad8a883))
    - --no-changelog during smart-release is now actually working ([`aa5fbb8`](https://github.com/Byron/cargo-smart-release/commit/aa5fbb849d23826eda484900dc110b55b8a29c8b))
    - Replace TODO with runtime logging ([`f3080ad`](https://github.com/Byron/cargo-smart-release/commit/f3080ad62640fe858f8b0bf3376edf5d05c07d76))
    - Unify presentation even more ([`a2e81fd`](https://github.com/Byron/cargo-smart-release/commit/a2e81fd125e7e418436d2de28630ae49b280c096))
    - Adjust expectations in smart-release journey tests ([`0ec2d8b`](https://github.com/Byron/cargo-smart-release/commit/0ec2d8b8f09f75e43480130f8bf6c6a727b81adb))
    - Group skipped items by skipped reason ([`8980b07`](https://github.com/Byron/cargo-smart-release/commit/8980b078329c637699aa6a1848705e3c7946c038))
    - Unify reporting style ([`b475c85`](https://github.com/Byron/cargo-smart-release/commit/b475c8506a0f5e82c4108c789fff7c02cfad4a50))
    - Fix reporting of skipped crates, consider adjustment ([`07e16bf`](https://github.com/Byron/cargo-smart-release/commit/07e16bfd25fb6eff6951d7ba40ce9d7262ab3cbe))
    - Abort if not a single provided crate would need publishing ([`87a42e0`](https://github.com/Byron/cargo-smart-release/commit/87a42e02a15ea67cc56489e5a5d49cbd19a7f8ad))
    - Improved reporting of skipped/refused crates; abort operation if there is nothing to publish ([`7fe354c`](https://github.com/Byron/cargo-smart-release/commit/7fe354c7fbc72892a44418abef6933531ff9dc3e))
    - Better reporting of crates that where refused to be published ([`5a459f8`](https://github.com/Byron/cargo-smart-release/commit/5a459f8a99cf0704180f27b08b3f15baf4e95d28))
    - 'changelog' subcommand change --dependencies to --no-dependencies ([`4d4cb23`](https://github.com/Byron/cargo-smart-release/commit/4d4cb23503e9bc0e8c9a0ac80217e23e87afbd0e))
    - Properly resolve breaking propagation through the graph ([`183e753`](https://github.com/Byron/cargo-smart-release/commit/183e7535ea12466dfd4eac4f4dbe88bdc8a89335))
    - Multi-round discovery of breaking changes from published packages ([`eeae98e`](https://github.com/Byron/cargo-smart-release/commit/eeae98ea54de20596cf4ce3d7ad6bd66b7d3ef1a))
    - Verify and partially fix journey tests ([`f7e98ff`](https://github.com/Byron/cargo-smart-release/commit/f7e98ff24a3ec7b35d17655526a1712d10a1346f))
    - Remove all now unused items ([`2eab138`](https://github.com/Byron/cargo-smart-release/commit/2eab1380ad9321a3b20a8416cd4a3612b92dd7cb))
    - Use Dependency in manifest editor ([`075f397`](https://github.com/Byron/cargo-smart-release/commit/075f397c5a51102048022c3cd77f32c7e266adcf))
    - Upgrade to clap 3 beta 5 ([`67071a9`](https://github.com/Byron/cargo-smart-release/commit/67071a992c0fdae3cb02eb1f358d170ac1bff3bf))
    - Show only changelogs that would be published ([`6296456`](https://github.com/Byron/cargo-smart-release/commit/6296456fb6e00d87190cf7b0836bb7a42627c60c))
    - Refactor ([`704e00d`](https://github.com/Byron/cargo-smart-release/commit/704e00d5b1a70c5ea7e0299e7c0d2cb592803d4b))
    - Fix reporting of skipped crates ([`c394c37`](https://github.com/Byron/cargo-smart-release/commit/c394c37df2172574467d7811cfa784ab2a36bf00))
    - Respect publish=false in cargo manifest ([`71e2dc9`](https://github.com/Byron/cargo-smart-release/commit/71e2dc9d7db48c7a6dd7eb07600a0b145a412b72))
    - More consistent reporting of what would be done ([`0dccfac`](https://github.com/Byron/cargo-smart-release/commit/0dccfac2ff53f7a01fca5a4e4185a34f3d0922ca))
    - Refactor ([`a938a2c`](https://github.com/Byron/cargo-smart-release/commit/a938a2cc5a8ae86e6576e309c51e9c09adc3589e))
    - Don't try to change crates that are already at the correct version ([`7444d69`](https://github.com/Byron/cargo-smart-release/commit/7444d6923946e51338b55c960b26fb0cd27947fb))
    - Keep ordering of causes for breaking changes when printing ([`0175cce`](https://github.com/Byron/cargo-smart-release/commit/0175cce79b46d94fb6027beb580dca7e7029482d))
    - Better safety bumps to be more concise ([`b6d1231`](https://github.com/Byron/cargo-smart-release/commit/b6d1231a0155ea55fe3fda8617dfd82874070371))
    - Perform safety bumps without forcing a publish ([`f12956d`](https://github.com/Byron/cargo-smart-release/commit/f12956dc716dffe1a0d13e7cc1a29be70ffc53f0))
    - Refactor ([`7560613`](https://github.com/Byron/cargo-smart-release/commit/7560613ef9c700c04214836e8304424aa47db295))
    - Inform about the crates seeing a mnifest update too; only show fully-skipped crates ([`05c7642`](https://github.com/Byron/cargo-smart-release/commit/05c7642e70e06537285ec9df60c6eece9cc44cca))
    - ! breaking changes cause intermediate (otherwise skipped) crates to be published. ([`44aff86`](https://github.com/Byron/cargo-smart-release/commit/44aff86eaaf481c7b3079a12782db817e19848f0))
    - Reverse-bumping for safety works, including publishing :) ([`878ac3d`](https://github.com/Byron/cargo-smart-release/commit/878ac3dba096c68fca56bd82c9580c0178d8abdb))
    - Track root-cause as well ([`97e2839`](https://github.com/Byron/cargo-smart-release/commit/97e283942059c6aa9de8b1d704c660aa3d1a892d))
    - Sketch backwards search for lifting crates to be published ([`bbe560d`](https://github.com/Byron/cargo-smart-release/commit/bbe560ddfddb47bdd192de3ee13e0c3f4fe7e425))
    - Realize that the search can't be 'flat' ([`c0d37c3`](https://github.com/Byron/cargo-smart-release/commit/c0d37c32240c8becec60b428b6d86f3795b12c20))
    - Start sketching backward traversal… ([`3b86300`](https://github.com/Byron/cargo-smart-release/commit/3b86300917213b7f09e48a4259202acf4bd1003b))
    - Sumarize manifest updates rather than spelling out each one ([`fc21b6e`](https://github.com/Byron/cargo-smart-release/commit/fc21b6ef6298c28ddb44b329494111220a89a735))
    - Update test expectations and formulate 'the algorithm' ([`7ec5891`](https://github.com/Byron/cargo-smart-release/commit/7ec589118d8b287a01e02eab09b843e26615d3ab))
    - Refactor ([`62c7b9c`](https://github.com/Byron/cargo-smart-release/commit/62c7b9c8275f82c53bc9de5ce4bb011589760aa0))
    - Assure changelog picks up safety bumps as well ([`6e1c5c4`](https://github.com/Byron/cargo-smart-release/commit/6e1c5c41aafea88e51271bf9e4ca6d3e6e9bfb23))
    - Collect crates for manifest updates ([`26d7027`](https://github.com/Byron/cargo-smart-release/commit/26d70270057fbd977e331d980d1353dccdb986ec))
    - Remove --no-multi-crate-release support entirely ([`79e37f5`](https://github.com/Byron/cargo-smart-release/commit/79e37f5a53802f4f2499d426856d0ea0d316599d))
    - Remove performance measurements ([`4d408b0`](https://github.com/Byron/cargo-smart-release/commit/4d408b040a00e2c78be54aa938bcee7776f8a2e7))
    - Refactor ([`a9ae43f`](https://github.com/Byron/cargo-smart-release/commit/a9ae43f01ef11095b4870520b6a153350ad34beb))
    - No newlines in gh traces ([`a0bfd8b`](https://github.com/Byron/cargo-smart-release/commit/a0bfd8b190ae5480b24e8a67d9fc605160a42e2b))
    - Refactor ([`4236253`](https://github.com/Byron/cargo-smart-release/commit/423625315d8c0305f6c70761cc705490437ff5eb))
    - Simplify use of 'verbose' flag by using log::trace! as well ([`c4fff25`](https://github.com/Byron/cargo-smart-release/commit/c4fff25a2b027b567f17a4d8e36664fd31d28fcc))
    - Refactor ([`05e076f`](https://github.com/Byron/cargo-smart-release/commit/05e076fe0d00ca6585edcd118dc4771638987aba))
    - Refactor ([`b226489`](https://github.com/Byron/cargo-smart-release/commit/b226489a4320874dac809903e9a559177c96300a))
    - Try to represent safety-bump versions ([`e4617b1`](https://github.com/Byron/cargo-smart-release/commit/e4617b1dd79b07da619f583e3f724a2c5611c975))
    - Refactor ([`04da4bb`](https://github.com/Byron/cargo-smart-release/commit/04da4bb008a0767eb0c4a024dd5f4c9ff95fdc23))
    - Simple version bumping logic based on what currently exists, with printout ([`a63e6d7`](https://github.com/Byron/cargo-smart-release/commit/a63e6d72e2eb5df7003dcbc5c4223e130e67b8d2))
    - Fully data-driven presentation of dependency tracking results… ([`01d1205`](https://github.com/Byron/cargo-smart-release/commit/01d1205ebb4db5b06a2329a9293977fa09f09c9f))
    - Refactor ([`168b822`](https://github.com/Byron/cargo-smart-release/commit/168b8220c3aeaaf12d6b40aa32d65a87579c1624))
    - Refactor ([`50816fc`](https://github.com/Byron/cargo-smart-release/commit/50816fc1164ed60a1f2d1be827991b47cbec9adf))
    - Refactor ([`b71865a`](https://github.com/Byron/cargo-smart-release/commit/b71865a0c2b24b23c7b3be3615ff9a6f9233a3bf))
    - Refactor ([`2a45313`](https://github.com/Byron/cargo-smart-release/commit/2a45313daafd4abe5167c3d26ef4fb17b2f1c8dc))
    - Remove `--only` alias and invert `--no-dependencies` to `--dependencies` ([`eaca66c`](https://github.com/Byron/cargo-smart-release/commit/eaca66c77fbdf2f0db7eaf5e03edb316b1a9ae95))
    - Keep track of skipped crate names for future use ([`7bfb217`](https://github.com/Byron/cargo-smart-release/commit/7bfb2179ad8fd6f8bdd4c73341d16c4c7853628a))
 * **[#224](https://github.com/Byron/cargo-smart-release/issues/224)**
    - Pin version of clap to beta 5 ([`730bcf7`](https://github.com/Byron/cargo-smart-release/commit/730bcf7b64b9efa4b6871752cc389363132ebec1))
 * **[#227](https://github.com/Byron/cargo-smart-release/issues/227)**
    - Create github release only after tags were created and pushed ([`510cd12`](https://github.com/Byron/cargo-smart-release/commit/510cd12ab2001f4c1ef6fadd669a3d1416039f92))
 * **[#228](https://github.com/Byron/cargo-smart-release/issues/228)**
    - 'changelog' understands '-e/--execute' as well. ([`0d20334`](https://github.com/Byron/cargo-smart-release/commit/0d203345367e28823f05ed1f8dbf26d47d04d6d6))
    - Nicer previews thanks to added newline ([`d078b9a`](https://github.com/Byron/cargo-smart-release/commit/d078b9a7f51c1665c67ab728a4180d1df7798c76))
    - Update changelog ([`c1ab1d3`](https://github.com/Byron/cargo-smart-release/commit/c1ab1d32358135eba7b62ee88a633678e3b9b04c))
    - Flexible tag parsing allows to find any version tags ([`be300a8`](https://github.com/Byron/cargo-smart-release/commit/be300a843f52d112bd97839fc8a207ecbc98e1b7))
    - Support for no prefixes in version headers ([`48a2dcc`](https://github.com/Byron/cargo-smart-release/commit/48a2dcca9de7ef243d8d7990ca0e7f7cdd3360a8))
    - Assume manifests cannot necessarily be read by `cargo_toml::Manifest` and fallback ([`a536314`](https://github.com/Byron/cargo-smart-release/commit/a53631473201cd7914da78c037cfbf2767c0c1fd))
 * **[#234](https://github.com/Byron/cargo-smart-release/issues/234)**
    - Auto-update crates-index if there is an indication ([`659d02a`](https://github.com/Byron/cargo-smart-release/commit/659d02a0661deb18db2271bc89c66c1609c83502))
    - Revert "FAIL: try to auto-udpate crates index with lifetime crazyness" ([`c3dfd10`](https://github.com/Byron/cargo-smart-release/commit/c3dfd10ab895430eb98ffd1c27e0a174ea8c1abe))
    - Try to auto-udpate crates index with lifetime crazyness ([`45f23a3`](https://github.com/Byron/cargo-smart-release/commit/45f23a30c715e3da15deeb5c4b808c99e3b80033))
 * **[#241](https://github.com/Byron/cargo-smart-release/issues/241)**
    - Improve usability of the pack-cache environment variable ([`93b624f`](https://github.com/Byron/cargo-smart-release/commit/93b624f0d736e92ffe22bcd5e28a35c0d332371e))
 * **[#259](https://github.com/Byron/cargo-smart-release/issues/259)**
    - Btree/hashmap free lookup of packs in store, keeping things more bundled ([`fd7545f`](https://github.com/Byron/cargo-smart-release/commit/fd7545f710b9a5dd32b8c670f0c13e204fef822f))
 * **[#262](https://github.com/Byron/cargo-smart-release/issues/262)**
    - Don't claim missing user edits if there are some ([`66c33aa`](https://github.com/Byron/cargo-smart-release/commit/66c33aa691d8de288fe01355398390cb72d46be6))
    - Don't mistake prefixed tags for versions ([`57316bb`](https://github.com/Byron/cargo-smart-release/commit/57316bb060571ab41ecdc99854091676b3bcd3f9))
    - Don't panic if there is a version requirement without version ([`1829e1c`](https://github.com/Byron/cargo-smart-release/commit/1829e1ce2998d1e625b589554db463f6fef1d0d2))
 * **[#266](https://github.com/Byron/cargo-smart-release/issues/266)**
    - Upgrade to crates-index 0.18 ([`afa0a50`](https://github.com/Byron/cargo-smart-release/commit/afa0a50d51f3e15186c5fac7ee817d169fc989cb))
    - Upgrade dependencies except for crates-index ([`9512d39`](https://github.com/Byron/cargo-smart-release/commit/9512d397ad943f21696c30fc8db0f3e92ed80ecf))
    - Revert "chore: upgrade all dependencies" ([`64088e8`](https://github.com/Byron/cargo-smart-release/commit/64088e8836e634f061f88550e2ede630f7e2bcde))
    - Upgrade all dependencies ([`38e3835`](https://github.com/Byron/cargo-smart-release/commit/38e3835b1b1fb90a7596b3cb08dd18c903a5acce))
    - Adjustments due to change in `git-repository` ([`d97c683`](https://github.com/Byron/cargo-smart-release/commit/d97c683a9f2863665247fcb13d87d0dafd5c965b))
    - Adjustments to match changes in `git-repository` ([`173e6ba`](https://github.com/Byron/cargo-smart-release/commit/173e6ba1a6a6e5d9481551c1cc68a114b54db512))
    - Adapt to changes in git-repository ([`5adbc3e`](https://github.com/Byron/cargo-smart-release/commit/5adbc3e2b33a3f622064a79df871c273f51a6bad))
 * **[#270](https://github.com/Byron/cargo-smart-release/issues/270)**
    - Use new built-in sorting to avoid more expensive sorting later on ([`5fecb9a`](https://github.com/Byron/cargo-smart-release/commit/5fecb9a4d6285d78a8711a621034be09cf4765ef))
 * **[#274](https://github.com/Byron/cargo-smart-release/issues/274)**
    - Remove easy::borrow::Error entirely; support for multiple objects per handle ([`9bc4049`](https://github.com/Byron/cargo-smart-release/commit/9bc4049fc3babeef1a4a5b3804dcb8eb4877000e))
    - Rename `easy::Object` to `OwnedObject`; remove `Ref` suffix from `ObjectRef` and `TreeRef` ([`d000e81`](https://github.com/Byron/cargo-smart-release/commit/d000e81d054fba5d75d5e6cdc338d295c6553725))
 * **[#279](https://github.com/Byron/cargo-smart-release/issues/279)**
    - Add missing docs ([`61b23b2`](https://github.com/Byron/cargo-smart-release/commit/61b23b2a82e901f3a407221ef354177530b95d23))
    - Adjust to git-hash changes ([`bd315a8`](https://github.com/Byron/cargo-smart-release/commit/bd315a8bcc77850f48e925a182e31ff62433873a))
 * **[#287](https://github.com/Byron/cargo-smart-release/issues/287)**
    - Smart-release now actually shows the time between releases ([`4c567cc`](https://github.com/Byron/cargo-smart-release/commit/4c567cc837721f721bad930921e9f347618498e9))
 * **[#298](https://github.com/Byron/cargo-smart-release/issues/298)**
    - Adjust to changes in git-traverse ([`2df4523`](https://github.com/Byron/cargo-smart-release/commit/2df4523652dadbcc44b1bb1de08cf96c9630657a))
    - Fix docs; consistent naming of 'repo' ([`38a95db`](https://github.com/Byron/cargo-smart-release/commit/38a95dbfbf663e8210cc21802dc5a7e114c078a3))
    - Adapt to changes in `git-repository' ([`1e4aaa1`](https://github.com/Byron/cargo-smart-release/commit/1e4aaa17adf5968ab44b15293951b5d26faf6114))
    - Clarify different repository types much better ([`feb40e0`](https://github.com/Byron/cargo-smart-release/commit/feb40e0f068c299ff3a5204caad0c3834bf0b9c9))
    - Upgrade parking_lot and cargo_toml ([`933b612`](https://github.com/Byron/cargo-smart-release/commit/933b6126f6159a08659b3d03fbf3cf0c6580d8a7))
 * **[#301](https://github.com/Byron/cargo-smart-release/issues/301)**
    - Adapt to changes in git-ref ([`b0810b6`](https://github.com/Byron/cargo-smart-release/commit/b0810b6f2f9bec736f962dec6ed2496704ea5ab5))
 * **[#308](https://github.com/Byron/cargo-smart-release/issues/308)**
    - More prominent message if 'bat' wasn't found in PATH ([`f90a34e`](https://github.com/Byron/cargo-smart-release/commit/f90a34eb131a7a56f0c7d513b13ae85fab69dcac))
 * **[#317](https://github.com/Byron/cargo-smart-release/issues/317)**
    - Fix broken link in README; clarify 'pre-release' ([`37f72c2`](https://github.com/Byron/cargo-smart-release/commit/37f72c29d8483ad84c8b98c2db065b9c0535ac27))
    - Fix broken link in README; clarify 'pre-release' ([`80a0aca`](https://github.com/Byron/cargo-smart-release/commit/80a0aca2027f9f33ff28e99daab739ffd5b02c5b))
    - Disambiguate usage of pre-release in stability guide ([`394f11f`](https://github.com/Byron/cargo-smart-release/commit/394f11f595aa12bb6fbb1d07ad9c8be60d9bdae4))
 * **[#318](https://github.com/Byron/cargo-smart-release/issues/318)**
    - Don't pass judgement on usefulness of certain kinds of git-conventional messages ([`ff55315`](https://github.com/Byron/cargo-smart-release/commit/ff553150c9a0195e0e561c3f75e96b5d4c27651a))
 * **[#331](https://github.com/Byron/cargo-smart-release/issues/331)**
    - Make fmt ([`027e966`](https://github.com/Byron/cargo-smart-release/commit/027e9665dbbf0a7a770e3448b76a1b46fb9eea6e))
 * **[#364](https://github.com/Byron/cargo-smart-release/issues/364)**
    - Prepare smart-release changelog ([`0b99c4a`](https://github.com/Byron/cargo-smart-release/commit/0b99c4aa04c0cb2bf688865496e5406fb6394991))
    - Dial down log level for unparseable items again ([`ba92ec4`](https://github.com/Byron/cargo-smart-release/commit/ba92ec487850d46ac93c6bf77e9ca2dfd7c6fe87))
    - Smart-release tries harder to wait for previously published packages ([`89f0dac`](https://github.com/Byron/cargo-smart-release/commit/89f0dac7de5a27f7995948f1e8cb73f66ca53200))
    - Consolidate naming of directories, use same convention as git2 ([`beca61a`](https://github.com/Byron/cargo-smart-release/commit/beca61a70d75dcbfa28542803653604e277faa28))
 * **[#422](https://github.com/Byron/cargo-smart-release/issues/422)**
    - Don't assume crates are non-breaking just because they are in the user selection. ([`b8b7345`](https://github.com/Byron/cargo-smart-release/commit/b8b7345e3a7c07746921313f8933fe658f284065))
 * **[#427](https://github.com/Byron/cargo-smart-release/issues/427)**
    - Implement :<path> parsing ([`63b0cc0`](https://github.com/Byron/cargo-smart-release/commit/63b0cc05d79190d8e15c47480c16bb61509000bd))
 * **[#429](https://github.com/Byron/cargo-smart-release/issues/429)**
    - Adjust changelogs prior to release ([`6dc0bfe`](https://github.com/Byron/cargo-smart-release/commit/6dc0bfe3800b7f05b656522559e62cfb2b4fc394))
    - Avoid running into the `default-members` trap with 'cargo publish'. ([`3be525a`](https://github.com/Byron/cargo-smart-release/commit/3be525aea817a0a1c217456c9f1fa9e4e39b847f))
 * **[#450](https://github.com/Byron/cargo-smart-release/issues/450)**
    - Adjust to deal with changes to git-repository ([`307de26`](https://github.com/Byron/cargo-smart-release/commit/307de267c4c99d9bf6ad25ca9947989af0528c03))
    - Adjust to changes in `git-url` ([`23641b8`](https://github.com/Byron/cargo-smart-release/commit/23641b8ece8270d1cebaa861bb1682cc8dfc4e1f))
    - Adapt to changes in `git-url` ([`04c63fd`](https://github.com/Byron/cargo-smart-release/commit/04c63fd2ae46f1505b7c854c5f063b8ad77271ee))
 * **[#470](https://github.com/Byron/cargo-smart-release/issues/470)**
    - Adapt to changes in `git-repository` ([`6cdc475`](https://github.com/Byron/cargo-smart-release/commit/6cdc47504492a75098fdf275b32b9ee949f9b102))
 * **[#512](https://github.com/Byron/cargo-smart-release/issues/512)**
    - Assure `git@github.com/user/repo` urls transform into https urls correctly. ([`7f422f2`](https://github.com/Byron/cargo-smart-release/commit/7f422f264d11586b80540e8d1014a8ebf7000ff4))
 * **[#513](https://github.com/Byron/cargo-smart-release/issues/513)**
    - Prepare for release ([`cec1ad7`](https://github.com/Byron/cargo-smart-release/commit/cec1ad722d895b5df29fba2e16a4077c5cf2bb89))
    - Improve the English skills of cargo-smart-release and fix a typo. ([`84180e2`](https://github.com/Byron/cargo-smart-release/commit/84180e2da6a5bd319ba6d1a46644159a44f2c889))
 * **[#560](https://github.com/Byron/cargo-smart-release/issues/560)**
    - `where -> were` typo fix. ([`2ac7c93`](https://github.com/Byron/cargo-smart-release/commit/2ac7c93d9698dc1227984c3427bf221b047982f8))
 * **[#67](https://github.com/Byron/cargo-smart-release/issues/67)**
    - Split data::output::count::objects into files ([`4452ea4`](https://github.com/Byron/cargo-smart-release/commit/4452ea424cf11a56e48ceb0e2764d31f7e2c242e))
 * **[#711](https://github.com/Byron/cargo-smart-release/issues/711)**
    - Assure we get the latest version of the `time` crate ([`27c22ca`](https://github.com/Byron/cargo-smart-release/commit/27c22ca046bd50c2a9df52ee284cc11e16e16dfd))
 * **Uncategorized**
    - Release cargo-smart-release v0.20.0 ([`a6197ea`](https://github.com/Byron/cargo-smart-release/commit/a6197eaa510e5cf2fa578335f4ff6fd256bccc5d))
    - Make sure smart-release can survive publishes ([`8ee9e94`](https://github.com/Byron/cargo-smart-release/commit/8ee9e94bcd22338941a5aca211e53936b7d10687))
    - Release gix-prompt v0.5.3, gix v0.49.1, cargo-smart-release v0.20.0 ([`6896a92`](https://github.com/Byron/cargo-smart-release/commit/6896a92c85f8410b7aabe18c232ad506e6da9e5f))
    - Adjust smart-release journey test to deal with the polarity changes of `--no-auto-publish-of-stable-crates` ([`12333a7`](https://github.com/Byron/cargo-smart-release/commit/12333a7b238588b488ba651237ba445157af7920))
    - Prepare changelogs prior to release ([`f8242e0`](https://github.com/Byron/cargo-smart-release/commit/f8242e0e21699172b426598385abba1edffce676))
    - Don't auto-publish stable crates by inverting `no-auto-publish-of-stable-crates` (to `auto-publish...`). ([`018bb93`](https://github.com/Byron/cargo-smart-release/commit/018bb93d253d1649c736d54609aa9b2c1f33296d))
    - Adjust package versions (by cargo-smart-release) ([`129d54b`](https://github.com/Byron/cargo-smart-release/commit/129d54b35775f5c114b038c2276eec38338c87b5))
    - Release gix-date v0.7.0, gix-trace v0.1.2, gix-actor v0.23.0, gix-commitgraph v0.17.1, gix-utils v0.1.4, gix-object v0.32.0, gix-ref v0.32.0, gix-config v0.25.0, gix-diff v0.32.0, gix-discover v0.21.0, gix-hashtable v0.2.3, gix-revwalk v0.3.0, gix-traverse v0.29.0, gix-index v0.20.0, gix-mailmap v0.15.0, gix-negotiate v0.4.0, gix-pack v0.39.0, gix-odb v0.49.0, gix-protocol v0.35.0, gix-revision v0.17.0, gix-refspec v0.13.0, gix-worktree v0.21.0, gix v0.48.0, safety bump 20 crates ([`031e18f`](https://github.com/Byron/cargo-smart-release/commit/031e18f42151ca97b3987a936d2164b39cdcfc82))
    - Adapt to changes in `gix-date` ([`91063ff`](https://github.com/Byron/cargo-smart-release/commit/91063ffe546c670b334bd3da39710d1a7059e028))
    - Release gix-date v0.6.0, gix-hash v0.11.3, gix-trace v0.1.1, gix-features v0.31.0, gix-actor v0.22.0, gix-path v0.8.2, gix-glob v0.9.0, gix-quote v0.4.5, gix-attributes v0.14.0, gix-chunk v0.4.3, gix-commitgraph v0.17.0, gix-config-value v0.12.2, gix-fs v0.3.0, gix-tempfile v7.0.0, gix-utils v0.1.3, gix-lock v7.0.0, gix-validate v0.7.6, gix-object v0.31.0, gix-ref v0.31.0, gix-sec v0.8.2, gix-config v0.24.0, gix-command v0.2.6, gix-prompt v0.5.2, gix-url v0.20.0, gix-credentials v0.16.0, gix-diff v0.31.0, gix-discover v0.20.0, gix-hashtable v0.2.2, gix-ignore v0.4.0, gix-bitmap v0.2.5, gix-revwalk v0.2.0, gix-traverse v0.28.0, gix-index v0.19.0, gix-mailmap v0.14.0, gix-negotiate v0.3.0, gix-pack v0.38.0, gix-odb v0.48.0, gix-packetline v0.16.3, gix-transport v0.33.0, gix-protocol v0.34.0, gix-revision v0.16.0, gix-refspec v0.12.0, gix-worktree v0.20.0, gix v0.47.0, gitoxide-core v0.29.0, gitoxide v0.27.0, safety bump 30 crates ([`8eeb543`](https://github.com/Byron/cargo-smart-release/commit/8eeb54319899f057d145cad71ab770a9c9ce99b2))
    - A build script to set a `gitoxide` version according to what's in git ([`79a1c82`](https://github.com/Byron/cargo-smart-release/commit/79a1c821cbdb397cd2728a59e6847d382f0404ff))
    - Add `clippy::redundant-closure-for-method-calls` lint ([`1cea980`](https://github.com/Byron/cargo-smart-release/commit/1cea98086a6676594c099162f72f7c910127f755))
    - Adapt to changes in `gix-actor` ([`3a3e66c`](https://github.com/Byron/cargo-smart-release/commit/3a3e66c74882ccecd033e08ea9b90a350f7ded46))
    - Adapt to changes in `gix-date` ([`43a8c58`](https://github.com/Byron/cargo-smart-release/commit/43a8c58ac3b52c5786afe1929863fedbc6ea992b))
    - Release gix-attributes v0.13.1, gix-diff v0.30.1, gix-revwalk v0.1.0, gix-traverse v0.27.0, gix-index v0.18.0, gix-revision v0.15.2, gix-negotiate v0.2.1, gix-pack v0.37.0, gix-odb v0.47.0, gix-protocol v0.33.2, gix-worktree v0.19.0, gix v0.46.0, safety bump 7 crates ([`a198d02`](https://github.com/Byron/cargo-smart-release/commit/a198d0241a43652ebe26a9b9671fe76874d44e89))
    - Adapt to changes in `gix` ([`5913ad3`](https://github.com/Byron/cargo-smart-release/commit/5913ad3fc9c69e5592875bd64bfa6a6bde7283fd))
    - Release gix-revision v0.15.1, gix v0.45.1 ([`85ea5dc`](https://github.com/Byron/cargo-smart-release/commit/85ea5dc933536cbe4e1162303dea6bcae6883542))
    - `just fmt` ([`cda1f31`](https://github.com/Byron/cargo-smart-release/commit/cda1f3159568b31ddd47c8d70b72e576351ab852))
    - Autofix map-or-unwrap clippy lint (and manual fix what was left) ([`5567e78`](https://github.com/Byron/cargo-smart-release/commit/5567e78026ef1d37c5aaa8b08d4d2638ee88dd55))
    - Merge branch 'main' into auto-clippy ([`d0d5f68`](https://github.com/Byron/cargo-smart-release/commit/d0d5f6831eec305da690af710c7ec73b3af94be5))
    - Auto-fix clippy to remove explicit iter looping ([`a821ae6`](https://github.com/Byron/cargo-smart-release/commit/a821ae6aa6ea06dae2cc5ad9adf18e543fdd944e))
    - Merge pull request #866 from nyurik/docs ([`b8aea81`](https://github.com/Byron/cargo-smart-release/commit/b8aea818ee54383d0bb5354c0c53cce167bf234b))
    - Include custom clippy settings ([`dd41c9e`](https://github.com/Byron/cargo-smart-release/commit/dd41c9ef5aa0165d07021f9c27a810a5e04c18f6))
    - Fix docs generation ([`147cfe1`](https://github.com/Byron/cargo-smart-release/commit/147cfe1e13dc60bfefe6b5299b8c01550f1577db))
    - Inline format args ([`b196db6`](https://github.com/Byron/cargo-smart-release/commit/b196db6d12a353fbf0b5d5dd61c98099f997866e))
    - Update precommit hooks ([`4d44cd7`](https://github.com/Byron/cargo-smart-release/commit/4d44cd7ca51f05fb06185677642d73c0ff0da079))
    - Release gix-commitgraph v0.15.0, gix-revision v0.14.0, gix-negotiate v0.1.0, safety bump 7 crates ([`4f40207`](https://github.com/Byron/cargo-smart-release/commit/4f40207f172e914e71bbe65a113e44f07434ca18))
    - Catch clippy config failures ([`2b6bb28`](https://github.com/Byron/cargo-smart-release/commit/2b6bb28cd18916a6244a2632a6abcba9362b9fd0))
    - Remove clippy lint past MSRV (needs 1.67) ([`80d4cdd`](https://github.com/Byron/cargo-smart-release/commit/80d4cdd688e88b897f384b770f9c13268ecb3793))
    - Adjust CLI docs to match the existing conventions (more) ([`1a61d5c`](https://github.com/Byron/cargo-smart-release/commit/1a61d5c22150aa3058b87c0eede20d3a9edcc668))
    - Cargo fmt ([`80e800e`](https://github.com/Byron/cargo-smart-release/commit/80e800e1f9beb693fd4995279dff89a5814f1edc))
    - Add --capitalize-commit option to capitalize commit message in cargo-smart-release ([`b9312a1`](https://github.com/Byron/cargo-smart-release/commit/b9312a1e405544517c6ef147f8e42142327a3bc9))
    - Add feature `vendored-openssl` which enables `crates-index/vendored-openssl` ([`25ef771`](https://github.com/Byron/cargo-smart-release/commit/25ef771957d6404531d3135184dae3079bbd92f9))
    - Release cargo-smart-release v0.19.0 ([`c306fbc`](https://github.com/Byron/cargo-smart-release/commit/c306fbc3c5f265a82248f6682cc0495585e8baf9))
    - Avoid panics in favor of error handling. That way more information can be provided which helps with a fix. ([`f5649e2`](https://github.com/Byron/cargo-smart-release/commit/f5649e2cbfc6c369354ea8aff02420d1748693ce))
    - Bump gix-path v0.8.0, safety bump 20 crates (gix set to 0.44.1 manually) ([`cb770d0`](https://github.com/Byron/cargo-smart-release/commit/cb770d0334fc3ffb24492f9900b3814c248315be))
    - Release cargo-smart-release v0.18.0 ([`ecb7dd4`](https://github.com/Byron/cargo-smart-release/commit/ecb7dd435ceb0e149e8f44252464fd0571bbdd38))
    - Thanks clippy ([`741f5f5`](https://github.com/Byron/cargo-smart-release/commit/741f5f59603a1c95b48ab28e10e1234bc08f873f))
    - Release gix-utils v0.1.0, gix-hash v0.11.0, gix-date v0.5.0, gix-features v0.29.0, gix-actor v0.20.0, gix-object v0.29.0, gix-archive v0.1.0, gix-fs v0.1.0, safety bump 25 crates ([`d3b78b3`](https://github.com/Byron/cargo-smart-release/commit/d3b78b3d59295717956bd676332d3df6a9232681))
    - Ban rebase merges ([`716170e`](https://github.com/Byron/cargo-smart-release/commit/716170eaa853ddf3032baa9b107eb3e44d6a4124))
    - Clarify why map_or is banned ([`96297f0`](https://github.com/Byron/cargo-smart-release/commit/96297f038d8d931bb9d5ba4dfcdced18d7c81061))
    - Ban for_each ([`60a8ec8`](https://github.com/Byron/cargo-smart-release/commit/60a8ec89e3f97baad0dbe097e03dc0cd30899e02))
    - Use new minimumReleaseAge field ([`afaba35`](https://github.com/Byron/cargo-smart-release/commit/afaba35d39c75d13138e2928cddeb0b93601cee3))
    - Match auto-generated style ([`d99db2e`](https://github.com/Byron/cargo-smart-release/commit/d99db2e632b25a8b020491c3e1d40bf2efd3472a))
    - Lower the MSRV churn for template ([`62401b8`](https://github.com/Byron/cargo-smart-release/commit/62401b8eafb71d8a928137f6f8dfc25340e39bbf))
    - Delay Renovate PRs until ready ([`2c4a7f5`](https://github.com/Byron/cargo-smart-release/commit/2c4a7f574f6fed6655e8b2f25916c22d7bf08ad1))
    - Update stabilidyDays to new syntax ([`563de12`](https://github.com/Byron/cargo-smart-release/commit/563de12d25e777e7244a73308090adcfb8b90014))
    - Match auto-generated style ([`4163ad7`](https://github.com/Byron/cargo-smart-release/commit/4163ad78c72df3a993bea6084fc05c6a2a44b9c2))
    - Fix Renovate regexes ([`f7b990b`](https://github.com/Byron/cargo-smart-release/commit/f7b990b803a4aa448e81a323df3a54e66d2d8df4))
    - Update to latest `bitflags` version. ([`8d838cf`](https://github.com/Byron/cargo-smart-release/commit/8d838cf152dfd68c539502fef8ee40081bb7455d))
    - Release gix-path v0.7.3, gix-config-value v0.10.2, gix-config v0.20.1, gix-discover v0.16.2, gix-index v0.15.1, gix-odb v0.43.1, gix-packetline v0.15.1, gix-protocol v0.30.2, gix-worktree v0.15.2, gix v0.43.1 ([`6d4d655`](https://github.com/Byron/cargo-smart-release/commit/6d4d655fad6c84e0d2020a582ec2065c8deb287d))
    - $HOME detection on windows ([`1a9f4b9`](https://github.com/Byron/cargo-smart-release/commit/1a9f4b9027acfcacbbc81dbdafe4b198db0e510f))
    - Include Cargo.lock ([`6c8df60`](https://github.com/Byron/cargo-smart-release/commit/6c8df60dc4015279cef303cab8f4760efb5ebea8))
    - Expand approved licenses ([`d1dd4ae`](https://github.com/Byron/cargo-smart-release/commit/d1dd4ae94067be2f3158fa46b0e78504705dfb26))
    - Remove rustfmt/clippy next jobs ([`037f379`](https://github.com/Byron/cargo-smart-release/commit/037f37906dad6d39f9fad371bc9a8ab76e8bd5c4))
    - Use workspace inheritance ([`afd6a45`](https://github.com/Byron/cargo-smart-release/commit/afd6a45ef73201bf5d5f3d4f0317f432b17c60d0))
    - Update release process ([`0838840`](https://github.com/Byron/cargo-smart-release/commit/083884043cc08394c6f91df81e6407721b2dc19e))
    - Don't set rustflags by default ([`2768727`](https://github.com/Byron/cargo-smart-release/commit/2768727452315929d88dda7d0686440d8e668736))
    - Quote strings in yaml ([`afeff23`](https://github.com/Byron/cargo-smart-release/commit/afeff23549a05cd0e5997f129e5d7a564ec41866))
    - Remove reference to travis ([`614b0a2`](https://github.com/Byron/cargo-smart-release/commit/614b0a2376b9ae6d95a1b768b93d06057f4b82d6))
    - Merge pull request #1 from epage/renovate/rust-1.x ([`29b981c`](https://github.com/Byron/cargo-smart-release/commit/29b981c5a67d00352a3b8dbaa9ba654bcffae4db))
    - Update msrv to v1.65.0 ([`fbaab42`](https://github.com/Byron/cargo-smart-release/commit/fbaab420b9e4e01e60522f87e89e2e0a28250c73))
    - Set changelog base ([`d6b4446`](https://github.com/Byron/cargo-smart-release/commit/d6b4446cd761d82313a0e69cf0da82ebfc4084cb))
    - First step ([`e7b7555`](https://github.com/Byron/cargo-smart-release/commit/e7b7555d1516d0b274e7269961fce9ec9b30bc98))
    - Merge pull request #798 from theduke/patch-1 ([`b88444a`](https://github.com/Byron/cargo-smart-release/commit/b88444ae65e9783b52f7a50ff35d6a3de1f437e0))
    - Fix minor typos ([`e866c4d`](https://github.com/Byron/cargo-smart-release/commit/e866c4d97dfbdc6dde52c750f7c3d34c0be43709))
    - Fix typo in README ([`47a9753`](https://github.com/Byron/cargo-smart-release/commit/47a9753fa7e36323ae4fb9e61d5d4c126bf646fd))
    - Release gix-tempfile v5.0.2, gix-validate v0.7.4, gix-config v0.20.0, gix-prompt v0.3.3, gix-diff v0.28.1, gix-discover v0.16.1, gix-pack v0.33.2, gix-transport v0.29.1, gix-protocol v0.30.1, gix-revision v0.12.1, gix-worktree v0.15.1, gix v0.43.0, safety bump gix v0.43.0 ([`3778bca`](https://github.com/Byron/cargo-smart-release/commit/3778bca3c293f2574d0c7d5e2756e495334bae1d))
    - Release gix-features v0.28.1, gix-tempfile v5.0.1, gix-ref v0.27.1, gix-pack v0.33.1, gix-packetline v0.15.0, gix-transport v0.29.0, gix-protocol v0.30.0, gix v0.42.0, safety bump 3 crates ([`ba0e2d5`](https://github.com/Byron/cargo-smart-release/commit/ba0e2d51d41cc76f4d56c906d6f09daa0f4b4849))
    - Release gix-tempfile v5.0.0, gix-lock v5.0.0, gix-ref v0.27.0, gix-config v0.19.0, gix-url v0.16.0, gix-credentials v0.12.0, gix-discover v0.16.0, gix-index v0.15.0, gix-pack v0.33.0, gix-odb v0.43.0, gix-transport v0.28.0, gix-protocol v0.29.0, gix-worktree v0.15.0, gix v0.41.0, safety bump 12 crates ([`8d9079e`](https://github.com/Byron/cargo-smart-release/commit/8d9079ec54357c9a7c13139b6e2fa4f5338d6555))
    - Release gix v0.40.0 ([`5d2303f`](https://github.com/Byron/cargo-smart-release/commit/5d2303f84cf79f032f59b7afa2e595438eb53413))
    - Release gix-features v0.28.0, gix-actor v0.19.0, gix-object v0.28.0, gix-diff v0.28.0, gix-traverse v0.24.0, gix-pack v0.32.0, safety bump 20 crates ([`90b22c9`](https://github.com/Byron/cargo-smart-release/commit/90b22c9a549ebf376f3a09ba93292757895a15cb))
    - Prepare for git-tempfile release ([`8478665`](https://github.com/Byron/cargo-smart-release/commit/8478665df530357e755ffc12357cfd64fb198bb1))
    - Make fmt ([`5c4e393`](https://github.com/Byron/cargo-smart-release/commit/5c4e393e8ac706ce8e8c411375bd7d63092327ed))
    - Release gix-object v0.26.4, gix-diff v0.26.3, gix v0.37.2, gix-commitgraph v0.13.1, gitoxide-core v0.25.0, gitoxide v0.23.0 ([`f48990d`](https://github.com/Byron/cargo-smart-release/commit/f48990df3e58c243069962a8e3746f1de57d14d6))
    - Release cargo-smart-release v0.17.0 ([`e5d4911`](https://github.com/Byron/cargo-smart-release/commit/e5d49114c8a1257b0bfe03cd99b17a255a4912e7))
    - Refactor ([`814b7e9`](https://github.com/Byron/cargo-smart-release/commit/814b7e9cd790a8e68866dfd6f2a2ec51eac83255))
    - Fix smart-release journey test expectation ([`bc4fe7d`](https://github.com/Byron/cargo-smart-release/commit/bc4fe7dd174ab62dbacab73f8a541288cece24d5))
    - Sentence case for changelog commit messages. ([`29917ba`](https://github.com/Byron/cargo-smart-release/commit/29917ba5ea4bd569204bb2fd9d9613d55e1e6655))
    - Release gix-config v0.16.3, gix v0.37.1 ([`c65eb46`](https://github.com/Byron/cargo-smart-release/commit/c65eb46c48740dd12d2e55a5392e36508dc03c7c))
    - Release gix-object v0.26.3, gix-diff v0.26.2, gix-traverse v0.22.2, gix v0.37.0, safety bump 3 crates ([`8786269`](https://github.com/Byron/cargo-smart-release/commit/8786269143f5d187e6dcf68015fcff98207e4eb7))
    - Release gix-date v0.4.3, gix-hash v0.10.3, gix-features v0.26.5, gix-actor v0.17.2, gix-glob v0.5.5, gix-path v0.7.2, gix-quote v0.4.2, gix-attributes v0.8.3, gix-validate v0.7.3, gix-object v0.26.2, gix-ref v0.24.1, gix-config v0.16.2, gix-command v0.2.4, gix-url v0.13.3, gix-credentials v0.9.2, gix-discover v0.13.1, gix-index v0.12.4, gix-mailmap v0.9.3, gix-pack v0.30.3, gix-packetline v0.14.3, gix-transport v0.25.6, gix-protocol v0.26.4, gix-revision v0.10.4, gix-refspec v0.7.3, gix-worktree v0.12.3, gix v0.36.1 ([`0f855bd`](https://github.com/Byron/cargo-smart-release/commit/0f855bd9fce7f2439cc01838d8b114fa62741e58))
    - Release cargo-smart-release v0.16.1 ([`b978fa7`](https://github.com/Byron/cargo-smart-release/commit/b978fa74e1d6e936a854e284039b20453f45ebe9))
    - Enable local-offset support in the `time` crate and opt-in to it. ([`a327b8a`](https://github.com/Byron/cargo-smart-release/commit/a327b8aec3d00c568630121c1af267accf833c5d))
    - Release cargo-smart-release v0.16.0 ([`c5c3cd0`](https://github.com/Byron/cargo-smart-release/commit/c5c3cd03fa895ce9302fa2136ba334fa067fa634))
    - Merge branch 'rename-crates' into inform-about-gix-rename ([`08458d5`](https://github.com/Byron/cargo-smart-release/commit/08458d5d949e6466969de35497b82e567a677d42))
    - Release git-date v0.4.3, git-hash v0.10.3, git-features v0.26.5, git-actor v0.17.2, git-glob v0.5.4, git-path v0.7.2, git-quote v0.4.2, git-attributes v0.8.3, git-bitmap v0.2.2, git-chunk v0.4.2, git-command v0.2.4, git-commitgraph v0.13.1, git-config-value v0.10.2, git-tempfile v3.0.3, git-lock v3.0.3, git-validate v0.7.3, git-object v0.26.2, git-ref v0.24.1, git-sec v0.6.3, git-config v0.16.2, git-prompt v0.3.3, git-url v0.13.3, git-credentials v0.9.2, git-diff v0.26.2, git-discover v0.13.1, git-fetchhead v0.1.0, git-filter v0.1.0, git-hashtable v0.1.2, git-traverse v0.22.2, git-index v0.12.4, git-lfs v0.1.0, git-mailmap v0.9.3, git-note v0.1.0, git-pack v0.31.0, git-odb v0.41.0, git-packetline v0.14.3, git-pathspec v0.1.0, git-transport v0.25.5, git-protocol v0.26.4, git-rebase v0.1.0, git-revision v0.10.4, git-refspec v0.7.3, git-sequencer v0.1.0, git-submodule v0.1.0, git-tix v0.1.0, git-tui v0.1.0, git-worktree v0.12.3, safety bump 2 crates ([`dcf6f0b`](https://github.com/Byron/cargo-smart-release/commit/dcf6f0bd807abdd20e151b1627f35ff4523d14b0))
    - Assure we can track dependencies correctly. ([`3ebf12e`](https://github.com/Byron/cargo-smart-release/commit/3ebf12e76678db9c1c6ebd964f6ef4e6811ba111))
    - Rename `git-testtools` to `gix-testtools` ([`641bf8e`](https://github.com/Byron/cargo-smart-release/commit/641bf8e75669e097be8ed3454e1a74c96e47a97c))
    - Adjust to renaming of `git-pack` to `gix-pack` ([`99ab05c`](https://github.com/Byron/cargo-smart-release/commit/99ab05cc26e0c89c0f46f65b43355059615d75c3))
    - Adjust to renaming of `git-odb` to `gix-odb` ([`e4ba81c`](https://github.com/Byron/cargo-smart-release/commit/e4ba81cee849df8d90c965f6ec3215b0c57a9a2a))
    - Adjust to renaming of `git-index` to `gix-index` ([`5f3314f`](https://github.com/Byron/cargo-smart-release/commit/5f3314f0158830c4c37587719b57473b699f7677))
    - Adjust to renaming of `git-diff` to `gix-diff` ([`72027a8`](https://github.com/Byron/cargo-smart-release/commit/72027a8e5a8266edf51c3cd72c2be7cbb5fac29a))
    - Adjust to renaming of `git-commitgraph` to `gix-commitgraph` ([`08071de`](https://github.com/Byron/cargo-smart-release/commit/08071de690846eb17f1f72a201ca5f6ba61634b3))
    - Adjust to renaming of `git-mailmap` to `gix-mailmap` ([`3b22f5a`](https://github.com/Byron/cargo-smart-release/commit/3b22f5a731c09bebee32974802f96c12be0dfc8c))
    - Adjust to renaming of `git-discover` to `gix-discover` ([`aeaf51c`](https://github.com/Byron/cargo-smart-release/commit/aeaf51cffb73b880e8f9a93ee009025cb42bfef3))
    - Adjust to renaming of `git-chunk` to `gix-chunk` ([`d21d294`](https://github.com/Byron/cargo-smart-release/commit/d21d294cadd556f9e871dbda551759a922640b1d))
    - Adjust to renaming of `git-bitmap` to `gix-bitmap` ([`283baa2`](https://github.com/Byron/cargo-smart-release/commit/283baa2196ba11c2da2d801feb3a00946c2c9d2f))
    - Adjust to renaming for `git-protocol` to `gix-protocol` ([`a7670d4`](https://github.com/Byron/cargo-smart-release/commit/a7670d4148ab1ff944ca4e7ed6b4f80243f185fa))
    - Adjust to renaming of `git-refspec` to `gix-refspec` ([`56a7071`](https://github.com/Byron/cargo-smart-release/commit/56a707160040eb43486c1376900ea8a9cd356f07))
    - Adjust to renaming of `git-revision` to `gix-revision` ([`079a5f6`](https://github.com/Byron/cargo-smart-release/commit/079a5f6caf65dd0c79daa32f110e2dd45eb91b2f))
    - Adjust to renaming of `git-transport` to `gix-transport` ([`f4034a0`](https://github.com/Byron/cargo-smart-release/commit/f4034a03a059c4818b4242df2877abccb95c4749))
    - Adjust to renaming of `git-credentials` to `gix-credentials` ([`80488cb`](https://github.com/Byron/cargo-smart-release/commit/80488cbff8a28b76b815403e233e137dc20c965d))
    - Adjust to renaming of `git-prompt` to `gix-prompt` ([`aec2b69`](https://github.com/Byron/cargo-smart-release/commit/aec2b699f835ef0ef462bf80e3c79c53ad1eeaa2))
    - Adjust to renaming of `git-command` to `gix-command` ([`cb6dc1f`](https://github.com/Byron/cargo-smart-release/commit/cb6dc1f153b9f7de6e42810bc2e8ad89b54f31f6))
    - Adjust to renaming of `git-packetline` to `gix-packetline` ([`df2589f`](https://github.com/Byron/cargo-smart-release/commit/df2589ffcff47d26550c909b35895c02b17ee040))
    - Adjust to renaming of `git-worktree` to `gix-worktree` ([`a9e5b37`](https://github.com/Byron/cargo-smart-release/commit/a9e5b373b03d541cca4b8fe100320d7d9623fee0))
    - Adjust to renamining of `git-hashtable` to `gix-hashtable` ([`eb7d90c`](https://github.com/Byron/cargo-smart-release/commit/eb7d90c8a62ded4a4fa9e0c5964856a0c0bc21e2))
    - Adjust to renamining of `git-worktree` to `gix-worktree` ([`e07024e`](https://github.com/Byron/cargo-smart-release/commit/e07024e01d2a5de3e5e75ba7e4bdc8d2a02f59c5))
    - Adjust to renaming of `git-url` to `gix-url` ([`b21fba4`](https://github.com/Byron/cargo-smart-release/commit/b21fba47278245992f866512bca7713c3fb0e892))
    - Adjust to renaming of `git-date` to `gix-date` ([`3c2621d`](https://github.com/Byron/cargo-smart-release/commit/3c2621d16f8b33a673c481544e1de43c228d29fd))
    - Adjust to renamining of `git-attributes` to `gix-attributes` ([`0d70a58`](https://github.com/Byron/cargo-smart-release/commit/0d70a5813faeae08f30e585dee3fb9046cdaa959))
    - Adjust to renaminig of `git-quote` to `gix-quote` ([`56e95e9`](https://github.com/Byron/cargo-smart-release/commit/56e95e9bbf012ede056c72d20fc6392b44d1ce72))
    - Adjust to renaming of `git-config` to `gix-config` ([`cc5c1ad`](https://github.com/Byron/cargo-smart-release/commit/cc5c1adc91a4baa7d4007c40b59fdf0b8d93186b))
    - Adjust to renaming of `git-ref` to `gix-ref` ([`27a096d`](https://github.com/Byron/cargo-smart-release/commit/27a096d66cca3c305391326aef6648b3f610dd71))
    - Adjust to renaming of `git-lock` to `gix-lock` ([`a7dca5f`](https://github.com/Byron/cargo-smart-release/commit/a7dca5fadcd874556d9e809dc36b7ae61d6fa16d))
    - Adjust to renaming of `git-tempfile` to `gix-tempfile` ([`d0f4723`](https://github.com/Byron/cargo-smart-release/commit/d0f4723d70e544fc5047b8b8c16a8845ee19fed9))
    - Adjust to renaming of `git-object` to `gix-object` ([`b69bfca`](https://github.com/Byron/cargo-smart-release/commit/b69bfcad574883ed25e5fb96aeeb66a1c2d92493))
    - Adjust to renaming of `git-actor` to `gix-actor` ([`c72ef7a`](https://github.com/Byron/cargo-smart-release/commit/c72ef7ae574cf86b19994cd8faabfee7a7818b85))
    - Adjust to renaming of `git-validate` to `gix-validate` ([`3afc327`](https://github.com/Byron/cargo-smart-release/commit/3afc32743bba66bcdf6c7e758d0787d9cf1b3a2c))
    - Adjust to renaming of `git-hash` to `gix-hash` ([`dbdbba9`](https://github.com/Byron/cargo-smart-release/commit/dbdbba9b989f71c5bba0f99310e5789621b8c9ca))
    - Adjust to renaming of `git-features` to `gix-features` ([`a3622fb`](https://github.com/Byron/cargo-smart-release/commit/a3622fb6f2fa3db0063b33bfcee1c25f22886992))
    - Adjust to renaming of `git-glob` to `gix-glob` ([`e432a5c`](https://github.com/Byron/cargo-smart-release/commit/e432a5c214f3ddab6edda64e4e92c7b08f8c20df))
    - Adjust to renaming of `git-sec` to `gix-sec` ([`e68b97a`](https://github.com/Byron/cargo-smart-release/commit/e68b97af78a23b98f32efbee4e7ed2f2261054d9))
    - Adapt to renaming of `git-path` to `gix-path` ([`5f6e77a`](https://github.com/Byron/cargo-smart-release/commit/5f6e77a746298063f6422ba54f691c30a21e739f))
    - Adjust to rename of `git-config-value` to `gix-config-value` ([`70b2293`](https://github.com/Byron/cargo-smart-release/commit/70b22933d7052d2f210523a38ee1af740782a135))
    - Show more debugging information if unreachable code is reached. ([`c6d0534`](https://github.com/Byron/cargo-smart-release/commit/c6d05342f9ae97738ae4621f28e855515464892a))
    - Rename `git-repository` to `gix` ([`e7476fd`](https://github.com/Byron/cargo-smart-release/commit/e7476fdd2d740cb351e70afcaa3f80d314c804ea))
    - Release git-repository v0.35.0, safety bump 3 crates ([`d0b9c38`](https://github.com/Byron/cargo-smart-release/commit/d0b9c38688e01a556bf8329f8acbe9732bc1deb9))
    - Rename tracking for crates in the crate-root. ([`84aeb51`](https://github.com/Byron/cargo-smart-release/commit/84aeb51fd6099d8cfa6ec4db08b78e06728d0664))
    - Release cargo-smart-release v0.15.0 ([`3de6d7c`](https://github.com/Byron/cargo-smart-release/commit/3de6d7ca551ac7fdcb3f3ec4e284489b5df5c705))
    - Handle worktree members which are also used as dependencies from crates.io. ([`ce96558`](https://github.com/Byron/cargo-smart-release/commit/ce965582a2e914190a30982559cfa4e5a98a578a))
    - Release git-date v0.4.2, git-hash v0.10.2, git-features v0.26.2, git-actor v0.17.1, git-glob v0.5.3, git-path v0.7.1, git-quote v0.4.1, git-attributes v0.8.2, git-config-value v0.10.1, git-tempfile v3.0.2, git-lock v3.0.2, git-validate v0.7.2, git-object v0.26.1, git-ref v0.24.0, git-sec v0.6.2, git-config v0.16.0, git-command v0.2.3, git-prompt v0.3.2, git-url v0.13.2, git-credentials v0.9.1, git-diff v0.26.1, git-discover v0.13.0, git-hashtable v0.1.1, git-bitmap v0.2.1, git-traverse v0.22.1, git-index v0.12.3, git-mailmap v0.9.2, git-chunk v0.4.1, git-pack v0.30.2, git-odb v0.40.2, git-packetline v0.14.2, git-transport v0.25.4, git-protocol v0.26.3, git-revision v0.10.2, git-refspec v0.7.2, git-worktree v0.12.2, git-repository v0.34.0, safety bump 3 crates ([`e1b1f80`](https://github.com/Byron/cargo-smart-release/commit/e1b1f80a478adcc30c9a9c5fcad5965a00b1474a))
    - Merge branch 'Lioness100/main' ([`a18e9a7`](https://github.com/Byron/cargo-smart-release/commit/a18e9a783145c74c0124b545fe0a5d3af573c69b))
    - Fix typos ([`09d6986`](https://github.com/Byron/cargo-smart-release/commit/09d6986a15f67e48a0184cbca927dc05c51be604))
    - Upgrade toml-edit and `cargo-toml` ([`7f13b7a`](https://github.com/Byron/cargo-smart-release/commit/7f13b7afabbbba2f254df7d1c0eec41523a77fd1))
    - Break cyclical dev dependencies ([`2937ba0`](https://github.com/Byron/cargo-smart-release/commit/2937ba0e9f422ae92d8d5ebd050e442ef784fd42))
    - Upgrade to clap 4.1 ([`19b6df4`](https://github.com/Byron/cargo-smart-release/commit/19b6df4e9d25d502ec4e21cb950a186f8b4300ce))
    - Release git-date v0.4.1, git-features v0.26.1, git-glob v0.5.2, git-attributes v0.8.1, git-tempfile v3.0.1, git-ref v0.23.1, git-sec v0.6.1, git-config v0.15.1, git-prompt v0.3.1, git-url v0.13.1, git-discover v0.12.1, git-index v0.12.2, git-mailmap v0.9.1, git-pack v0.30.1, git-odb v0.40.1, git-transport v0.25.3, git-protocol v0.26.2, git-revision v0.10.1, git-refspec v0.7.1, git-worktree v0.12.1, git-repository v0.33.0 ([`03752e6`](https://github.com/Byron/cargo-smart-release/commit/03752e679c66aa2967aa93a3c57226d53f90001b))
    - Thanks clippy ([`355b17c`](https://github.com/Byron/cargo-smart-release/commit/355b17c871a40c6a4738fa7ef97b4ad8ff0cbf3d))
    - Upgrade env_logger ([`e7dec11`](https://github.com/Byron/cargo-smart-release/commit/e7dec1121192af258ae032ed3dd3cb318cafb798))
    - Upgrade toml_edit ([`b65af5f`](https://github.com/Byron/cargo-smart-release/commit/b65af5f5dc441d0851d5c44cce48757465308efd))
    - Upgrade `cargo_toml` ([`2e25afb`](https://github.com/Byron/cargo-smart-release/commit/2e25afb299d2a0cd347c9eb427fbd9ac0aade75d))
    - Release git-date v0.4.0, git-actor v0.17.0, git-object v0.26.0, git-traverse v0.22.0, git-index v0.12.0, safety bump 15 crates ([`182a2cb`](https://github.com/Byron/cargo-smart-release/commit/182a2cb2eb2809aab570586d0ce84eb9cf414e23))
    - Release git-features v0.26.0, git-actor v0.16.0, git-attributes v0.8.0, git-object v0.25.0, git-ref v0.22.0, git-config v0.14.0, git-command v0.2.1, git-url v0.13.0, git-credentials v0.9.0, git-diff v0.25.0, git-discover v0.11.0, git-traverse v0.21.0, git-index v0.11.0, git-mailmap v0.8.0, git-pack v0.29.0, git-odb v0.39.0, git-transport v0.25.0, git-protocol v0.26.0, git-revision v0.9.0, git-refspec v0.6.0, git-worktree v0.11.0, git-repository v0.31.0, safety bump 24 crates ([`7ac8855`](https://github.com/Byron/cargo-smart-release/commit/7ac885576562b711390471d1bfaa723af79195de))
    - Release git-features v0.25.1, git-url v0.12.2, git-odb v0.38.1, git-transport v0.24.2, git-repository v0.30.2 ([`80c52d6`](https://github.com/Byron/cargo-smart-release/commit/80c52d653071e865ae99b82e2794343c12691e76))
    - Release git-url v0.12.1, git-transport v0.24.1, git-protocol v0.25.1, git-repository v0.30.1, git-commitgraph v0.12.0, gitoxide-core v0.22.0, gitoxide v0.20.0 ([`675b020`](https://github.com/Byron/cargo-smart-release/commit/675b0202cd8a731279bce186de673c46a20353a7))
    - Merge branch 'fix/relative-scplike-urls' ([`97e1a56`](https://github.com/Byron/cargo-smart-release/commit/97e1a56135f1e8ffbe05d24aecf95e22acd74ce9))
    - Adapt to changes in `git-url` ([`ffc5576`](https://github.com/Byron/cargo-smart-release/commit/ffc5576d9722ef274f70e4acef3c406511250a58))
    - Release git-date v0.3.1, git-features v0.25.0, git-actor v0.15.0, git-glob v0.5.1, git-path v0.7.0, git-attributes v0.7.0, git-config-value v0.10.0, git-lock v3.0.1, git-validate v0.7.1, git-object v0.24.0, git-ref v0.21.0, git-sec v0.6.0, git-config v0.13.0, git-prompt v0.3.0, git-url v0.12.0, git-credentials v0.8.0, git-diff v0.24.0, git-discover v0.10.0, git-traverse v0.20.0, git-index v0.10.0, git-mailmap v0.7.0, git-pack v0.28.0, git-odb v0.38.0, git-packetline v0.14.1, git-transport v0.24.0, git-protocol v0.25.0, git-revision v0.8.0, git-refspec v0.5.0, git-worktree v0.10.0, git-repository v0.30.0, safety bump 26 crates ([`5390cfe`](https://github.com/Byron/cargo-smart-release/commit/5390cfeb995247c011a66676578ae74ca113c669))
    - Adapt to changes in `git-repository` ([`e87f25a`](https://github.com/Byron/cargo-smart-release/commit/e87f25a44bf463d1f2c237bc7b10cdcd9f784c9f))
    - Adapt to changes in `git-config` ([`baf0016`](https://github.com/Byron/cargo-smart-release/commit/baf00163f0dd8859d4b38edc587ab0b305d26626))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`82621f1`](https://github.com/Byron/cargo-smart-release/commit/82621f1a18324b9e330d0b33fbb3c3f7e28a03ad))
    - Disable tag.gpgSign in test scripts ([`83e647f`](https://github.com/Byron/cargo-smart-release/commit/83e647f427ba959d19036fe0f9a35b54f1d5ce1b))
    - Upgrade edition to 2021 in most crates. ([`963798a`](https://github.com/Byron/cargo-smart-release/commit/963798af8339ee3d1278aea3c2abca26e1a674c1))
    - Release git-glob v0.4.2, git-config-value v0.8.2, git-lock v2.2.0, git-ref v0.19.0, git-config v0.11.0, git-discover v0.8.0, git-index v0.8.0, git-transport v0.22.0, git-protocol v0.23.0, git-worktree v0.8.0, git-repository v0.28.0, gitoxide-core v0.20.0, gitoxide v0.18.0, safety bump 9 crates ([`83265dd`](https://github.com/Byron/cargo-smart-release/commit/83265dd3e3cc131c548b94f0f26e812e13256395))
    - Release git-diff v0.22.0, git-index v0.7.1, git-pack v0.26.0, git-odb v0.36.0, git-transport v0.21.2, git-repository v0.27.0, safety bump 6 crates ([`b74f30c`](https://github.com/Byron/cargo-smart-release/commit/b74f30cdcb5e82c9664a94fa3979af6a64829e6c))
    - Release cargo-smart-release v0.14.0 ([`351cef4`](https://github.com/Byron/cargo-smart-release/commit/351cef439bc3d87f0e56d27a00061953864961e8))
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`afbbaa1`](https://github.com/Byron/cargo-smart-release/commit/afbbaa185e93f3110e48464943dd8f5e6e05d445))
    - Thanks clippy ([`a561581`](https://github.com/Byron/cargo-smart-release/commit/a561581ce3f942fe986de667dc62c745c9b7f334))
    - Adapt to changes in `git-repository` ([`910afa1`](https://github.com/Byron/cargo-smart-release/commit/910afa13e39bc431271bce378c52f5787933c309))
    - Release cargo-smart-release v0.13.0 ([`9556047`](https://github.com/Byron/cargo-smart-release/commit/95560477ef076904b46f8b02efdb9fa43c290488))
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`2f193dd`](https://github.com/Byron/cargo-smart-release/commit/2f193dd27237e4503442a933ac90fa6a5dfa9856))
    - Thanks clippy ([`740311e`](https://github.com/Byron/cargo-smart-release/commit/740311e84bbcde4e1abff27462a5a5c43279714e))
    - Build complete history information to match with `did crate changed` queries ([`39f2c7d`](https://github.com/Byron/cargo-smart-release/commit/39f2c7dfaae72057784091a1fbeae524c7691d0f))
    - Log errors if these log messages cause stopping the release process. ([`83f9c09`](https://github.com/Byron/cargo-smart-release/commit/83f9c09430cae18d69bb57eedcb8b4822af6f39a))
    - Probably improve logic of determining which conclusion to draw from version data. ([`b45b3b3`](https://github.com/Byron/cargo-smart-release/commit/b45b3b3ca1bc4868688fe0d5e533ab235d8e41d7))
    - Thanks clippy ([`25304c0`](https://github.com/Byron/cargo-smart-release/commit/25304c0646a81c5df007b6bb23e951db51e216b6))
    - Fix smart-release journey tests ([`38cd58f`](https://github.com/Byron/cargo-smart-release/commit/38cd58f0a2d2979118639890e03a9f1ea829bcca))
    - Use `git-repository` to obtain the current push url. ([`19e1ed4`](https://github.com/Byron/cargo-smart-release/commit/19e1ed4ccacc07f425a72e854ae412818fba7bcf))
    - Use `git-repository` to figure out the actual remote to push to. ([`7ec3901`](https://github.com/Byron/cargo-smart-release/commit/7ec390194333027e0ceae286b06e639ff0a3f16f))
    - Upgrade all dependencies, except for `windows` ([`3bca794`](https://github.com/Byron/cargo-smart-release/commit/3bca794cfd9de5c8d5c6d70c3ca79c39b0f49564))
    - Make fmt ([`3a0e689`](https://github.com/Byron/cargo-smart-release/commit/3a0e689b0632cf932dd6dad6ed6b73f5964c5c06))
    - Merge branch 'main' into filter-refs-by-spec ([`aaf0fb6`](https://github.com/Byron/cargo-smart-release/commit/aaf0fb673ae78ac92a525bd7e16849d4ea6152bc))
    - Release git-features v0.22.4, git-url v0.8.0, safety bump 4 crates ([`72c55c3`](https://github.com/Byron/cargo-smart-release/commit/72c55c3634a5ce4ff316e3e60634379b35804781))
    - Merge branch 'main' into filter-refs-by-spec ([`5959c67`](https://github.com/Byron/cargo-smart-release/commit/5959c67cae22a7af9ad5517ee9677be792c7de0c))
    - Release git-diff v0.18.1, git-discover v0.4.2, git-traverse v0.16.4, git-repository v0.23.1 ([`b9d86c9`](https://github.com/Byron/cargo-smart-release/commit/b9d86c97aef5faa179c52e92c58576d7b39ddd75))
    - Merge branch 'main' into filter-refs-by-spec ([`78b98d0`](https://github.com/Byron/cargo-smart-release/commit/78b98d0169cb9e8238851084576a7f5c31848d3c))
    - Adjust journey tests expectations ([`8c3cb51`](https://github.com/Byron/cargo-smart-release/commit/8c3cb5171a18c845a139017793d458d6541244bb))
    - Release cargo-smart-release v0.12.1 ([`63092cf`](https://github.com/Byron/cargo-smart-release/commit/63092cf3d8cb69b00a09914a90b079502b24b378))
    - Update dependencies and assure we get the right version of `crates-index` ([`6d1735b`](https://github.com/Byron/cargo-smart-release/commit/6d1735b2f41829315da2010b5949d0aebd509d04))
    - Fix depreaction warning ([`a946b54`](https://github.com/Byron/cargo-smart-release/commit/a946b54e16bb206eb6ba967d4d87b30a04571549))
    - Release cargo-smart-release v0.12.0 ([`4053bcb`](https://github.com/Byron/cargo-smart-release/commit/4053bcb522fcb4ddc47b0749e001f04a0ad2548a))
    - Merge branch 'main' into filter-refs-by-spec ([`dcf8533`](https://github.com/Byron/cargo-smart-release/commit/dcf85336527f54a46f024c3057bfc5c6a727d60e))
    - Release git-object v0.20.3, git-ref v0.15.4, git-config v0.7.1, git-diff v0.18.0, git-traverse v0.16.3, git-pack v0.22.0, git-odb v0.32.0, git-url v0.7.3, git-transport v0.19.3, git-protocol v0.19.1, git-refspec v0.1.1, git-repository v0.23.0, safety bump 6 crates ([`5352661`](https://github.com/Byron/cargo-smart-release/commit/5352661aa99e4fe7d8fa8fa9f698557b6d272471))
    - Merge branch 'main' into filter-refs-by-spec ([`5238e14`](https://github.com/Byron/cargo-smart-release/commit/5238e140cf61275b0769b7f0ccc7ca8c565be01b))
    - Release git-date v0.1.0, git-actor v0.11.4, git-revision v0.4.3, git-repository v0.22.1, cargo-smart-release v0.11.0, git-commitgraph v0.8.2, gitoxide-core v0.17.0, gitoxide v0.15.0 ([`0e9b6f1`](https://github.com/Byron/cargo-smart-release/commit/0e9b6f1767e68c1afa11f3cf6e1053f86b94d366))
    - Update changelogs prior to release ([`bba61de`](https://github.com/Byron/cargo-smart-release/commit/bba61de051dc95dac554d0b4a0d5182547657c30))
    - Improve performance configuration of smart-release, allowing it to build on msvc by default ([`68ad40e`](https://github.com/Byron/cargo-smart-release/commit/68ad40e9a569299a5336f11a91131b5b1387eba3))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`8e662cb`](https://github.com/Byron/cargo-smart-release/commit/8e662cb2f61f317c8d16af1f8833c3752f3da87d))
    - Merge branch 'main' into remote-ls-refs ([`8b62b53`](https://github.com/Byron/cargo-smart-release/commit/8b62b5362bcbfb17ab33bcdf22959ac08044265f))
    - Remove default link to cargo doc everywhere ([`38a07f2`](https://github.com/Byron/cargo-smart-release/commit/38a07f2ad061f2eba143532c94d23b4254aec438))
    - Merge branch 'main' into remote-ls-refs ([`27f7e21`](https://github.com/Byron/cargo-smart-release/commit/27f7e21addd8f4ef24b3f4d9bc993643b9fb82a1))
    - Prepare for release of git-repository ([`9ef1fbe`](https://github.com/Byron/cargo-smart-release/commit/9ef1fbe78692995fbf9919f77174c81520172cf7))
    - Merge branch 'main' into remote-ls-refs ([`32db219`](https://github.com/Byron/cargo-smart-release/commit/32db2199aec96c2951973e4534e988034f13e843))
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`6745c79`](https://github.com/Byron/cargo-smart-release/commit/6745c790e4fa26dbdd782fbbf254801fdbdba10e))
    - Merge branch 'main' into remote-ls-refs ([`b818514`](https://github.com/Byron/cargo-smart-release/commit/b8185140a0d62d78c9334d461fdd6a0d99c919df))
    - Thanks clippy ([`5a781b9`](https://github.com/Byron/cargo-smart-release/commit/5a781b9667b079c933a0da0f704c3ba7dd5d48a7))
    - Fix typos ([`9056e34`](https://github.com/Byron/cargo-smart-release/commit/9056e34ccde3de5fc71b8a22460b4bd121b20f04))
    - Fix build after changes to `git-url` and `git-config` ([`38cae7f`](https://github.com/Byron/cargo-smart-release/commit/38cae7f9edb76c94a3a5d02b440e5380a9285f2a))
    - Thanks clippy ([`e20bece`](https://github.com/Byron/cargo-smart-release/commit/e20bece9c77751295202fcb7e6655ca14072d765))
    - Release git-path v0.3.0, safety bump 14 crates ([`ac2f40c`](https://github.com/Byron/cargo-smart-release/commit/ac2f40cde1213f986746ab16d1e4593ab7f7ba3b))
    - Use clap 3.2.5 to be able to opt-in to deprecations ([`53b4865`](https://github.com/Byron/cargo-smart-release/commit/53b486513acc7706e1c803d98ca3c1640d38d9a4))
    - Thanks clippy ([`7b1a12f`](https://github.com/Byron/cargo-smart-release/commit/7b1a12feabc68f6a633aafb7fabd7e82f9e8c621))
    - Fix smart-release journey tests ([`9ba1127`](https://github.com/Byron/cargo-smart-release/commit/9ba1127586324441b03127161f9353251a83e0b0))
    - Adjust cargo-smart-release to use latest `git-repository` version ([`d5c5038`](https://github.com/Byron/cargo-smart-release/commit/d5c5038602c9539abef7861fdbb0b53a9e29073d))
    - Allow dependency edits to apply to `target.<cfg>.*dependencies`. ([`be95bcb`](https://github.com/Byron/cargo-smart-release/commit/be95bcb34086e4de77626f5232fa562dd2f03c9d))
    - Make it possible (in theory) to find versions in `target` dependencies. ([`2d251e5`](https://github.com/Byron/cargo-smart-release/commit/2d251e56a0205706e12f853611176191fdfd0082))
    - List any dependency update that is caused by other crates in preview. ([`8647f72`](https://github.com/Byron/cargo-smart-release/commit/8647f724ef5fa00edcd886e44c039f80d72211c4))
    - More useful debug output for `traverse::Dependency`. ([`8f21aa2`](https://github.com/Byron/cargo-smart-release/commit/8f21aa27c8e9030363beb048285e082024f48465))
    - Also remove cargo-smart-release from workspace ([`320262a`](https://github.com/Byron/cargo-smart-release/commit/320262a0302cbc6ddb0b3dc7be87d494a19efc11))
    - Release git-sec v0.1.2, git-discover v0.1.3, cargo-smart-release v0.10.2 ([`f6cf666`](https://github.com/Byron/cargo-smart-release/commit/f6cf6667448c8d8daa51490ca565863ce07ffa26))
    - Release git-path v0.1.3, git-discover v0.1.2, git-repository v0.18.1, cargo-smart-release v0.10.1 ([`100ac8f`](https://github.com/Byron/cargo-smart-release/commit/100ac8f12a3ec86d7480b9095e7a831d134ab903))
    - Correctly determine top-level crate name. ([`e34556b`](https://github.com/Byron/cargo-smart-release/commit/e34556be1f0fed4c6413681ae2b936d8de19cb1e))
    - Release git-path v0.1.2, git-sec v0.1.1, git-config v0.4.0, git-discover v0.1.1, git-pack v0.19.1, git-repository v0.18.0, cargo-smart-release v0.10.0, safety bump 2 crates ([`f27fefd`](https://github.com/Byron/cargo-smart-release/commit/f27fefde9b948f722e57a5b2d75af233c67fcb62))
    - Adjust to changes in git-ref ([`18a3a74`](https://github.com/Byron/cargo-smart-release/commit/18a3a74d997bf58c837fc39d8c6f739385dbe706))
    - Adjust test expectations to match improved parsing in git-conventional ([`3637664`](https://github.com/Byron/cargo-smart-release/commit/363766430ea81767d0221a268079339d87f48f62))
    - Release git-glob v0.2.0, safety bump 3 crates ([`23b3911`](https://github.com/Byron/cargo-smart-release/commit/23b39114773bc36eff6eafeb87336286a69fe1da))
    - Merge branch 'worktree-stack' ([`327a192`](https://github.com/Byron/cargo-smart-release/commit/327a19227831d8ebe585255df82a9470c7ba6b7c))
    - Fix clippy - many false positives this time ([`fc029d0`](https://github.com/Byron/cargo-smart-release/commit/fc029d0aa6d0c0ce3c059c0356c7dcaee68fc981))
    - Fix clippy - many false positives this time ([`90a7914`](https://github.com/Byron/cargo-smart-release/commit/90a791419b606d4ded48a53492b7ac121b872012))
    - Upgrade toml_edit for cargo-smart-release ([`ca72b2b`](https://github.com/Byron/cargo-smart-release/commit/ca72b2bbf366cd005d1e9a4e0231f5e038ee6898))
    - Release git-config v0.2.1, git-diff v0.15.0, git-traverse v0.14.0, git-pack v0.18.0, git-odb v0.28.0, git-ref v0.12.1, git-revision v0.1.0, git-repository v0.16.0, gitoxide-core v0.14.0, gitoxide v0.12.0, safety bump 6 crates ([`713baea`](https://github.com/Byron/cargo-smart-release/commit/713baea6d6aa9f42cdd0011ae89cdfc8ed1ec8e1))
    - Release git-diff v0.14.0, git-bitmap v0.1.0, git-index v0.2.0, git-tempfile v2.0.1, git-lock v2.0.0, git-mailmap v0.1.0, git-traverse v0.13.0, git-pack v0.17.0, git-quote v0.2.0, git-odb v0.27.0, git-packetline v0.12.4, git-url v0.4.0, git-transport v0.16.0, git-protocol v0.15.0, git-ref v0.12.0, git-worktree v0.1.0, git-repository v0.15.0, cargo-smart-release v0.9.0, safety bump 5 crates ([`9acdda3`](https://github.com/Byron/cargo-smart-release/commit/9acdda3b4edd35f293204ad2ebb6946340a7c09a))
    - Thanks clippy ([`8b3fa40`](https://github.com/Byron/cargo-smart-release/commit/8b3fa402550a90265e10f09bf84c9fc4a8de4d13))
    - Adapt to breaking changes in git-actor ([`344e842`](https://github.com/Byron/cargo-smart-release/commit/344e8424d239bdc506e9b0f438aa0a3f501ce7e9))
    - Fix clap warnings ([`5abfe76`](https://github.com/Byron/cargo-smart-release/commit/5abfe764472a9cf3b95769aa2588438a6ae740d2))
    - Release git-tempfile v2.0.0, safety bump 6 crates ([`7058818`](https://github.com/Byron/cargo-smart-release/commit/70588180ba5a19b81bf645dc8332fea46431f4b7))
    - Adapt cargo-smart-release to changes in git-tempfile ([`b03947f`](https://github.com/Byron/cargo-smart-release/commit/b03947f436740667e5fea6b07898a7bead1aa160))
    - Improve headline parsing for git-conventional messages. ([`7634f69`](https://github.com/Byron/cargo-smart-release/commit/7634f69f76480a0b76a58e3565f27d925d8fd844))
    - Merge branch 'sassman-config-subsection-iter' ([`96e4b45`](https://github.com/Byron/cargo-smart-release/commit/96e4b4531b302695b35f21902695c27e08f17b23))
    - Release cargo-smart-release v0.8.0 ([`261159e`](https://github.com/Byron/cargo-smart-release/commit/261159e6127289127add3a0a216d3dc1331f544f))
    - Release git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`c401680`](https://github.com/Byron/cargo-smart-release/commit/c4016809bfe676b80e518030873f9255f32d3707))
    - Release git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`f78c228`](https://github.com/Byron/cargo-smart-release/commit/f78c228feb193b589bcec6c373863f0021316fa0))
    - Release git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`a01bc52`](https://github.com/Byron/cargo-smart-release/commit/a01bc524e112f0c968a73e0a4a373ffc55094066))
    - Release git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`e56c586`](https://github.com/Byron/cargo-smart-release/commit/e56c58686be525b76785944e71d5f619832ff1ac))
    - Release git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`fd66aa4`](https://github.com/Byron/cargo-smart-release/commit/fd66aa458d294a5fd385a245899c6998a01eb9e9))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`7c7f97c`](https://github.com/Byron/cargo-smart-release/commit/7c7f97ced10d242ff22052520c1b0b0aab09752b))
    - Prepar changelogs for cargo-smart-release release ([`889254d`](https://github.com/Byron/cargo-smart-release/commit/889254d0c990153ec1355666f47dedf755b5916f))
    - Highlight (non-fatal) errors when losslessly parsing changelogs ([`9ee78e7`](https://github.com/Byron/cargo-smart-release/commit/9ee78e7426d26ca010ed935f92566ed2f1fc3201))
    - Better not have items within items in changelogs ([`4da8e9f`](https://github.com/Byron/cargo-smart-release/commit/4da8e9f181f2275394750c44e2b6a4edfe6187ba))
    - Upgrade dependencies ([`bdcfe79`](https://github.com/Byron/cargo-smart-release/commit/bdcfe79c90f228f569e9dac186fcaf4413f45283))
    - Minor refactor ([`adf19ca`](https://github.com/Byron/cargo-smart-release/commit/adf19ca9693dc6987d13442d1cde8e77a15f3713))
    - Upgrade to pulldown-cmark 0.9 ([`eb02eb7`](https://github.com/Byron/cargo-smart-release/commit/eb02eb7bc9244e90bc408b215f234f4e5690cdd2))
    - Commit statistics reveal the days passes between releases ([`14af24b`](https://github.com/Byron/cargo-smart-release/commit/14af24b24977d7bfcd3297760eb2080a3b403ed2))
    - Upgrade to clap 3.0.0 ([`e928947`](https://github.com/Byron/cargo-smart-release/commit/e92894735c60952d3bd083deeb7f79236d032cb1))
    - Adapt to changes in git-repository ([`428747b`](https://github.com/Byron/cargo-smart-release/commit/428747ba40e759d4921564f2cb5f15b03c564f5f))
    - Release git-chunk v0.2.0, safety bump 4 crates ([`44ecb21`](https://github.com/Byron/cargo-smart-release/commit/44ecb2151b624c61996203935905f57a8e89c965))
    - Upgrade to latest clap rc ([`a5fef78`](https://github.com/Byron/cargo-smart-release/commit/a5fef78134fd28581caf7169c20e2deff5da1d10))
    - Make fmt ([`4d10141`](https://github.com/Byron/cargo-smart-release/commit/4d101418e1abe4b0ddfcd60a40dae96888c8c35c))
    - Thanks clippy ([`b8c1380`](https://github.com/Byron/cargo-smart-release/commit/b8c13806eef261274072ab6b134285b3d0ce02a8))
    - Release git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0 ([`668c9cd`](https://github.com/Byron/cargo-smart-release/commit/668c9cd9b7c22846273b6975bf6871be446ea465))
    - Release git-features v0.18.0, git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0, safety bump 12 crates ([`d3e30a1`](https://github.com/Byron/cargo-smart-release/commit/d3e30a1def1c737999f573ed756b89ba5a47fe07))
    - Release git-repository v0.12.0, cargo-smart-release v0.6.0 ([`bf596d8`](https://github.com/Byron/cargo-smart-release/commit/bf596d8a9f5110682c491e14488c3c5fbe51514d))
    - Release git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0 ([`e60dcf7`](https://github.com/Byron/cargo-smart-release/commit/e60dcf7ff28af53938a87bb23389d23702bee70d))
    - Adjusting changelogs prior to release of git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0, safety bump 5 crates ([`349cf36`](https://github.com/Byron/cargo-smart-release/commit/349cf367111acbe759f9ec9703f309fd9625a93f))
    - Don't let dev-dependencies participate in traversal unless they have a version specified. ([`bfb6b5f`](https://github.com/Byron/cargo-smart-release/commit/bfb6b5fe9ff34c6882eb57c47099bb729f941b85))
    - Note about smart-release being (too) eager to release ([`00c6f15`](https://github.com/Byron/cargo-smart-release/commit/00c6f156af4db9f9d06e9f79d9e7e800e0b86197))
    - Refactor ([`1c1c842`](https://github.com/Byron/cargo-smart-release/commit/1c1c842e13d77ffa103b16635b8c5592997df660))
    - Write down a few more 'cargo changelog' shortcomings ([`b60ccde`](https://github.com/Byron/cargo-smart-release/commit/b60ccde631b618614dea64975c187cb60349e52e))
    - Release cargo-smart-release v0.5.6 ([`8d0d063`](https://github.com/Byron/cargo-smart-release/commit/8d0d063160a9b92884d2f2bc76942f97b4806e4e))
    - Release cargo-smart-release v0.5.5 ([`c2b6a99`](https://github.com/Byron/cargo-smart-release/commit/c2b6a99fcc23b145dd485ba43fe7b08791b79e3c))
    - Release cargo-smart-release v0.5.4 ([`7829ac6`](https://github.com/Byron/cargo-smart-release/commit/7829ac6eb1528792ee7e71108044e47a365ec590))
    - Release cargo-smart-release v0.5.3 ([`74099da`](https://github.com/Byron/cargo-smart-release/commit/74099daaaf35d3245cbed6b9782ee5d538f362b5))
    - Strip `.git` suffix from repository paths when using it in urls ([`ffd0561`](https://github.com/Byron/cargo-smart-release/commit/ffd0561cab8fcdb8b36bd5d88bed4054c14727b5))
    - Remove extra '/' after https://github.com/ based URLs ([`0b077c0`](https://github.com/Byron/cargo-smart-release/commit/0b077c0e12cf2edb7e5a3b49116ccbeee1fa18ec))
    - Release cargo-smart-release v0.5.2 ([`5d7faaf`](https://github.com/Byron/cargo-smart-release/commit/5d7faaf3eb5001ee9ee98b2295578744d1aa5138))
    - Release cargo-smart-release v0.5.1 ([`eba5aa9`](https://github.com/Byron/cargo-smart-release/commit/eba5aa931f32ec136230be417c44a44063b07b4c))
    - Release cargo-smart-release v0.5.0 ([`6517d92`](https://github.com/Byron/cargo-smart-release/commit/6517d925d54d637e8b22ab977cbef48b3cbae755))
    - Changelog update ([`ba37d54`](https://github.com/Byron/cargo-smart-release/commit/ba37d548a7172387d79d66a7faaf1d7a7e0feae1))
    - Adjusting changelogs prior to release of cargo-smart-release v0.5.0 ([`bb9fc87`](https://github.com/Byron/cargo-smart-release/commit/bb9fc87f3cbb47217161b681fea490a0997b2542))
    - Release git-hash v0.8.0, git-features v0.17.0, git-actor v0.6.0, git-object v0.15.0, git-diff v0.11.0, git-traverse v0.10.0, git-pack v0.13.0, git-odb v0.23.0, git-packetline v0.12.0, git-transport v0.13.0, git-protocol v0.12.0, git-ref v0.9.0, git-repository v0.11.0, git-commitgraph v0.6.0, gitoxide-core v0.12.0, gitoxide v0.10.0, cargo-smart-release v0.5.0, safety bump 16 crates ([`6fae139`](https://github.com/Byron/cargo-smart-release/commit/6fae139187d3d0abc2da4fd26a0b1471ee37b14b))
    - Thanks clippy ([`60cd546`](https://github.com/Byron/cargo-smart-release/commit/60cd54622bbaa1f54168d6e90ba51f136ae7c1b8))
    - Thanks clippy ([`99f4249`](https://github.com/Byron/cargo-smart-release/commit/99f4249ef774343db0b53b1422fe89e6508b8f4b))
    - Thanks clippy ([`e513627`](https://github.com/Byron/cargo-smart-release/commit/e513627aaec5786975e05c34822954534f1de779))
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`ff6f6b9`](https://github.com/Byron/cargo-smart-release/commit/ff6f6b9b16e7278b22b6fa1e5242bacc0b2acf79))
    - Thanks clippy ([`1abca7a`](https://github.com/Byron/cargo-smart-release/commit/1abca7a677d380e2f78ab99c7d5279a83f69b65f))
    - Thanks clippy ([`ef4bbf4`](https://github.com/Byron/cargo-smart-release/commit/ef4bbf4fa1149b740dd4f47336fd68a0e87e779f))
    - Thanks clippy ([`81325a6`](https://github.com/Byron/cargo-smart-release/commit/81325a660d2494651f56a3611ffa3440bb584591))
    - Thanks clippy ([`875f61b`](https://github.com/Byron/cargo-smart-release/commit/875f61b8ebd1d3085f60c5a995c6f7f514f54a00))
    - Thanks clippy ([`4f9ef68`](https://github.com/Byron/cargo-smart-release/commit/4f9ef68be0ea37eac0201de63d334bcc1287928a))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a697701`](https://github.com/Byron/cargo-smart-release/commit/a697701cf231427c981e178da41129fc56311f80))
    - Thanks clippy ([`6bf84f1`](https://github.com/Byron/cargo-smart-release/commit/6bf84f1fe3475ebcfa62ded9dd7552f80d85c3f3))
    - Thanks clippy ([`e451877`](https://github.com/Byron/cargo-smart-release/commit/e45187755cb136c6e761b6851a5dabd3bcd25e4c))
    - Thanks clippy ([`97e06ca`](https://github.com/Byron/cargo-smart-release/commit/97e06cad45a1d87ada61cb036c5c6092e07432db))
    - Thanks clippy ([`3dfaad5`](https://github.com/Byron/cargo-smart-release/commit/3dfaad5b292b87f4ff1c5aad11a2aa262af47006))
    - Thanks clippy ([`da36c45`](https://github.com/Byron/cargo-smart-release/commit/da36c451d08fc6e0aea97c40cf4129d446645acf))
    - Thanks clippy ([`4bbaf07`](https://github.com/Byron/cargo-smart-release/commit/4bbaf07d4109b46fe3c8ac2fafcac69ead93dcba))
    - Update changelogs just for fun ([`82bee6f`](https://github.com/Byron/cargo-smart-release/commit/82bee6f0f0636c024c70a7c702de0d9a5dd6f22b))
    - Thanks clippy ([`ceb48f9`](https://github.com/Byron/cargo-smart-release/commit/ceb48f9f0568a64e53e55e0cda6f6c6f3f6ebc97))
    - Thanks clippy ([`1e76bf4`](https://github.com/Byron/cargo-smart-release/commit/1e76bf43176fb2654da660f60f694622d2fc7555))
    - Thanks clippy ([`6a96579`](https://github.com/Byron/cargo-smart-release/commit/6a96579e4f16a46b2e21c9007c26b432c43dc67f))
    - Thanks clippy ([`3d0f483`](https://github.com/Byron/cargo-smart-release/commit/3d0f4833da412e83a2084a730909cd38d615d45e))
    - Thanks clippy ([`6c30330`](https://github.com/Byron/cargo-smart-release/commit/6c3033037b7d7fb7f14ecff6a1cbec3f00f69305))
    - Thanks clippy ([`38e6392`](https://github.com/Byron/cargo-smart-release/commit/38e639223fee6427172bb6decb32cbe456fbf05c))
    - Thanks clippy ([`8086f68`](https://github.com/Byron/cargo-smart-release/commit/8086f6870841f022bde6c226fe93cd2e84d8d425))
    - Thanks clippy ([`8a89a78`](https://github.com/Byron/cargo-smart-release/commit/8a89a78ed9819e1979e8aa8f7196dec1ce73209f))
    - Thanks clippy ([`fc4d445`](https://github.com/Byron/cargo-smart-release/commit/fc4d445f7c02cb59bd5a68c0ed78c19f88cf32f8))
    - Thanks clippy ([`ceb111a`](https://github.com/Byron/cargo-smart-release/commit/ceb111ab0904cbb7c26f33267bcd806da0bf83a3))
    - Merge branch 'main' into changelog-generation ([`4506491`](https://github.com/Byron/cargo-smart-release/commit/45064916402611149eaa830fe87abcde96718e8e))
    - Don't claim to change manifest version if it's the same one ([`d504df3`](https://github.com/Byron/cargo-smart-release/commit/d504df3b8ed0439226b659ca9bfdc55adc0506b2))
    - Thanks clippy ([`33b4890`](https://github.com/Byron/cargo-smart-release/commit/33b4890fc67e39df9e68d2a80d806763100b9567))
    - Thanks clippy ([`b06ebb4`](https://github.com/Byron/cargo-smart-release/commit/b06ebb479f8b58b3a6aeab7d37eac6175aaf0c50))
    - Thanks clippy ([`cf8982f`](https://github.com/Byron/cargo-smart-release/commit/cf8982fa488dafd74fb35237b11560adf92c3574))
    - Thanks clippy ([`9207525`](https://github.com/Byron/cargo-smart-release/commit/920752584944f8823d68580e3db330c9962abc6a))
    - Bump git-repository v0.10.0 ([`e837d98`](https://github.com/Byron/cargo-smart-release/commit/e837d98e221611bd533e31058b4af37791962b9f))
    - Thanks clippy ([`b6ff21f`](https://github.com/Byron/cargo-smart-release/commit/b6ff21fbaf64eeda14555c338c1a645bf443443f))
    - [repository #164] fix build ([`1eead74`](https://github.com/Byron/cargo-smart-release/commit/1eead74ccd21f98059b8243616b164facf96ad2b))
    - Release git-repository v0.9.1 ([`ce5da38`](https://github.com/Byron/cargo-smart-release/commit/ce5da38f18b0e8ae9b032002f64989b76b5fd874))
</details>

## 0.19.0 (2023-04-27)

### New Features

 - <csr-id-00471ae75464ee14b93480b016858b6606e0b194/> avoid panics in favor of error handling. That way more information can be provided which helps with a fix.

## 0.18.0 (2023-04-27)

### Documentation

 - <csr-id-cc48c35d0ecf35824910c5b6ecc62fe9b2aff1b5/> fix minor typos

### Bug Fixes

 - <csr-id-d1bd513f27e17787eb223f7b0521f954c518153e/> $HOME detection on windows

## 0.17.0 (2023-02-23)

### New Features

 - <csr-id-eee22110b7f0f3e4765043e4acc9c832c2b86549/> capitalize the first letter of reproduced commit messages.
   That way they look more consistent with the rest of the text, which typically
   is capitalized as well.

## 0.16.1 (2023-02-17)

### Bug Fixes

 - <csr-id-fe58a79edde7d916ce72badc40204c507d015ebf/> enable local-offset support in the `time` crate and opt-in to it.
   This should allow proper times for release dates like before as they
   respect the local time, instead of defaulting to UTC-time.

## 0.16.0 (2023-02-17)

### New Features

 - <csr-id-cd1202388b1cd290211c2f9580d2acbdd82384f9/> rename tracking for crates in the crate-root.
   Now it's possible to rename crates if they are directly at the crate root
   without loosing their history.

### Bug Fixes

 - <csr-id-35a1df715e1e8e28fb98237908c964ddbb8981fa/> assure we can track dependencies correctly.
   Previously, if worktree crates would also be used as crates.io crates,
   the dependency traversal would fail to find packages that come in from crates.io
   as opposed to the workspace, and discard them, causing dependencies to be missed.
   
   Now we correctly ignore workspace dependencies from crates.io.

## 0.15.0 (2023-02-09)

<csr-id-63969671df60b4e07e2d7d671c657639d055b0b8/>

### Chore

 - <csr-id-63969671df60b4e07e2d7d671c657639d055b0b8/> upgrade to clap 4.1

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### Bug Fixes

 - <csr-id-dc580414cbc7593c3ccf257f1f62d7323a3a3424/> handle worktree members which are also used as dependencies from crates.io.
   Previously there would be an assertion error if worktree members
   are not used only by path, but also by dependency to crates.io.
 - <csr-id-1ce3190000f6211ce31468c7603d491bb5b90293/> Disable tag.gpgSign in test scripts
   This is done for the same reason that commit.gpgsign is disabled for test
   scripts. It prevents test failures if the user has tag.gpgsign enabled in
   their global git config when invoking tests.

### New Features (BREAKING)

 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `gix-features` and `gix-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

## 0.14.0 (2022-11-06)

### Bug Fixes

 - <csr-id-0eca94d84bd82f2083b41acdb316edce54365f11/> `where -> were` typo fix.

## 0.13.0 (2022-10-10)

### Bug Fixes

 - <csr-id-118c19628e00dce0248ea975c5e93745c8058b5a/> build complete history information to match with `did crate changed` queries
   Previously it was possible see a crate was changed, but didn't receive a
   version bump which would in turn halt the release process.
   
   The issue was an algorithm inability to find changes in the commitgraph
   because it would not look at the correct tree, causing trees to be
   missed entirely. This in turn caused it to not see changes that were
   present and the mismatch in question.
 - <csr-id-03f3ffc0816659b03f4eb0b2f24154ab2f86b95a/> log errors if these log messages cause stopping the release process.
   Previously it was possible see `log::warn` but have the process abort
   with proclaimed errors which weren't obvious. Now they are `log::error`
   as one would expect.

## 0.12.1 (2022-08-31)

### Fix

- Use correct English in `Commit Details`, see [#513](https://github.com/Byron/gitoxide/issues/513) for details.

## 0.12.0 (2022-08-30)

### Bug Fixes

 - <csr-id-fcbea050d04f0b763adef80d9de829f171dda571/> Assure `git@github.com/user/repo` urls transform into https urls correctly.

## 0.11.0 (2022-08-24)

<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Bug Fixes

 - <csr-id-376749cc49c6dafcc314b8435d6feac81482b3f5/> allow dependency edits to apply to `target.<cfg>.*dependencies`.
   Previously these would be skipped, which would cause the publish to
   abort due to invalid manifests - some dependencies would still refer
   to an outdated but incompatible version.
 - <csr-id-988c61e07bdb52870794e70e94b925de7acb402e/> List any dependency update that is caused by other crates in preview.
   Previously it was possible that crates there were about to be published
   didn't show up in the list of crates that received a safety version
   bump.

## 0.10.2 (2022-05-27)

### Bug Fixes

 - <csr-id-64b951cade24c69d522a76ad217bea70a4afe45a/> Avoid running into the `default-members` trap with 'cargo publish'.
   Default-members in a cargo workspace can override what's actually
   published, so we have to be explicit about what to publish.
   
   This is only the case when there is more than one members in the
   workspace, even though it would probably work as well if the package
   would be specified with a single-crate workspace.

## 0.10.1 (2022-05-23)

### Bug Fixes

 - <csr-id-33a2bd6bd3faf597f020924e42082a714d3253b9/> Correctly determine top-level crate name.
   Previously it was possible to think the crate is part of a multi-crate
   worktree even though it wasn't, causing changelogs to not pick up their
   history as it would look for different tag names.

## 0.10.0 (2022-05-21)

### Bug Fixes

 - <csr-id-fcaa6353297fd1d4cb30ca3a873f76efb62e45e1/> Don't assume crates are non-breaking just because they are in the user selection.
   Crates showing up 'early' in our list could cause the entire
   breakage-propagation to fail which led the crate to be ignored entirely
   even when their dependees changed their version. This led to
   inconsistent version requirements which would abort any cargo call.

## 0.9.0 (2022-04-03)

<csr-id-51d1c686763b4c036ec2c3c15d7c3ebb48e208de/>
<csr-id-bbc6efeceb26050973e1425e68a52e51b9df4572/>

A quality-of-life release which should make publishing of inter-dependent crates much more reliable.

### New Features

 - Wait for previously published crates explicitly to avoid running into publish failures due to the previously published crate not present 
   even after 3 attempts.

### Bug Fixes

 - <csr-id-f9daba439e2d669c8b0a6bcac9ff50cbf9d80371/> improve headline parsing for git-conventional messages.
   
   It is now case-insensitive, which prevents it from getting tripped
   up in some cases.
 - <csr-id-1feb118e87f302d030ceca03ce8f8c22d40d7f03/> Don't pass judgement on usefulness of certain kinds of git-conventional messages.
   
   Previously we would intentionally avoid writing out information about
   refactors or chores as they are not deemed useful in a changelog.
   
   However, this can be confusing for anyone but the original author.
   
   We now write them as seen.
   
   Future iterations on this may consider adding more options
   to configure which sections should go into the changelog.

### Refactor (BREAKING)

 - <csr-id-bbc6efeceb26050973e1425e68a52e51b9df4572/> clarify different repository types much better

## 0.8.4 (2021-09-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [smart-release] auto-detect changes in production crates as well ([`4f50c0e`](https://github.com/Byron/cargo-smart-release/commit/4f50c0ead9ef3240628006798e5b2a0e58446f55))
    - [smart-release #195] update test output to match CI… ([`beb155f`](https://github.com/Byron/cargo-smart-release/commit/beb155f3b5d648b86a964cd51b551ae2f2ee8a84))
    - [smart-release #195] better error for untracked files. ([`4dee24b`](https://github.com/Byron/cargo-smart-release/commit/4dee24bcdb9a4f0116f90bf2492807736ae2d0b6))
    - [smart-release #195] assure dependent packages are not packages to be published ([`68a33f8`](https://github.com/Byron/cargo-smart-release/commit/68a33f86b0f424fb4aeaa7e18a31cdf81601b809))
    - [smart-release #195] refactor ([`39a87f8`](https://github.com/Byron/cargo-smart-release/commit/39a87f8df0c0dad24294f6bbc18bcd503e91d153))
    - [smart-release #195] refactor ([`e54a241`](https://github.com/Byron/cargo-smart-release/commit/e54a241d7baddd2f4a40dd94cf081c963615897a))
    - [smart-release #195] don't tout changes that aren't really there… ([`7b103e4`](https://github.com/Byron/cargo-smart-release/commit/7b103e49eb9dc4e52cae8b06c6d7ce662c190135))
    - [smart-release #195] another test to validate log output ([`64f9ff5`](https://github.com/Byron/cargo-smart-release/commit/64f9ff52b74365ffc6cb7c30eab78867b31fa1a9))
    - [smart-release #195] a test that in theory should trigger the desired behaviour ([`fd4bd1b`](https://github.com/Byron/cargo-smart-release/commit/fd4bd1b26ff986455fc377fd8e9d625c5eda042b))
    - [smart-release #194] basic journey test setup ([`87d9451`](https://github.com/Byron/cargo-smart-release/commit/87d94518e97ef3d2443b13308d6977836ba920a5))
    - Thanks clippy ([`493c9d2`](https://github.com/Byron/cargo-smart-release/commit/493c9d21ad4483738f1f8bbd22ad604ad2edbe4b))
    - [smart-release #194] conservative pre-release version updates ([`ce5833f`](https://github.com/Byron/cargo-smart-release/commit/ce5833f8941943108709b4c19a48950ccb1fc825))
    - Bump git-repository v0.9.0 ([`6085a6a`](https://github.com/Byron/cargo-smart-release/commit/6085a6a687a6368e12e6ca4d7700e9efc7549c89))
    - Release cargo-smart-release v0.3.1 ([`2b48d9c`](https://github.com/Byron/cargo-smart-release/commit/2b48d9c991eb9e3fbdea134211c9493c4fea1450))
</details>

## 0.8.3 (2021-09-06)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 39 commits contributed to the release over the course of 20 calendar days.
 - 20 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [repository #190] refactor ([`c53f8c1`](https://github.com/Byron/cargo-smart-release/commit/c53f8c168ac1bd2247f76d8ea5f418c5dae76102))
    - [repository #190] fix build ([`b047941`](https://github.com/Byron/cargo-smart-release/commit/b047941dfdfaa556d5127f8047b65a1493a6d3c7))
    - [repository #190] a major step forward with `head()` access ([`2121ca6`](https://github.com/Byron/cargo-smart-release/commit/2121ca6f6bed7f4d8c42f549f747435a07ca102d))
    - Release cargo-smart-release v0.3.0 ([`1dd9e67`](https://github.com/Byron/cargo-smart-release/commit/1dd9e6780be463236e8f8f60fa204da0aec50f76))
    - [smart-release #174] add asciinema recording of failed release ([`4000034`](https://github.com/Byron/cargo-smart-release/commit/40000346bf3bf2198e1c2173a7c523f8cd35662e))
    - [smart-release #174] prepare changelog ([`20d7a93`](https://github.com/Byron/cargo-smart-release/commit/20d7a939a7169af7892697f4cb3b743b89d8da3f))
    - Bump git-repository v0.8.0 ([`a395f98`](https://github.com/Byron/cargo-smart-release/commit/a395f981c75afd5c4d08c2c8d3c757126ee8a17f))
    - [smart-release] Adjust commit message depending on whether we are skipping the publish… ([`0df4452`](https://github.com/Byron/cargo-smart-release/commit/0df44522757bf7c2b72ebf4c70ed82bd241cdcaf))
    - [object #177] migrate immutable::tree to crate::tree ([`4c297b5`](https://github.com/Byron/cargo-smart-release/commit/4c297b5b5dc26df97af807bffcd32cd733b15885))
    - [ref #175] make 'mutable' module private ([`42717b6`](https://github.com/Byron/cargo-smart-release/commit/42717b6bc8d68673846e4095a6933bd9c749052f))
    - Release git-lock v1.0.0 ([`be05ee4`](https://github.com/Byron/cargo-smart-release/commit/be05ee434db0c5eb60b05acdc81b5fbe5c232235))
    - [stability #171] git-ref is now ST1 and available through git-repository ([`e731578`](https://github.com/Byron/cargo-smart-release/commit/e73157876148ad2ce3777cb2505384ef89a1c7ef))
    - [smart-release #171] Try to avoid unstable git-repository features… ([`2a85fce`](https://github.com/Byron/cargo-smart-release/commit/2a85fcebe5b15b039132d4e993efff5c0625fedd))
    - [stability #171] Don't provide access to less stable crates in `Respository` ([`ef7e6c1`](https://github.com/Byron/cargo-smart-release/commit/ef7e6c19968160aee3417b7ff5a7d291b7063be4))
    - [stability #171] Don't leak unstable plumbing crates in git-repository… ([`6655eb2`](https://github.com/Byron/cargo-smart-release/commit/6655eb2fe60df19c8b8a82e2d1b3708263b1ba15))
    - [stability #171] finish tier description… ([`b4d3317`](https://github.com/Byron/cargo-smart-release/commit/b4d3317e2208f863e07cbca8418fccbb185f65d5))
    - [ref #165] refactor ([`517f5f5`](https://github.com/Byron/cargo-smart-release/commit/517f5f548050aaef21286616b98e82d93042d7b0))
    - [repository #165] refactor ([`96beaf8`](https://github.com/Byron/cargo-smart-release/commit/96beaf8aa9d0393c21959fcb82b4632aefd3e8c5))
    - [repository #165] refactor; fine grained allow(missing_docs)… ([`bc9d82d`](https://github.com/Byron/cargo-smart-release/commit/bc9d82d15668cde06ff7e940c45b165e6658523c))
    - [repository #165] prepare for writing light docs for Easy ([`1173003`](https://github.com/Byron/cargo-smart-release/commit/1173003472fd728b95c865313c9846f2dd7a0d5f))
    - [repository #165] refactor ([`7da1c77`](https://github.com/Byron/cargo-smart-release/commit/7da1c771f9bcee869d0075b1b771f0b960b39957))
    - [repository #165] a sample of a simpler way to create a tag ([`abc0b5d`](https://github.com/Byron/cargo-smart-release/commit/abc0b5d980db9e091b8b261bdbc6a9ddf23874db))
    - [smart-release #165] Use generic edit-reference functionality ([`7a33e94`](https://github.com/Byron/cargo-smart-release/commit/7a33e9484ecae76e253dfaccfd897e502eb560c2))
    - [repository #165] refactor ([`c5ad72c`](https://github.com/Byron/cargo-smart-release/commit/c5ad72c9c71c589587129b64a72ca7871d8b48eb))
    - [repository #165] offer panicking type conversions for objects ([`a469f3a`](https://github.com/Byron/cargo-smart-release/commit/a469f3ae48c508921e22dbf019e34bf959a66680))
    - [repository #165] try a more common naming convention for fallbile things… ([`9eae290`](https://github.com/Byron/cargo-smart-release/commit/9eae29037d598263e6690ce53a30207bd0b2cf53))
    - [smart-release #162] use TreeRef capabilities to lookup path ([`7a2ed98`](https://github.com/Byron/cargo-smart-release/commit/7a2ed98d5573228e94d865a88028979420c61bbd))
    - [repository #162] finally let smart-release use the correct abstraction for peeling ([`33e8a5e`](https://github.com/Byron/cargo-smart-release/commit/33e8a5ec2509e46a2359dcfc62a0340645a2a666))
    - [repository #162] Add id field to ObjectRef… ([`55f8f38`](https://github.com/Byron/cargo-smart-release/commit/55f8f382f1f052b8d69027c220287b2342c53ed4))
    - [repository #162] experiment with finding objects… ([`0a82358`](https://github.com/Byron/cargo-smart-release/commit/0a82358bd89ceec05a3dc9a5813bb07fa5bbd4f9))
    - [repository #162] Cannot ever store a RefCell Ref in an object… ([`3e577df`](https://github.com/Byron/cargo-smart-release/commit/3e577df9ae6565a701ef01ad42e58fdd5d0892ce))
    - [repository #162] experiemnt with optionally keeping data in Object ([`f5876fe`](https://github.com/Byron/cargo-smart-release/commit/f5876fe7a5f8b4f07cefd1c97d837ff36b359a14))
    - [smart-release #162] Fix short flags ([`01fc50f`](https://github.com/Byron/cargo-smart-release/commit/01fc50f4cb1428960586cbf586f4c744061bdd7f))
    - [smart-release #162] don't throw away work… ([`4771395`](https://github.com/Byron/cargo-smart-release/commit/4771395f0db2f480724d4b9666e7a0364be60e16))
    - [smart-release #162] refactor ([`9c61e3a`](https://github.com/Byron/cargo-smart-release/commit/9c61e3ac90625c6f35ebf0713428fae4a930582a))
    - [smart-release #162] peeling objects to a certain target kind… ([`f70f0e9`](https://github.com/Byron/cargo-smart-release/commit/f70f0e9887679fac3533176c53ad216ad6388d36))
    - [smart-release #162] a single import path for ReferenceExt ([`e1e9616`](https://github.com/Byron/cargo-smart-release/commit/e1e96160d99704c58565eba0a36d979303b2b07f))
    - [smart-release #162] replace reference peeling with git_easy ([`42802a2`](https://github.com/Byron/cargo-smart-release/commit/42802a2af9fdba2b6fa34b5661bd17654bb85efd))
    - [smart-release #162] smart-release uses Easy repository in 'plumbing' mode ([`ff3f077`](https://github.com/Byron/cargo-smart-release/commit/ff3f07742f53d53d5d1bd1faa9d8b2f163eece19))
</details>

## 0.8.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 74 commits contributed to the release over the course of 4 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [smart-release #164] improve handling of empty commits ([`70b8543`](https://github.com/Byron/cargo-smart-release/commit/70b85433bf26d738731665951dacd51b2e9edb00))
    - [smart-release #164] Make it easier to change a single crate's version only ([`5d7990f`](https://github.com/Byron/cargo-smart-release/commit/5d7990f78225ba012a02a14ed12ab111ffe036cc))
    - [smart-release #162] only warn if there is working tree modifications in dry-run mode… ([`8ebef73`](https://github.com/Byron/cargo-smart-release/commit/8ebef732039ec95bca733dc060dd19f0db3af78b))
    - [smart-release #162] clearer messages ([`01a5fd1`](https://github.com/Byron/cargo-smart-release/commit/01a5fd17bdec2c383741eb741a673df7958880a1))
    - Thanks clippy ([`c87ea9d`](https://github.com/Byron/cargo-smart-release/commit/c87ea9d15839cd4600501b5d405a026eaeab8ff4))
    - [smart-release #162] top-level crate uses version-only tag ([`1b913d7`](https://github.com/Byron/cargo-smart-release/commit/1b913d79aa6b413b1bdd134f74c69f8376b23e01))
    - [smart-release #162] FAIL: single-crate workspaces use version-only tags ([`d8147eb`](https://github.com/Byron/cargo-smart-release/commit/d8147eb62b9a58b9812f831ab8b2a591ac634b9f))
    - [smart-release] better --verbosity handling ([`07ca1bd`](https://github.com/Byron/cargo-smart-release/commit/07ca1bd9679e69071dea28a6f2b0f238c603b3b3))
    - [smart-release] properly obtain top-level crate name using manifest ([`bf4a890`](https://github.com/Byron/cargo-smart-release/commit/bf4a890268fe57ecdd13d7dd706305ceac1701d4))
    - Apply nightly rustfmt rules. ([`df13c6b`](https://github.com/Byron/cargo-smart-release/commit/df13c6bf1f57b41bf56bb180213417dae952a8c7))
    - Release cargo-smart-release v0.2.4 ([`6716f9d`](https://github.com/Byron/cargo-smart-release/commit/6716f9d864cb80a02375049914641a484dbdf96f))
    - [smart-release #160] fix auto-push issue ([`844960f`](https://github.com/Byron/cargo-smart-release/commit/844960f076a9e2631962591a0843b27bff0bdd1e))
    - Release cargo-smart-release v0.2.3 ([`438896d`](https://github.com/Byron/cargo-smart-release/commit/438896d0387c376d987af09a7fb4995c2fd7a13a))
    - [smart-release #160] update chnagelog ([`9b5b321`](https://github.com/Byron/cargo-smart-release/commit/9b5b3216de230ef95c22f6b9c9a4a9e207242b7b))
    - [smart-release #160] Add the --skip-push flag… ([`1a8de98`](https://github.com/Byron/cargo-smart-release/commit/1a8de986345193aadecf997c2f375b17925e221c))
    - [smart-release #160] Push after creating a single tag ([`e418201`](https://github.com/Byron/cargo-smart-release/commit/e4182017cd19fdf138de297b246c81b477b2ef7c))
    - [smart-release #160] a seemingly nice '--verbose' mode… ([`dff05e1`](https://github.com/Byron/cargo-smart-release/commit/dff05e1a52e4501251088d87dacec13c268b3b83))
    - Thanks clippy ([`74e42d1`](https://github.com/Byron/cargo-smart-release/commit/74e42d1db7167c2b023fbc19643eb0067a935ae0))
    - [smart-release #160] avoid trying to use an empty path when detecting changes… ([`9a24d05`](https://github.com/Byron/cargo-smart-release/commit/9a24d05977b8fdc65cfb29c9fdd1ff5b1dfba174))
    - Release cargo-smart-release v0.2.2 ([`79a43c9`](https://github.com/Byron/cargo-smart-release/commit/79a43c9e9bf9630bd76944e9c25a7517f0992103))
    - Release cargo-smart-release v0.2.1 ([`72d4613`](https://github.com/Byron/cargo-smart-release/commit/72d4613262c001d2aa1a5ececdf4ec35f90b8b30))
    - [smart-release #155] Another note ([`a1eca1d`](https://github.com/Byron/cargo-smart-release/commit/a1eca1dfccfb2ec0f549acf4634d5e6ece4bd3e7))
    - [smart-release #155] how to increase version numbers ([`79613c0`](https://github.com/Byron/cargo-smart-release/commit/79613c0c264d231202d64d2acce339bdf52c2e11))
    - Release cargo-smart-release v0.2.0 ([`be09a0e`](https://github.com/Byron/cargo-smart-release/commit/be09a0e29328dc97023b45d767b7a1844057985f))
    - [smart-release #155] keep dependency versions by default ([`c37cdb6`](https://github.com/Byron/cargo-smart-release/commit/c37cdb60a8a03bdb106e6420cd292c5e8cdd41c7))
    - [smart-release #155] fix bug :D ([`2231523`](https://github.com/Byron/cargo-smart-release/commit/2231523efc217bfd7b9d432b1f694839c09b7980))
    - [smart-release #155] workflow notes and inversion of flag for comfort ([`db48d23`](https://github.com/Byron/cargo-smart-release/commit/db48d23ef1924dee4bfe7034376bf4276e344c1f))
    - Thanks clippy ([`defc689`](https://github.com/Byron/cargo-smart-release/commit/defc689daf6bf7b6a7f1f3a857e91ee4576b91ea))
    - [smart-release #155] inform about latest features ([`cf2138f`](https://github.com/Byron/cargo-smart-release/commit/cf2138ff9943b6ec249775d832985e45d98ff844))
    - [smart-release #155] refactor ([`8e8dc1d`](https://github.com/Byron/cargo-smart-release/commit/8e8dc1d53437df1349d84c8d6a62dd5da5385b36))
    - [smart-release #155] prepare release ([`0501dc1`](https://github.com/Byron/cargo-smart-release/commit/0501dc147b8aa9e89ce38d6ad6616d362ce3882d))
    - [smart-release #155] even smarter bumping ([`b337224`](https://github.com/Byron/cargo-smart-release/commit/b337224112be54d7cb9ada7fe3ad599250cf4f8a))
    - [smart-release #155] --bump-dependencies only ([`173d737`](https://github.com/Byron/cargo-smart-release/commit/173d7378f8b34a0b2b1156205afd793cb4003420))
    - [smart-release #155] incorporate crates-index for additional version check ([`81de864`](https://github.com/Byron/cargo-smart-release/commit/81de8640bb6fa843315c4f9e8ce3f94f8366117b))
    - [smart-release #155] prepare for crates-index; refactor ([`5670924`](https://github.com/Byron/cargo-smart-release/commit/5670924a0571abedeecb112e6204fbe9868d0cf4))
    - [smart-release #155] make it an actual depth-first traversal :D ([`349ef3a`](https://github.com/Byron/cargo-smart-release/commit/349ef3ab8761d54af87a3cbea0aa2fa4f827e16a))
    - [smart-release #155] sanity check for dry-run/no-dry-run-cargo-publish ([`3d89d39`](https://github.com/Byron/cargo-smart-release/commit/3d89d39ce663e6f21acedaadbb421808c6c011f3))
    - [smart-release #155] update README, add changelog ([`283c6f1`](https://github.com/Byron/cargo-smart-release/commit/283c6f100777547ad09272ceec3648d1193c86bb))
    - Thanks clippy ([`4e249c2`](https://github.com/Byron/cargo-smart-release/commit/4e249c297fb824ef89635ea4882297d3a6210e0f))
    - [smart-release #155] graceful handling of unspecified crate to publish ([`56eb633`](https://github.com/Byron/cargo-smart-release/commit/56eb633639a101f051906f1772a26d380750e770))
    - [smart-release #155] rely only on cargo metadata for root paths ([`e8e452e`](https://github.com/Byron/cargo-smart-release/commit/e8e452eb38e0285ed42c49ea84cdb01a7589cd68))
    - [smart-release #155] also ignore provided crate names if they didn't change ([`eccab32`](https://github.com/Byron/cargo-smart-release/commit/eccab323992efee858083cb8ca515d8ceea2fe9c))
    - [smart-release #155] gracefully fail when encountering unknown comparators ([`2ad38b9`](https://github.com/Byron/cargo-smart-release/commit/2ad38b9d5b068307d6a1564b91d2720135010872))
    - [smart-release #155] don't set versions if the new ones match ([`65fbe63`](https://github.com/Byron/cargo-smart-release/commit/65fbe634187f16252605d8ed0e0419aecba63035))
    - [smart-release #155] refactor ([`943d241`](https://github.com/Byron/cargo-smart-release/commit/943d241c76177ec19381fd71bdc4f0c89f374f45))
    - [smart-release #155] remove dia-semver ([`bd92266`](https://github.com/Byron/cargo-smart-release/commit/bd92266a4e34f13004a3788361e5bc790f7c5b68))
    - [smart-release #155] don't set versions where there are none when fixing manifests ([`577999e`](https://github.com/Byron/cargo-smart-release/commit/577999e7bf1e6c1aea7ed449cb30718a5d725be9))
    - [smart-release #155] also find renamed dependencies when updating versions ([`ee99c1c`](https://github.com/Byron/cargo-smart-release/commit/ee99c1c876674b04bb3d13f2a04a73589b10c81a))
    - [smart-release #155] a note ([`8408641`](https://github.com/Byron/cargo-smart-release/commit/8408641352080e5ba6212c221517b1305e8d380c))
    - [smart-release #155] invert meaning of cargo-publish dryrun flag ([`d325d2b`](https://github.com/Byron/cargo-smart-release/commit/d325d2b5791c1069ed61fc1275353d815c71d181))
    - [smart-release #155] allow dry-running cargo publish, too… ([`e3ae915`](https://github.com/Byron/cargo-smart-release/commit/e3ae915dba62a5e8ab3f352a717cf0a0a052bf32))
    - [smart-release #155] allow dry-running cargo-publish, too ([`63045a0`](https://github.com/Byron/cargo-smart-release/commit/63045a0aeeb5b7e499e17b95529e6e366c6e6d84))
    - [smart-release #155] Flag to auto-publish dependent stable crates as well ([`8916cd5`](https://github.com/Byron/cargo-smart-release/commit/8916cd58f7a255876d54f9982b144ff77f906c8e))
    - [smart-release #155] don't auto-add stable crates but suggest to do something about it ([`71efbfc`](https://github.com/Byron/cargo-smart-release/commit/71efbfc7be3614108af6a7109c638a28f7e059c2))
    - [smart-release #155] refactor ([`df1bf66`](https://github.com/Byron/cargo-smart-release/commit/df1bf66c6bc4914a2ad5580d040f1f9dc6948e9e))
    - Thanks clippy ([`7d7a89c`](https://github.com/Byron/cargo-smart-release/commit/7d7a89c7b90b4bc47207fcfef43e15c3c19cabd8))
    - [smart-release #155] refactor ([`27827c3`](https://github.com/Byron/cargo-smart-release/commit/27827c3226ffe1392dbea8bd21f2a3dcf4c4c11e))
    - [smart-release #155] don't rely on cargo resolution order for cyclic case/publish groups ([`f9cb14f`](https://github.com/Byron/cargo-smart-release/commit/f9cb14f1fc6d667ec10c70c0d719ea5e032d6ba6))
    - [smart-release #155] avoid using cargo resolution order ([`1fdc475`](https://github.com/Byron/cargo-smart-release/commit/1fdc475908b8d92da024afb2417ea28c43472775))
    - [smart-release #155] properly handle multi-crate dependencies (if there is no cycle) ([`016c4f4`](https://github.com/Byron/cargo-smart-release/commit/016c4f4da2c83833b29e40e64ee4cf44309c30c4))
    - [smart-release #155] trust our own resolution order more… ([`514301c`](https://github.com/Byron/cargo-smart-release/commit/514301c6d231d79e71e72dfdcaf4adce2d7a44a3))
    - [smart-release #155] refactor ([`f03b79a`](https://github.com/Byron/cargo-smart-release/commit/f03b79ae01d69b173ae3de1d6618fa550446d370))
    - [smart-release #155] don't check cycles on dependencies without version ([`b68a1f8`](https://github.com/Byron/cargo-smart-release/commit/b68a1f8995ac3f67971b3fb56d1bb8f4c26a753c))
    - [smart-release #155] refactor ([`00a364b`](https://github.com/Byron/cargo-smart-release/commit/00a364bbaf49bb636bd3cc842e7c13feffa1bccd))
    - [smart-release #155] refactor ([`56c04b3`](https://github.com/Byron/cargo-smart-release/commit/56c04b32c20b504d409c3b1526ce584e1ff6a329))
    - [smart-release #155] refactor ([`c610e0f`](https://github.com/Byron/cargo-smart-release/commit/c610e0ffc9f135731a9d88cce9edf80ca3ab3c3c))
    - Remove dev-dependency cycles by removing their version ([`ece1d05`](https://github.com/Byron/cargo-smart-release/commit/ece1d055fa341512cec42270f2e8c07cd878bb5f))
    - [smart-release #155] prepare release ([`fe3cb77`](https://github.com/Byron/cargo-smart-release/commit/fe3cb772a8761bfabcd17ff1ec614bb72b0bdd93))
    - [smart-release #155] cargo compatibility ([`c3982a7`](https://github.com/Byron/cargo-smart-release/commit/c3982a7cecf7261391ee32a86c9d14c2a3d803ef))
    - [smart-release #155] add readme ([`5534a5c`](https://github.com/Byron/cargo-smart-release/commit/5534a5c24b08e9ffa8f0da6dcc50840bc271c734))
    - [smart-release #155] --skip-tag flag ([`2420c87`](https://github.com/Byron/cargo-smart-release/commit/2420c87db321e8b6ce695e6f241de650697c8eca))
    - [smart-release #155] --bump option ([`c57f2d0`](https://github.com/Byron/cargo-smart-release/commit/c57f2d0da7fba779390bb1116a020e626a8615b1))
    - [smart-release #155] remove subcommands ([`2f84447`](https://github.com/Byron/cargo-smart-release/commit/2f8444794ad214f6f7ec81118bdfa076edba4826))
    - [smart-release #155] rename from 'utils' ([`035224d`](https://github.com/Byron/cargo-smart-release/commit/035224ddabf75d6d14c65237f28c8bf426e16735))
</details>

## 0.8.0 (2022-01-23)

<csr-id-a3caf3938bf0f1cea1bee0f55c082062dd250bed/>

### Chore

 - <csr-id-a3caf3938bf0f1cea1bee0f55c082062dd250bed/> upgrade all dependencies

### New Features

 - <csr-id-51d1c686763b4c036ec2c3c15d7c3ebb48e208de/> highlight (non-fatal) errors when losslessly parsing changelogs
 - <csr-id-4843b7bdcb1b05e2b99e199e168665be07123846/> Commit statistics reveal the days passes between releases

### Bug Fixes

 - <csr-id-9c1e38bfcffea372f06c78a44b2abc2284b7a87e/> more prominent message if 'bat' wasn't found in PATH

### Changed (BREAKING)

 - <csr-id-c4184f3c31ffc4597bd089e8140653906a6594d8/> Remove easy::borrow::Error entirely; support for multiple objects per handle
   This massive simplification finally allows any amounts of objects to be
   created while adding support for reusing their data buffers thanks
   to a simple free-list stored with the handle.
 - <csr-id-880b56426859306aa30038ff35e2ad14607e9e90/> rename `easy::Object` to `OwnedObject`; remove `Ref` suffix from `ObjectRef` and `TreeRef`

### New Features (BREAKING)

 - <csr-id-15e60b2d80e4452a316d14f938583b23fb9e17e6/> upgrade to crates-index 0.18
   It now assumes that the crates-index must exist, which might not always
   be the case and rightfully so. Now we wrap it to get back to the
   original behavior.

## 0.7.0 (2021-11-29)

### Bug Fixes

 - <csr-id-f4421d83d022a56e47f534a8c676bcb9cb3d230d/> don't mistake prefixed tags for versions
   Previously we would be too generous when accepting version tags, now
   we accept the prefixes 'v' and 'vers' and no prefix at all.
 - <csr-id-6eae7f1119e2a7928286f233fc397b92274bb0ab/> don't panic if there is a version requirement without version
 - <csr-id-b12b76c93db43044d6976ae218c11a8f3f3cd81d/> don't claim missing user edits if there are some

## 0.6.0 (2021-11-16)

<csr-id-82075e8a101adb2fda0c11e6567e2148d2e66b8f/>

### Other

 - <csr-id-82075e8a101adb2fda0c11e6567e2148d2e66b8f/> try to auto-update crates index with lifetime craziness
   Even though it could work, it's too complicated.

### New Features

 - <csr-id-aafb0550222aab97b52c8d716c506709b6720d3f/> auto-update crates-index if there is an indication
   There is the possibility of false-positives triggering such an update
   if manifests are edited by hand, which is not the common case.
   
   If it is, please let us know.
 - <csr-id-a4a53765952729d4ad59d8adcd3ce66c4c71589f/> 'changelog' understands '-e/--execute' as well.
   This makes writing changelogs before release easier as the command-line
   has to change less.

### Bug Fixes

 - <csr-id-57a50a68313cee4c63b1c32f3dedb2837bb751fc/> Don't let dev-dependencies participate in traversal unless they have a version specified.
   This prevents safety bumps due to breaking changes in dev dependencies,
   which are generally ignored if there is no version specified.

## 0.5.6 (2021-10-20)

### Bug Fixes

 - <csr-id-ff2c07acea56eeed679dfbe59b5ab1d4baa45d42/> nicer previews thanks to added newline

## 0.5.5 (2021-10-20)

The `v` prefix is not enforced anymore and is handled depending on what's already present.

This helps to handle changelogs with slightly different styles as well.

### New Features

 - <csr-id-3613a95d730d0aeef87d9c256f93bd528d4945bb/> Support for lack of prefixes in version headers.
   
   These are also inherited so once set by a single versioned release
   section, fully generated sections will inherit their prefix from
   that one.

### Bug Fixes

 - <csr-id-9d0d1fd71196b129b229a7d9475fdd6b99e8675b/> Assume manifests cannot necessarily be read by `cargo_toml::Manifest` and fallback.
   
   This prevents errors to occur in some configurations when no crate is specified on the command-line.

## v0.5.4 (2021-10-20)

### Bug Fixes

 - <csr-id-77f433e806e43c8d355b3e176ed740ba4de9777c/> create github release only after tags were created and pushed

## v0.5.3 (2021-10-20)

### Bug Fixes

 - <csr-id-a3aaa3e0fa38085530bc20443de176306fc8d5d2/> strip `.git` suffix from repository paths when using it in urls
 - <csr-id-53ee1a751e5d79aa3e325a5fd3c3a211fc3d06a1/> remove extra '/' after https://github.com/ based URLs

## v0.5.2 (2021-10-19)

Releases will be more atomic and it will try hard to complete all pending operations even in the light
of failure. Now GitHub releases will be created right after a publish succeeded, and tags will be pushed
for all successful publishes.

### New Features

 - <csr-id-db3cb11c466fff57f3f272d7269dc95a636e1c1f/> Add `-d` short flag for `--allow-dirty` in `changelog`

### Bug Fixes

 - <csr-id-8c3ca9cf58c44af627fc9b3c4138891635b1c554/> Push all available tags even if an error occurred.
   
   That way, tags don't remain unpushed despite having been created
   successfully, just because one crate later in the publishing
   process fails.
 - <csr-id-b769c47079a16042ef592a0199cb2d0f6afeeb5e/> Create GitHub release right after publishing succeeds.
   
   This is more atomic and prevents loosing all github releases if one
   publish fails later on.
 - <csr-id-ae8570050a313457bb2fd6659e31f34fd29bc325/> `src/` dir of root packages is only used if there is multiple workspace members.
   
   Otherwise one part of the dependency resolver might have concluded that there are changes, while another part would not have.
   The outcome would have been the same, but the messaging around it would have been different unnecessarily.

## v0.5.1 (2021-10-19)

This release contains an important bugfix which may have caused panics when the root-package didn't have changes.

### New Features

 - <csr-id-ed8abfdac40f5c8b17981b8a990572f6f07c8862/> `changelog` subcommand fails if there is nothing to do

### Bug Fixes

 - <csr-id-ce68733379a8ab4644c849ba1571bc7063962c64/> Fix panic due to unexpected internal state.
   
   When there was no change in the src/ directory of the top-level crate,
   the dependency resolution would not be able to auto-bump the version
   as no change occurred, but another part would usually detect a change
   as it wasn't confined to the top-level src/ directory.
   
   This could lead to a panic as an invariant wasn't upheld.
   
   This was fixed by letting both parts agree to use the src/ directory
   to determine changes of the top-level directory, and by making panics
   impossible while improving the messaging around this state should it
   still occur. The latter is rough, probably rare, but usable.
 - <csr-id-6ee4f5d20c832a54ca5d841773d93f0927a16f25/> Correct the reporting of manifest changes.
   
   Previously even unchanged crates would trigger workspace crates
   to be recorded for manifest changes.
   
   Now only crates that are to receive manifest changes will be triggering
   this.

## v0.5.0 (2021-10-19)

<csr-id-07372dd045de88f283d35d8f3dcc4c079dce88e9/>
<csr-id-3519f9a1f4002232aec752dadf7d3737bd97ce3d/>

A release with breaking changes as the dependency engine was reworked to handle even more cases
and make future improvements easier.

### Other

 - <csr-id-3519f9a1f4002232aec752dadf7d3737bd97ce3d/> try to assure that breaking changes are always published in correct order
   The problem here is that even though we can turn non-publishable breaks
   into publishable ones without loosing information, they will not be in
   the correct order.
   
   The solution is to merge dependency trees instead of clearing them with
   weird logic.

### New Features

 - <csr-id-6d4edfa3b2d2c6700e0956716a575831b940cb50/> Respect `publish=false` in cargo manifest
 - <csr-id-7648bf3c7554352bec8e1355f9b593d891b2b17f/> Perform safety bumps without forcing a publish.
   
   This is what's required to assure that future publishes of such
   transitively dependent crates won't cause downstream breakage the next time the tool is run.
 - <csr-id-b806a9c982da1e5ff42c268e430c67363f3a7918/> Inform about safety bumps more explicitly,
   and generally greatly improve the way the course of action is described.

### Bug Fixes

 - <csr-id-501c1d102c0e5e4635120bb1aa857e97a2b537b4/> Dependency resolution.
   
   Previously the ordering of crates for release might not have been
   correct due to this issue that is now fixed.
   
   We need depth-first traversals and previously it would extend skipped
   dependencies, effectively putting them into their own ordering.
   
   Previously it would restore that ordering, but not anymore, causing
   this bug that was entirely unnecessary.
 - <csr-id-5e98e5559707cf308e2cd64494fe73a99f9e9c8e/> `--no-changelog` during smart-release is now actually working
   
   Previously the flag had no effect and changelogs would always be
   generated, possibly stopping the release as at least one of them
   needed manual work.
 - <csr-id-dfc588b25ede3faa578eb8e131e73c857117a6df/> Pin version of clap to beta 5.
   
   This assures we don't get broken automatically in future.
   Previously that wasn't possible as the dependency of `clap`,
   `clap-derive` was also using a beta version and wasn't constrained,
   hence it would be updated and cause breaking changes with pinned
   versions of consumers of `clap`.
 - <csr-id-fb6b909e49d8428e53da6e2ce3c2f878025e00f7/> ! breaking changes cause intermediate (otherwise skipped) crates to be published.
   This assures that about-to-be-released crates that have breaking changes
   anywhere in their dependency graph will cause all crates leading up to,
   and including, a breaking change to be published as well.

### Changed (BREAKING)

<csr-id-2f87196217a6e685dc447b4af091842926aed6d0/>

 - <csr-id-59302ae24db791988c22322c2c1ad72e2918f89a/> `changelog` subcommand inverts `--dependencies` to `--no-dependencies`
 - Remove `--no-multi-crate-release` support entirely
  
   As the default is to do multi-crate releases and now having to deal
   with single-create releases reduces maintenance burden.

   The solution to this problem is to not specify version constraints in
   dev-dependencies to workspace crates.

   We also don't check for this anymore, which might be re-added
   at some point if there is demand.This makes dependency resolution similar to cargo smart-release by default and is less surprising.

## v0.4.0 (2021-10-15)

<csr-id-3c0a6389fe5ff981dadca20e8a4a4a0d2ef66e13/>
<csr-id-77ed17c703e502e132cda9a94eb8c63db0b627ad/>
<csr-id-1cb41f81cffe19c75aadf49a5cc7ec390ec6cae7/>
<csr-id-ae8780e08303946412cedc19ea4d2679be49ec97/>
<csr-id-509550f8aa8210f3688c78167a56a21fc1817515/>
<csr-id-11b64fce4630371633b6415f227eecdc6b42b20b/>
<csr-id-0ebfeb614264ca06ab763189e55e6c016c9997af/>
<csr-id-80b8331092f4856f52afa1d85fa375ae688bdd28/>
<csr-id-e59f901f47fb0180211494a1591aed62b856406a/>
<csr-id-19fc134d2a34f2ea84b2cc8fbd15ca55c55df35e/>
<csr-id-e668bf23ddba9a676a885f1f401d2d2885784eef/>
<csr-id-8fe461281842b58aa11437445637c6e587bedd63/>
<csr-id-e16603b15b5488b81563c583cd8f5292ab9d24a2/>
<csr-id-fb750b65ca64c894ffb79cd0049f10a8db255ab6/>
<csr-id-f6f2d1b2c1c50d36ee046ed58ffffed0444cd25a/>
<csr-id-a040f7d882eb5f6db0d54ba7e32437da3579a075/>
<csr-id-9b78c344ee287c4c2908ccbe64bd64c2c9648459/>
<csr-id-b1a39046056bf4a862cebe69f44f3ea1e53a2069/>
<csr-id-ecf38b8c013e46a33aa0b2c1b4e9cf547c8393c4/>
<csr-id-342b443a4f49736a10c2b311d69841dbf581ceec/>
<csr-id-0d30094f4d397f932288f8c04ffd01f956113dc8/>
<csr-id-a56bd7b134d315e22e5c8d01ca2d927de75955a9/>
<csr-id-c50704a0595884c3fb20629aba0f22bf99893cbf/>
<csr-id-681d743e5579197d7262c40237dda0116fc4af1c/>
<csr-id-798b650ad848001b10018087ed6c5d8a4055ece8/>
<csr-id-7ca029c73eee51302d6828c6f9e8862d3fd4fbd4/>
<csr-id-73794a4e382404cb7b684c9054278fb4ff8a84ce/>
<csr-id-d1145d1a6219ddafa7a41c82d6149b289f033640/>
<csr-id-443f000015de2117eae08fedf7d23f0d1ac6abff/>
<csr-id-0c355ed24eb230e9834e797d5c8dc72ae21f0c46/>
<csr-id-5fc33266b2626a07b19d2f5bd075e2c600204a3d/>
<csr-id-17322fa378fdecad80ad1349292aaaee8bcd00f6/>
<csr-id-ac0696b8226a1478fa90b932306f35e5dbf464b1/>
<csr-id-87ebacc65f56f8765eb787fea1bd27f2c99dfd97/>
<csr-id-41afad3386461b658ee859225785b6de86d13cfb/>
<csr-id-ae7def47388aeb56c7df4a73fd13ff508cee7017/>
<csr-id-fbf267eeb424bf90649be278ee847fe3f2a3db80/>
<csr-id-d422b9a31a37a03551bec4382039aaf3a7e49902/>
<csr-id-e7c061b10c263001eb4abf03098d6694b770f828/>
<csr-id-66292fd1076c2c9db4694c5ded09799a0be11a03/>
<csr-id-06996e032b1e451a674395ebaca94434fac46f05/>
<csr-id-422701be4ed6d2a61361af9b6eb0f4f470d1d782/>
<csr-id-daec7167df524b329daad7dabb1b9920b6ef8936/>
<csr-id-debe0094826f83839f907523715def929133fd58/>
<csr-id-56e39fac54bfa3871c42bbf76a9f7c49486b85be/>
<csr-id-1954b467cf1e97e22629c55487b4a66cb1380a89/>
<csr-id-9062a472ac63887900562ed341c7b68665b8587a/>
<csr-id-293bfc0278c5983c0beaec93253fb51f00d81156/>
<csr-id-650241251a420602f74037babfc24c9f64df78d8/>
<csr-id-2b4a61589a7cba3f7600710e21304e731ae3b36a/>
<csr-id-72e175209441b12f3d4630e5118e21a3156146df/>
<csr-id-90e6128727932f917c485f411e623fc6a9c2ad4d/>
<csr-id-ff894e5b0257722c31578772ed694324194c0741/>
<csr-id-78d31d9de2710b4369862c1226f18d4a2d79a9c4/>
<csr-id-0e02831fff83f6d6b0ea8889d54196e54e4e4aff/>
<csr-id-d66c5aea01a7d1df2cc539c52b789ad39a058ad2/>
<csr-id-d4ffb4f2ac935f6345bdc7d03cc1878007609503/>
<csr-id-9fc15f92ddec4ccfd0803d2b1231ed08d424cf33/>
<csr-id-9e430df135e87ee9e9673e7d52f072f39abaf4d9/>
<csr-id-a33dd5d21039441556ab89c997195f1bcc5bc543/>
<csr-id-1a683a91a2850d663cf87fb326e5ab66ae86fc96/>
<csr-id-3677b782f8bc63a38d4d49b8555b5a6b9a618f84/>
<csr-id-cdf41998360527161a1b04821bab377489f6c5f0/>

This major release adds **changelog** support to automatically generate scaffolding to be filled in by hand. The feature is driven by
[conventional commit](https://www.conventionalcommits.org) messages which are used sparingly to mark important changes only.
Furthermore, it will deduce the require version bump, i.e. patch, minor or major, automatically by looking at the commit history
and interpreting _'conventional commit'_ messages. This means that from time to time one would sprinkle in a specifically formatted
commit message to leave enough information to determine the correct release version and create changelog scaffolding.

If you have 10 minutes, the following video gives the whirlwind tour through the new features (_and note that all issues discovered there
have been fixed :)_).

[![12 minute introduction video](https://img.youtube.com/vi/EOft_uMDVYE/0.jpg)](https://www.youtube.com/watch?v=EOft_uMDVYE)

If you have 30 minutes, there is also [a long version of the video](https://youtu.be/a4CzzxJ7ecE).

And there is another one showing `cargo smart-release` releasing `gitoxide 0.9.0`, along with some explanation on how it works. 

[![8 minute video releasing gitoxide](https://img.youtube.com/vi/ZS9fwPDYLpI/0.jpg)](https://www.youtube.com/watch?v=ZS9fwPDYLpI)

### Refactor

 - <csr-id-8fe461281842b58aa11437445637c6e587bedd63/> split data::output::count::objects into files

### Other

 - <csr-id-e16603b15b5488b81563c583cd8f5292ab9d24a2/> :remote_url() is now optional
   Otherwise it wouldn't work on repos that don't have a remote set yet.
   Instead of failing, we don't create links.
 - <csr-id-fb750b65ca64c894ffb79cd0049f10a8db255ab6/> assure the current package version is actually breaking
 - <csr-id-f6f2d1b2c1c50d36ee046ed58ffffed0444cd25a/> better verbosity handling when comparing to crates-index
 - <csr-id-a040f7d882eb5f6db0d54ba7e32437da3579a075/> turn off safety bump with its own flag
 - <csr-id-9b78c344ee287c4c2908ccbe64bd64c2c9648459/> improved safety bump log message
 - <csr-id-b1a39046056bf4a862cebe69f44f3ea1e53a2069/> commit message reveals safety bumps
 - <csr-id-ecf38b8c013e46a33aa0b2c1b4e9cf547c8393c4/> released crates only receive minor bumps…
   …which signals a change while allowing dependents to pin themselves to
   patch updates only.
   
   This would be users of "unstable" git-repository features for example.
   which then also don't want to see new minor versions automatically
   as it may cause breakage.
 - <csr-id-342b443a4f49736a10c2b311d69841dbf581ceec/> update changelog
 - <csr-id-0d30094f4d397f932288f8c04ffd01f956113dc8/> way more tests to nail current log output
   This is the basis for adjusting the output verbosity or information
   where it matters.
 - <csr-id-a56bd7b134d315e22e5c8d01ca2d927de75955a9/> dependency upgrade works
 - <csr-id-c50704a0595884c3fb20629aba0f22bf99893cbf/> calculate new version of dependent
 - <csr-id-681d743e5579197d7262c40237dda0116fc4af1c/> don't claim "conservative" updates for major version change
 - <csr-id-798b650ad848001b10018087ed6c5d8a4055ece8/> assure we can find non-sequential connections
 - <csr-id-7ca029c73eee51302d6828c6f9e8862d3fd4fbd4/> all logic to calculate dependent version bumps
 - <csr-id-73794a4e382404cb7b684c9054278fb4ff8a84ce/> an algorithm to collect dependencies by 'growing'
 - <csr-id-d1145d1a6219ddafa7a41c82d6149b289f033640/> foundation for bumping versions
   The idea is that the dependency traversal may also produce a new version
   number, which is when it will naturally be set for all dependents later.
 - <csr-id-443f000015de2117eae08fedf7d23f0d1ac6abff/> 
 - <csr-id-0c355ed24eb230e9834e797d5c8dc72ae21f0c46/> add git-conventional
 - <csr-id-5fc33266b2626a07b19d2f5bd075e2c600204a3d/> consider nom for custom parsing, but…
   …realize that the easiest way is definitely the excellent
   git-conventional crate.
   
   This also means we have to stop specifying crates in commit messages
   or find another way to do that.
 - <csr-id-17322fa378fdecad80ad1349292aaaee8bcd00f6/> refactor
 - <csr-id-ac0696b8226a1478fa90b932306f35e5dbf464b1/> refactor
 - <csr-id-87ebacc65f56f8765eb787fea1bd27f2c99dfd97/> refactor
 - <csr-id-41afad3386461b658ee859225785b6de86d13cfb/> a seemingly slow version of path lookup, but…
   …in debug mode it's faster than the fast path, despite doing more
   and being the same when it comes to searching path components.
 - <csr-id-ae7def47388aeb56c7df4a73fd13ff508cee7017/> fast filter by single-component path
 - <csr-id-fbf267eeb424bf90649be278ee847fe3f2a3db80/> prepare for fast lookup of paths
 - <csr-id-d422b9a31a37a03551bec4382039aaf3a7e49902/> configure caches with env vars using `apply_environment()`
 - <csr-id-e7c061b10c263001eb4abf03098d6694b770f828/> refactor
 - <csr-id-66292fd1076c2c9db4694c5ded09799a0be11a03/> set package cache via RepositoryAccessExt
 - <csr-id-06996e032b1e451a674395ebaca94434fac46f05/> object-cache to allow for a speed boost…
   …by avoiding duplicate accesses to hit the object database.
   However, the cost for the cache are relatively high and involve some
   memory copying, so hit rates of about 50% is certainly what is needed
   to get any speed boost at all.
 - <csr-id-422701be4ed6d2a61361af9b6eb0f4f470d1d782/> actually build the segment vec, without pruning for now
 - <csr-id-daec7167df524b329daad7dabb1b9920b6ef8936/> build commit history for later use in changelog generation
 - <csr-id-debe0094826f83839f907523715def929133fd58/> sketch history acquisition
 - <csr-id-56e39fac54bfa3871c42bbf76a9f7c49486b85be/> add 'Head::peeled()' method
 - <csr-id-1954b467cf1e97e22629c55487b4a66cb1380a89/> some performance logging
 - <csr-id-9062a472ac63887900562ed341c7b68665b8587a/> build ref lookup table
 - <csr-id-293bfc0278c5983c0beaec93253fb51f00d81156/> loose reference iteration with non-dir prefixes…
   Previously it was expected for the prefix `Path` to always exist for
   the prefix to be valid. This, however, is not similar to packed
   prefixes, which allow non-dir prefixes as well.
   
   Now we will check if the prefix is actually a directory, and if not
   split it into its parent directory and the filename portion. The latter
   is then used for prefix matching file names within that directory.
 - <csr-id-650241251a420602f74037babfc24c9f64df78d8/> Add 'references().all().peeled().'…
   …to not only make typical usage of iterated references more convenient
   but also work around a double-borrow error one would see otherwise.
 - <csr-id-2b4a61589a7cba3f7600710e21304e731ae3b36a/> filter refs correctly, but…
   …it needs a way to peel references right away without trying
   to double-borrow. This means the Iterator needs to implement this.
 - <csr-id-72e175209441b12f3d4630e5118e21a3156146df/> find tag references by name…
   …even though it's clear that loose refs won't be found with prefixes
   that aren't directories, but contain a partial file.
   
   This is more like a bug to be fixed, as that works naturally for
   packed-refs for instance.
 - <csr-id-90e6128727932f917c485f411e623fc6a9c2ad4d/> improve changelog format
 - <csr-id-ff894e5b0257722c31578772ed694324194c0741/> sketch first step of info generation
 - <csr-id-78d31d9de2710b4369862c1226f18d4a2d79a9c4/> changelog gets crates to work on
 - <csr-id-0e02831fff83f6d6b0ea8889d54196e54e4e4aff/> handle unborn heads
 - <csr-id-d66c5aea01a7d1df2cc539c52b789ad39a058ad2/> fmt
 - <csr-id-d4ffb4f2ac935f6345bdc7d03cc1878007609503/> refactor
 - <csr-id-9fc15f92ddec4ccfd0803d2b1231ed08d424cf33/> refactor
 - <csr-id-9e430df135e87ee9e9673e7d52f072f39abaf4d9/> refactor
 - <csr-id-a33dd5d21039441556ab89c997195f1bcc5bc543/> initial test for changelog
   Which doesn't test that much.
 - <csr-id-1a683a91a2850d663cf87fb326e5ab66ae86fc96/> very basic support for changelog command…
   …which shows that it probably just wants to be separate for now before
   being integrated?
 - <csr-id-3677b782f8bc63a38d4d49b8555b5a6b9a618f84/> add 'cargo changelog' sub-command binary
 - <csr-id-cdf41998360527161a1b04821bab377489f6c5f0/> add changelog to most tests

### Changelog Support in `cargo smart-release`

When using `cargo smart-release` in dry-run mode (_default_), additional information regarding changelog will be printed.
This informs you a release would be attempted, or if manual adjustments to the changelogs would be required, for example as
they are fully generated with statistical information only.

If there is no issue with the initial changelogs, passing the `--execute` flag will write the changelogs after
providing them to you for preview (using `bat`) for a last chance to abort the operation. Otherwise the publishing
will proceed, which includes the creation of tag objects containing the relevant section of the changelog, along with
a GitHub release which is annotated with the same section (_only if the repository is hosted on GitHub_).

If there are issues to be corrected, there will be suggestions to run `cargo changelog --write --only <crate-name>`
one by one, or the release operation will have left a single commit with all changelogs written out.
In any case, it's recommended to re-write the changelog after editing to assure it is indeed stable and won't change each time
the generator is run.

For more information, run `cargo smart-release -h`.

### The `cargo changelog` Sub-Command

This new sub-command sports the same dependency resolution as `smart-release` itself, operates in dry-run mode by default
to preview changelogs that would be written. Use the `--write` flag to actually write changes to disk.

It's primary use is to conveniently generate changelogs from time to time to add the final polish by hand before
actually releasing them along with the crate with `smart-release`.

For more information, run `cargo changelog -h`.

### Other BREAKING Changes

- renamed `--skip-*` flags to `--no-*` for consistency
- rename `--skip-dependencies` to `--no-dependencies` to be more inline with existing terminology of other flags.
- rename short name for `--execute` to `-e` from `-n` for consistency

### Other Changes

 - <csr-id-e668bf23ddba9a676a885f1f401d2d2885784eef/> `--no-dependencies` now has `--only` as alias

### Bug Fixes

 - <csr-id-11eebdcc572a72b2e66a9db3cae0a01f12a81619/> Previously it might have been possible to see that it won't use a 'new' crate version as it's already in the manifest, _even_ if these are the same. This is now fixed.

## v0.3.1 (2021-09-07)

## v0.3.0 (2021-08-27)

- add `--skip-dependencies` flag
- add `--verbose` flag and be less verbose in dry-runs by default to provide only essential information
- improvements to notification clarity

### Breaking

- Use short flag for `--no-bump-on-demand` in `--bump-dependencies`

## v0.2.4 (2021-08-15)

- Fix auto-push functionality

## v0.2.3 (2021-08-15)

- Less verbosity by default which is helpful on the first run to get an overview. Use `--verbose/-v` for all the details.
- Also push tags and HEAD by default, unless `--skip-push` is specified.

## v0.2.2 (2021-08-15)

- support for unsorted packed-refs files

## v0.2.1 (2021-08-13)

## v0.2.0 (2021-08-13)

## v0.1.0 (2021-08-13)

- initial release

