use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    process::Output,
};

use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A supported feature configuration from `package.metadata.api-check.profiles`.
///
/// Both fields are required when deserializing, and unknown fields are rejected.
/// The check functions validate feature names against the package manifest and
/// reject configurations that enable an excluded unstable feature, including
/// through aliases or default features.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeatureProfile {
    /// Whether the package's default features are enabled for this configuration.
    ///
    /// Serialized as `default-features`. When false, the checker passes
    /// `--no-default-features` to Cargo.
    #[serde(rename = "default-features")]
    pub default_features: bool,
    /// Package feature names explicitly enabled in addition to any defaults.
    ///
    /// An empty list selects no additional features. The list preserves its
    /// input order; release checks compare profile selections as sets when
    /// deciding whether a previously supported configuration was removed.
    pub features: Vec<String>,
}

/// The JSON protocol shared by release planning and API checkers.
///
/// Passed to [`release`](crate::api_check::release), or serialized on stdin to
/// `cargo api-check release`. Paths and versions serialize as strings. All
/// fields except [`baseline_version`](Self::baseline_version) are required when
/// deserializing, and unknown fields are rejected.
///
/// The current manifests still contain their working-tree dependency versions.
/// The dependency plans describe proposed releases without rewriting those
/// manifests before the check.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseRequest {
    /// Cargo package name of the workspace member to check.
    ///
    /// This is the manifest's `package.name`, which may differ from its Rust
    /// library name or a dependency alias.
    pub package: String,
    /// Manifest of the current package whose public API is checked.
    ///
    /// Relative paths are resolved against the process's current directory.
    /// The checker canonicalizes the path and requires it to identify the
    /// requested package in Cargo metadata.
    pub manifest_path: PathBuf,
    /// Published version used by the release planner to select the baseline tag.
    ///
    /// This is protocol context; [`release`](crate::api_check::release) reads
    /// baseline sources using `baseline_revision` or its explicit checkout
    /// argument, rather than using this version to select a revision.
    /// `None` is omitted when serializing and is used by local revision checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_version: Option<Version>,
    /// Local Git revision supplying the baseline sources, such as a tag or commit ID.
    ///
    /// The revision must be available to `git archive`. It is ignored when
    /// [`release`](crate::api_check::release) receives a baseline checkout override.
    pub baseline_revision: String,
    /// Feature names excluded from the checked package's supported API promise.
    ///
    /// Stable profiles must not enable these features, directly or indirectly.
    /// An empty list excludes no features. This does not remove experimental
    /// rustdoc items from the boundary policy's scrutiny when stable API refers
    /// to them.
    pub unstable_features: Vec<String>,
    /// Planned versions of all workspace packages, including transitive dependencies.
    ///
    /// The checker resolves each package's actual library name and flags planned
    /// breaking releases whose types or other items appear in the supported API.
    /// Local checks use an empty list because they have no proposed release plan.
    pub dependencies: Vec<ReleaseDependency>,
}

/// A workspace dependency's published, current, and planned versions.
///
/// Used in [`ReleaseRequest::dependencies`]. Versions serialize as semver strings;
/// all fields are required when deserializing, and unknown fields are rejected.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseDependency {
    /// Cargo package name of the workspace member, rather than its library name or alias.
    pub name: String,
    /// Version currently declared in the package's manifest.
    pub current_version: Version,
    /// Latest published version known to the planner, or the manifest version if unknown.
    ///
    /// This is the reference for deciding whether the planned version is breaking,
    /// so a manually bumped manifest does not hide a change from published users.
    pub baseline_version: Version,
    /// Version proposed by the release plan, which may equal `current_version`.
    pub next_version: Version,
    /// Whether `next_version` is a breaking increase relative to `baseline_version`.
    ///
    /// The planner supplies this flag using Cargo's version compatibility rules,
    /// including minor increases below 1.0 and patch increases below 0.1.
    /// The checker uses the supplied flag; it does not recompute it during
    /// deserialization or replace the current manifest versions with the plan.
    pub breaking: bool,
}

/// Public dependency paths and the dependency names rejected by an audit allowlist.
///
/// Returned by [`audit`](crate::api_check::audit) after reading a rustdoc JSON
/// document. A nonempty `disallowed` set is a completed policy violation;
/// malformed or unsupported documents instead produce `Err`. The library does
/// not print this result, so callers can choose their own output format.
#[derive(Debug)]
pub struct Audit {
    /// Rust library names mapped to the external item paths referenced by the public API.
    ///
    /// Names can differ from Cargo package names. Each set contains the resolved
    /// paths of referenced types, traits, or other items, rather than source-file
    /// locations. Map keys and paths are sorted and deduplicated.
    pub dependencies: BTreeMap<String, BTreeSet<String>>,
    /// Dependency library names present in `dependencies` but absent from the allowlist.
    ///
    /// Empty when no allowlist was supplied or every exposed dependency was allowed.
    /// Supplying an empty allowlist rejects every dependency found by the audit.
    /// Paths explaining each rejection are available in `dependencies`.
    pub disallowed: BTreeSet<String>,
}

/// The source used for compatibility comparison.
///
/// Recorded in [`Incompatible::baseline`]. A boundary-only audit has no baseline
/// and therefore no promise of compatibility with a previous release.
#[derive(Debug)]
pub enum Baseline {
    /// Git revision whose archived sources were compared with the working tree.
    ///
    /// The string is a resolved commit ID for [`check`](crate::api_check::check()).
    /// For [`release`](crate::api_check::release), it retains the revision supplied
    /// in [`ReleaseRequest::baseline_revision`], which can also be a tag or branch.
    Revision(
        /// Revision token used for the baseline archive, or the commit ID resolved by a local check.
        String,
    ),
    /// Canonical path to an explicitly supplied baseline workspace checkout.
    ///
    /// The directory substitutes for the current workspace root when locating
    /// the checked package's manifest. This records the supplied source directory,
    /// not one of the temporary archives created by the checker.
    Checkout(
        /// Absolute, canonical path to the supplied baseline workspace root.
        PathBuf,
    ),
}

/// All incompatibilities found by a completed check.
///
/// [`check`](crate::api_check::check()), [`boundary`](crate::api_check::boundary), and
/// [`release`](crate::api_check::release) return `Ok(Some(report))` only when at
/// least one issue was detected. `Ok(None)` means every check performed passed.
/// Tool and configuration failures instead return `Err`, because the checks
/// could not complete.
///
/// The report owns its evidence, including semver-checker output, so it remains
/// usable after temporary baseline directories and rustdoc files are removed.
///
/// ```no_run
/// use cargo_smart_release_core::api_check;
///
/// if let Some(report) = api_check::check("my-library", None, None)? {
///     for issue in &report.issues {
///         tracing::warn!(profile = ?issue.profile, reason = ?issue.reason, "Incompatible supported API");
///     }
/// }
/// # Ok::<(), anyhow::Error>(())
/// ```
#[derive(Debug)]
pub struct Incompatible {
    /// Cargo package name of the checked workspace member.
    pub package: String,
    /// Canonical path to that package's current manifest.
    pub manifest_path: PathBuf,
    /// Source used for compatibility comparison, or `None` for a current boundary audit.
    ///
    /// `None` occurs for [`boundary`](crate::api_check::boundary) and for local
    /// checks with no supported release baseline. Those reports describe policy
    /// violations in the current API, rather than changes from an earlier release.
    pub baseline: Option<Baseline>,
    /// All detected issues, with the feature profile that exposed each one.
    ///
    /// Reports returned by the checker contain at least one entry. Entries are
    /// collected in check order without deduplication across profiles or checking
    /// mechanisms, so one API change can have several supporting reasons.
    pub issues: Vec<Incompatibility>,
}

/// An incompatibility and the supported feature configuration that exposed it.
///
/// Multiple entries can describe the same underlying change under different
/// profiles or with evidence from both signature and semver checks.
#[derive(Debug)]
pub struct Incompatibility {
    /// Supported feature configuration under which the issue was detected.
    ///
    /// For [`Reason::ProfileRemoved`], this is the previously supported profile
    /// that the current manifest no longer declares.
    pub profile: FeatureProfile,
    /// Classification of the issue and its owned diagnostic evidence.
    pub reason: Reason,
}

/// Evidence for an incompatible API or a supported boundary violation.
///
/// `crate_name` fields identify Rust libraries, which may differ from the Cargo
/// package names used in release requests. Path fields identify API items, not
/// source-file locations. Different variants can describe the same underlying
/// change; callers should retain the evidence they need rather than assuming
/// one reason per changed item.
#[derive(Debug)]
pub enum Reason {
    /// A feature profile supported by the baseline is absent from the current manifest.
    ///
    /// The removed configuration is retained in [`Incompatibility::profile`].
    ProfileRemoved,
    /// Supported API refers to a type whose rustdoc marks it as experimental.
    ///
    /// Experimental markers are a documentation line starting with `**Unstable:**`
    /// or equal to `**Unstable.**`. This can arise in the checked package or while
    /// recursively inspecting a dependency's supported members.
    ExperimentalType {
        /// Rustdoc's canonical type path, or its item ID when no path summary exists.
        path: String,
    },
    /// Supported API directly exposes an experimental item from a workspace dependency.
    ///
    /// An unstable type or whole-crate exception does not make an item with an
    /// experimental rustdoc marker part of the supported contract.
    ExperimentalDependencyItem {
        /// Rust library name of the dependency defining the experimental item.
        crate_name: String,
        /// Exposed dependency item path, including its library name.
        path: String,
    },
    /// Supported API exposes an item from a pre-1.0 dependency without an exception.
    ///
    /// Neither `allowed-unstable-types` nor `allowed-unstable-crates` admits the item.
    UnlistedDependencyItem {
        /// Rust library name of the pre-1.0 dependency.
        crate_name: String,
        /// Exposed dependency item path missing a boundary policy exception.
        path: String,
    },
    /// Supported API exposes a stable dependency item outside that dependency's supported API.
    ///
    /// For example, an experimental re-export can make a type visible while
    /// keeping that export path outside the stable contract.
    UnsupportedDependencyItem {
        /// Rust library name of the dependency at version 1.0 or above.
        crate_name: String,
        /// Exposed item path that is not reachable through the dependency's supported API.
        path: String,
    },
    /// Recursive traversal of an admitted pre-1.0 item reaches an unlisted type.
    ///
    /// A named type exception does not automatically admit other types exposed
    /// by its fields or supported methods. A whole-crate exception would admit
    /// these unmarked types; experimental markers are checked separately.
    UnlistedDependencyType {
        /// Rust library name of the dependency defining the unlisted type.
        crate_name: String,
        /// Public type path, or rustdoc's canonical path or item ID if no export name exists.
        path: String,
    },
    /// A planned breaking dependency release affects items exposed by the supported API.
    BreakingDependency {
        /// Complete dependency plan, including the versions and supplied breaking flag.
        dependency: ReleaseDependency,
        /// Actual Rust library name resolved from that dependency's Cargo targets.
        crate_name: String,
        /// Exposed paths found in either the baseline or current supported API.
        paths: BTreeSet<String>,
    },
    /// An exposed dependency's package source or Cargo compatibility family changed.
    ///
    /// Identity strings combine the Cargo source with a compatibility family.
    /// Workspace sources use a workspace-relative manifest path; external path
    /// sources use their manifest path. Families distinguish major versions at
    /// 1.0 or above, minor versions under 1.0, and patch versions under 0.1.
    DependencyIdentityChanged {
        /// Rust library name of the dependency whose type identity changed.
        crate_name: String,
        /// Exposed dependency paths found in either the baseline or current API.
        paths: BTreeSet<String>,
        /// Baseline identity strings, each encoded as `<source>::<compatibility-family>`.
        before: BTreeSet<String>,
        /// Current identity strings in the same encoding as `before`.
        after: BTreeSet<String>,
    },
    /// A previously supported item disappeared or its normalized signature changed.
    ///
    /// Snapshots retain the comparison evidence, not complete rustdoc documents.
    /// Referenced item IDs and source aliases are normalized to resolved paths;
    /// documentation, source locations, and traversal-only fields are omitted.
    SignatureChanged {
        /// Supported item path or synthetic `impl <trait> for <type>` path, optionally with a member suffix.
        path: String,
        /// Baseline normalized signature, including the item kind, shape, and type information.
        ///
        /// Member signatures can also include their enclosing implementation's signature.
        before: Value,
        /// Current normalized signature, or `None` when the supported item disappeared.
        ///
        /// `None` also covers an item still present in source but removed from the
        /// supported contract, for example by marking it experimental.
        after: Option<Value>,
    },
    /// A recursively supported library in the baseline is absent from the current supported graph.
    ///
    /// This can follow removal of its exposed types or of the policy permitting
    /// recursive checking, even when the dependency itself still exists.
    SupportedDependencyRemoved {
        /// Rust library name of the previously supported library.
        crate_name: String,
    },
    /// `cargo-semver-checks` completed a comparison and reported breaking API changes.
    ///
    /// Tool execution failures produce `Err` rather than this reason. The retained
    /// output can describe changes beyond normalized signatures, such as auto
    /// traits, exhaustive enums, or construction of public types.
    SemverCheck {
        /// Rust library name whose supported rustdoc documents were compared.
        crate_name: String,
        /// Exit status and exact stdout/stderr bytes from the completed comparison.
        ///
        /// The status has exit code 100 for the supported semver-checker protocol.
        /// Output is retained without decoding, truncation, or loss of non-UTF-8 bytes;
        /// callers can decode it for display while preserving the original evidence.
        output: Output,
    },
}

impl Incompatible {
    pub(super) fn record(&mut self, profile: &FeatureProfile, reasons: impl IntoIterator<Item = Reason>) {
        self.issues.extend(reasons.into_iter().map(|reason| Incompatibility {
            profile: profile.clone(),
            reason,
        }));
    }

    pub(super) fn into_option(self) -> Option<Self> {
        (!self.issues.is_empty()).then_some(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn release_requests_preserve_the_protocol_and_reject_invalid_types() -> anyhow::Result<()> {
        let value = json!({
            "package": "api", "manifest_path": "api/Cargo.toml", "baseline_version": "1.2.3",
            "baseline_revision": "api-v1.2.3", "unstable_features": [],
            "dependencies": [{"name": "dep", "current_version": "0.2.0", "baseline_version": "0.2.0",
                "next_version": "0.3.0", "breaking": true}]
        });
        let request: ReleaseRequest = serde_json::from_value(value.clone())?;
        assert_eq!(request.dependencies[0].next_version, Version::new(0, 3, 0));
        assert_eq!(serde_json::to_value(request)?, value);
        for (field, invalid) in [
            ("next_version", json!("invalid")),
            ("breaking", json!("true")),
            ("name", json!(null)),
        ] {
            let mut invalid_request = value.clone();
            invalid_request["dependencies"][0][field] = invalid;
            assert!(
                serde_json::from_value::<ReleaseRequest>(invalid_request).is_err(),
                "{field}"
            );
        }
        let mut unknown = value;
        unknown["baseline_revison"] = json!("typo");
        assert!(serde_json::from_value::<ReleaseRequest>(unknown).is_err());
        for invalid in [
            json!({"default-features": "false", "features": []}),
            json!({"default-features": false, "features": [42]}),
        ] {
            assert!(serde_json::from_value::<FeatureProfile>(invalid).is_err());
        }
        Ok(())
    }
}
