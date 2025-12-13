//! Source collector for copying dependency sources to a local directory.
//!
//! This module provides functionality to collect dependency source code
//! from `~/.cargo/registry/src/` into a local `.source_{timestamp}/` directory.

use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env as StdEnv, fs as StdFs};

use cargo_metadata::{DependencyKind, Metadata, MetadataCommand, PackageId};
use serde_json as SJSON;

use crate::error::Error;

/// Metadata about a collected crate.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CollectedCrate {
    /// Crate name.
    pub name: String,

    /// Crate version.
    pub version: String,

    /// Rust edition.
    pub edition: String,

    /// Enabled features.
    pub features: Vec<String>,

    /// Crate description.
    pub description: Option<String>,

    /// Relative path within the .source_*/ directory.
    pub source_path: String,
}

/// Manifest stored in `.source_*/manifest.json`.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SourceManifest {
    /// When the sources were collected.
    pub collected_at: String,

    /// Workspace root path.
    pub workspace_root: String,

    /// Collected crates by key "{name}-{version}".
    pub crates: HashMap<String, CollectedCrate>,
}

/// Result of a collection operation.
#[derive(Debug)]
pub struct CollectionResult {
    /// Path to the created .source_*/ directory.
    pub output_dir: PathBuf,

    /// Number of crates collected.
    pub crates_collected: usize,

    /// Crates that were skipped (not found in registry).
    pub skipped: Vec<String>,
}

/// Options for source collection.
#[derive(Debug, Default)]
pub struct CollectOptions {
    /// Include dev-dependencies.
    pub include_dev: bool,

    /// Custom output directory (overrides timestamp-based naming).
    pub output: Option<PathBuf>,

    /// Dry run - don't actually copy files.
    pub dry_run: bool,

    /// Only copy `src/` directory and `Cargo.toml` (minimal mode).
    ///
    /// By default (false), the entire crate directory is copied to ensure
    /// all source files are available (including `build.rs`, modules outside
    /// `src/`, etc.).
    pub minimal_sources: bool,

    /// Skip adding `.source_*` pattern to `.gitignore`.
    pub no_gitignore: bool,
}

/// Collector for gathering dependency sources.
#[derive(Debug)]
pub struct SourceCollector {
    /// Cargo metadata for the workspace.
    metadata: Metadata,

    /// Path to cargo registry sources.
    registry_path: PathBuf,
}

impl SourceCollector {
    /// Create a new collector for the current directory.
    ///
    /// # Errors
    ///
    /// Returns an error if cargo metadata cannot be loaded.
    pub fn new() -> Result<Self, Error> {
        Self::from_manifest(None)
    }

    /// Create a new collector from a specific manifest path.
    ///
    /// # Errors
    ///
    /// Returns an error if cargo metadata cannot be loaded.
    pub fn from_manifest(manifest_path: Option<&Path>) -> Result<Self, Error> {
        let mut cmd = MetadataCommand::new();

        if let Some(path) = manifest_path {
            cmd.manifest_path(path);
        }

        let metadata = cmd
            .exec()
            .map_err(|e| Error::SourceCollector(format!("Failed to load cargo metadata: {e}")))?;

        let home = StdEnv::var("HOME")
            .or_else(|_| StdEnv::var("USERPROFILE"))
            .map_err(|_| Error::SourceCollector("Could not determine home directory".into()))?;

        let registry_path = PathBuf::from(home).join(".cargo/registry/src");

        Ok(Self {
            metadata,
            registry_path,
        })
    }

    /// Collect all dependency sources.
    ///
    /// # Errors
    ///
    /// Returns an error if collection fails.
    pub fn collect(&self, options: &CollectOptions) -> Result<CollectionResult, Error> {
        // Determine output directory
        let output_dir = match &options.output {
            Some(path) => path.clone(),
            None => self.generate_output_dir()?,
        };

        if options.dry_run {
            return self.dry_run_collect(&output_dir, options);
        }

        // Create output directory
        StdFs::create_dir_all(&output_dir)
            .map_err(|e| Error::SourceCollector(format!("Failed to create output dir: {e}")))?;

        let mut manifest = SourceManifest {
            collected_at: TimeUtils::chrono_lite_now(),
            workspace_root: self.metadata.workspace_root.to_string(),
            crates: HashMap::new(),
        };

        let mut skipped = Vec::new();
        let mut collected_count = 0;

        // Get dev-only packages if we need to filter them out
        let dev_only = if options.include_dev {
            HashSet::new()
        } else {
            self.get_dev_only_packages()
        };

        // Collect each external dependency
        for pkg in &self.metadata.packages {
            // Skip workspace members
            if self.metadata.workspace_members.contains(&pkg.id) {
                continue;
            }

            // Skip dev-only dependencies if not requested
            if dev_only.contains(&pkg.id) {
                continue;
            }

            let version = pkg.version.to_string();
            let key = format!("{}-{}", pkg.name, version);

            // Find source in registry
            match self.find_registry_source(&pkg.name, &version) {
                Some(source_path) => {
                    let dest_dir = output_dir.join(&key);

                    // Copy source files
                    Self::copy_crate_source(&source_path, &dest_dir, options.minimal_sources)?;

                    // Add to manifest
                    manifest.crates.insert(
                        key.clone(),
                        CollectedCrate {
                            name: pkg.name.to_string(),
                            version: version.clone(),
                            edition: pkg.edition.to_string(),
                            features: pkg.features.keys().cloned().collect(),
                            description: pkg.description.clone(),
                            source_path: key,
                        },
                    );

                    collected_count += 1;
                },
                None => {
                    skipped.push(format!("{}-{}", pkg.name, version));
                },
            }
        }

        // Write manifest.json
        let manifest_path = output_dir.join("manifest.json");
        let manifest_json = SJSON::to_string_pretty(&manifest)
            .map_err(|e| Error::SourceCollector(format!("Failed to serialize manifest: {e}")))?;
        StdFs::write(&manifest_path, manifest_json)
            .map_err(|e| Error::SourceCollector(format!("Failed to write manifest: {e}")))?;

        // Update .gitignore unless disabled
        if !options.no_gitignore {
            self.update_gitignore()?;
        }

        Ok(CollectionResult {
            output_dir,
            crates_collected: collected_count,
            skipped,
        })
    }

    /// Generate a timestamp-based output directory name.
    fn generate_output_dir(&self) -> Result<PathBuf, Error> {
        let workspace_root = self.metadata.workspace_root.as_std_path();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| Error::SourceCollector(format!("Failed to get timestamp: {e}")))?
            .as_secs();

        // Try up to 3 times with incrementing timestamp
        for i in 0..3 {
            let dir_name = format!(".source_{}", timestamp + i);
            let path = workspace_root.join(&dir_name);

            if !path.exists() {
                return Ok(path);
            }
        }

        Err(Error::SourceCollector(
            "Too many .source_* directories exist. Please clean up old ones.".into(),
        ))
    }

    /// Find a crate's source in the cargo registry.
    fn find_registry_source(&self, name: &str, version: &str) -> Option<PathBuf> {
        if !self.registry_path.exists() {
            return None;
        }

        let target_dir = format!("{name}-{version}");

        // Scan registry index directories
        for entry in StdFs::read_dir(&self.registry_path).ok()? {
            let entry = entry.ok()?;
            let index_path = entry.path();

            if index_path.is_dir() {
                let crate_path = index_path.join(&target_dir);

                if crate_path.exists() && crate_path.is_dir() {
                    return Some(crate_path);
                }
            }
        }

        None
    }

    /// Copy crate source to destination.
    ///
    /// If `minimal` is false (default), copies the entire crate directory.
    /// If `minimal` is true, only copies `src/` and `Cargo.toml`.
    ///
    /// In both modes, `Cargo.toml` is renamed to `Crate.toml` to avoid
    /// confusing cargo when the collected sources are in the workspace.
    fn copy_crate_source(source: &Path, dest: &Path, minimal: bool) -> Result<(), Error> {
        StdFs::create_dir_all(dest)
            .map_err(|e| Error::SourceCollector(format!("Failed to create dir: {e}")))?;

        if minimal {
            // Minimal mode: only copy src/ and Cargo.toml
            let src_dir = source.join("src");

            if src_dir.exists() {
                Self::copy_dir_recursive(&src_dir, &dest.join("src"))?;
            }

            // Copy and rename Cargo.toml to Crate.toml
            let cargo_toml = source.join("Cargo.toml");
            if cargo_toml.exists() {
                StdFs::copy(&cargo_toml, dest.join("Crate.toml"))
                    .map_err(|e| Error::SourceCollector(format!("Failed to copy Cargo.toml: {e}")))?;
            }
        } else {
            // Full mode: copy entire directory, but rename Cargo.toml
            for entry in StdFs::read_dir(source).map_err(|e| {
                Error::SourceCollector(format!("Failed to read source dir: {e}"))
            })? {
                let entry = entry.map_err(|e| {
                    Error::SourceCollector(format!("Failed to read entry: {e}"))
                })?;
                let path = entry.path();
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();

                // Rename Cargo.toml to Crate.toml to avoid confusing cargo
                let dest_name = if file_name_str == "Cargo.toml" {
                    "Crate.toml".into()
                } else {
                    file_name
                };
                let dest_path = dest.join(dest_name);

                if path.is_dir() {
                    Self::copy_dir_recursive(&path, &dest_path)?;
                } else {
                    StdFs::copy(&path, &dest_path).map_err(|e| {
                        Error::SourceCollector(format!(
                            "Failed to copy {} to {}: {e}",
                            path.display(),
                            dest_path.display()
                        ))
                    })?;
                }
            }
        }

        Ok(())
    }

    /// Get the set of package IDs that are dev-only dependencies.
    ///
    /// A package is considered dev-only if it is only reachable from workspace
    /// members via dev-dependencies (not normal or build dependencies).
    fn get_dev_only_packages(&self) -> HashSet<PackageId> {
        let Some(resolve) = &self.metadata.resolve else {
            return HashSet::new();
        };

        // Build a map of package ID to its node for quick lookup
        let nodes: HashMap<&PackageId, _> =
            resolve.nodes.iter().map(|node| (&node.id, node)).collect();

        // Collect all packages reachable via non-dev dependencies from workspace members
        let mut non_dev_reachable: HashSet<PackageId> = HashSet::new();
        let mut to_visit: Vec<&PackageId> = self.metadata.workspace_members.iter().collect();

        while let Some(pkg_id) = to_visit.pop() {
            if let Some(node) = nodes.get(pkg_id) {
                for dep in &node.deps {
                    // Check if this dependency has any non-dev dependency kinds
                    let has_non_dev = dep
                        .dep_kinds
                        .iter()
                        .any(|dk| !matches!(dk.kind, DependencyKind::Development));

                    if has_non_dev && non_dev_reachable.insert(dep.pkg.clone()) {
                        to_visit.push(&dep.pkg);
                    }
                }
            }
        }

        // Dev-only packages are those in metadata.packages but NOT in non_dev_reachable
        // (excluding workspace members themselves)
        self.metadata
            .packages
            .iter()
            .filter(|pkg| {
                !self.metadata.workspace_members.contains(&pkg.id)
                    && !non_dev_reachable.contains(&pkg.id)
            })
            .map(|pkg| pkg.id.clone())
            .collect()
    }

    /// Perform a dry run, returning what would be collected.
    #[expect(clippy::unnecessary_wraps, reason = "Not really")]
    fn dry_run_collect(
        &self,
        output_dir: &Path,
        options: &CollectOptions,
    ) -> Result<CollectionResult, Error> {
        let mut skipped = Vec::new();
        let mut collected_count = 0;

        // Get dev-only packages if we need to filter them out
        let dev_only = if options.include_dev {
            HashSet::new()
        } else {
            self.get_dev_only_packages()
        };

        for pkg in &self.metadata.packages {
            if self.metadata.workspace_members.contains(&pkg.id) {
                continue;
            }

            // Skip dev-only dependencies if not requested
            if dev_only.contains(&pkg.id) {
                continue;
            }

            let version = pkg.version.to_string();

            if self.find_registry_source(&pkg.name, &version).is_some() {
                collected_count += 1;
            } else {
                skipped.push(format!("{}-{}", pkg.name, version));
            }
        }

        Ok(CollectionResult {
            output_dir: output_dir.to_path_buf(),
            crates_collected: collected_count,
            skipped,
        })
    }

    /// Update .gitignore to include .source_* pattern.
    fn update_gitignore(&self) -> Result<(), Error> {
        let gitignore_path = self.metadata.workspace_root.join(".gitignore");
        let pattern = ".source_*";

        // Read existing content
        let content = StdFs::read_to_string(&gitignore_path).unwrap_or_default();

        // Check if pattern already exists
        if content.lines().any(|line| line.trim() == pattern) {
            return Ok(());
        }

        // Append pattern
        let mut file = StdFs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&gitignore_path)
            .map_err(|e| Error::SourceCollector(format!("Failed to open .gitignore: {e}")))?;

        // Add newline if file doesn't end with one
        if !content.is_empty() && !content.ends_with('\n') {
            writeln!(file).map_err(|e| {
                Error::SourceCollector(format!("Failed to write to .gitignore: {e}"))
            })?;
        }

        writeln!(file, "{pattern}")
            .map_err(|e| Error::SourceCollector(format!("Failed to write to .gitignore: {e}")))?;

        Ok(())
    }

    /// List all external dependencies.
    #[must_use]
    pub fn list_dependencies(&self) -> Vec<(&str, &str)> {
        self.metadata
            .packages
            .iter()
            .filter(|pkg| !self.metadata.workspace_members.contains(&pkg.id))
            .map(|pkg| (pkg.name.as_str(), pkg.version.to_string().leak() as &str))
            .collect()
    }

    /// Recursively copy a directory.
    fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<(), Error> {
        StdFs::create_dir_all(dest).map_err(|e| {
            Error::SourceCollector(format!("Failed to create dir {}: {e}", dest.display()))
        })?;

        for entry in StdFs::read_dir(src).map_err(|e| {
            Error::SourceCollector(format!("Failed to read dir {}: {e}", src.display()))
        })? {
            let entry =
                entry.map_err(|e| Error::SourceCollector(format!("Failed to read entry: {e}")))?;
            let path = entry.path();
            let dest_path = dest.join(entry.file_name());

            if path.is_dir() {
                Self::copy_dir_recursive(&path, &dest_path)?;
            } else {
                StdFs::copy(&path, &dest_path).map_err(|e| {
                    Error::SourceCollector(format!(
                        "Failed to copy {} to {}: {e}",
                        path.display(),
                        dest_path.display()
                    ))
                })?;
            }
        }

        Ok(())
    }
}

struct TimeUtils;

impl TimeUtils {
    /// Simple ISO 8601 timestamp without external dependency.
    fn chrono_lite_now() -> String {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let secs = duration.as_secs();

        // Convert to approximate ISO 8601 (good enough for our purposes)
        // This is a simplified version - not handling leap seconds etc.
        let days_since_epoch = secs / 86400;
        let time_of_day = secs % 86400;

        let hours = time_of_day / 3600;
        let minutes = (time_of_day % 3600) / 60;
        let seconds = time_of_day % 60;

        // Approximate date calculation (doesn't account for leap years perfectly)
        let mut year = 1970;
        let mut remaining_days = days_since_epoch;

        loop {
            let days_in_year = if Self::is_leap_year(year) { 366 } else { 365 };

            if remaining_days < days_in_year {
                break;
            }

            remaining_days -= days_in_year;
            year += 1;
        }

        let mut month = 1;
        let days_in_months = if Self::is_leap_year(year) {
            [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        } else {
            [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        };

        for days in days_in_months {
            if remaining_days < days {
                break;
            }
            remaining_days -= days;
            month += 1;
        }

        let day = remaining_days + 1;

        format!("{year:04}-{month:02}-{day:02}T{hours:02}:{minutes:02}:{seconds:02}Z")
    }

    const fn is_leap_year(year: u64) -> bool {
        (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
    }
}

#[cfg(test)]
mod tests {
    use super::{SourceCollector, TimeUtils};

    #[test]
    fn test_chrono_lite_now() {
        let ts = TimeUtils::chrono_lite_now();
        // Should be in ISO 8601 format
        assert!(ts.contains('T'));
        assert!(ts.ends_with('Z'));
        assert!(ts.starts_with("20")); // 2000s
    }

    #[test]
    fn test_is_leap_year() {
        assert!(TimeUtils::is_leap_year(2000));
        assert!(TimeUtils::is_leap_year(2024));
        assert!(!TimeUtils::is_leap_year(1900));
        assert!(!TimeUtils::is_leap_year(2023));
    }

    #[test]
    fn test_get_dev_only_packages_detects_dev_deps() {
        // This test runs on the actual cargo-docs-md project
        let collector = SourceCollector::new().expect("Failed to create collector");
        let dev_only = collector.get_dev_only_packages();

        // Convert to package names for easier assertion
        let dev_only_names: Vec<&str> = collector
            .metadata
            .packages
            .iter()
            .filter(|pkg| dev_only.contains(&pkg.id))
            .map(|pkg| pkg.name.as_str())
            .collect();

        // insta and divan are dev-only dependencies
        assert!(
            dev_only_names.contains(&"insta"),
            "insta should be detected as dev-only, got: {dev_only_names:?}"
        );
        assert!(
            dev_only_names.contains(&"divan"),
            "divan should be detected as dev-only, got: {dev_only_names:?}"
        );
    }

    #[test]
    fn test_get_dev_only_packages_excludes_normal_deps() {
        let collector = SourceCollector::new().expect("Failed to create collector");
        let dev_only = collector.get_dev_only_packages();

        // Convert to package names for easier assertion
        let dev_only_names: Vec<&str> = collector
            .metadata
            .packages
            .iter()
            .filter(|pkg| dev_only.contains(&pkg.id))
            .map(|pkg| pkg.name.as_str())
            .collect();

        // Normal dependencies should NOT be in dev-only
        assert!(
            !dev_only_names.contains(&"serde"),
            "serde should NOT be dev-only"
        );
        assert!(
            !dev_only_names.contains(&"clap"),
            "clap should NOT be dev-only"
        );
        assert!(
            !dev_only_names.contains(&"syn"),
            "syn should NOT be dev-only"
        );
        // tracing is in both deps and dev-deps, but since it's a normal dep it shouldn't be dev-only
        assert!(
            !dev_only_names.contains(&"tracing"),
            "tracing should NOT be dev-only (it's also a normal dependency)"
        );
    }

    #[test]
    fn test_get_dev_only_packages_with_no_resolve() {
        // When there's no resolve graph, should return empty set
        let mut collector = SourceCollector::new().expect("Failed to create collector");

        // Clear the resolve to simulate metadata without resolve
        collector.metadata.resolve = None;

        let dev_only = collector.get_dev_only_packages();
        assert!(
            dev_only.is_empty(),
            "Should return empty set when no resolve graph"
        );
    }

    #[test]
    fn test_list_dependencies_excludes_workspace_members() {
        let collector = SourceCollector::new().expect("Failed to create collector");
        let deps = collector.list_dependencies();

        // Should not include the workspace member (cargo-docs-md itself)
        let dep_names: Vec<&str> = deps.iter().map(|(name, _)| *name).collect();
        assert!(
            !dep_names.contains(&"cargo-docs-md"),
            "Should not include workspace member"
        );

        // Should include actual dependencies
        assert!(
            dep_names.contains(&"serde"),
            "Should include serde dependency"
        );
    }

    #[test]
    fn test_collect_options_defaults() {
        let options = super::CollectOptions::default();

        assert!(!options.include_dev, "include_dev should default to false");
        assert!(options.output.is_none(), "output should default to None");
        assert!(!options.dry_run, "dry_run should default to false");
        assert!(
            !options.minimal_sources,
            "minimal_sources should default to false (full copy)"
        );
        assert!(
            !options.no_gitignore,
            "no_gitignore should default to false (update gitignore)"
        );
    }

    #[test]
    fn test_copy_crate_source_minimal_mode() {
        use std::fs;
        use tempfile::TempDir;

        // Create a mock source directory structure
        let source_dir = TempDir::new().expect("Failed to create temp dir");
        let source_path = source_dir.path();

        // Create src/ directory with a file
        fs::create_dir_all(source_path.join("src")).expect("Failed to create src dir");
        fs::write(source_path.join("src/lib.rs"), "// lib content").expect("Failed to write lib.rs");

        // Create Cargo.toml
        fs::write(source_path.join("Cargo.toml"), "[package]\nname = \"test\"")
            .expect("Failed to write Cargo.toml");

        // Create additional files that should NOT be copied in minimal mode
        fs::write(source_path.join("build.rs"), "fn main() {}").expect("Failed to write build.rs");
        fs::create_dir_all(source_path.join("benches")).expect("Failed to create benches dir");
        fs::write(source_path.join("benches/bench.rs"), "// bench")
            .expect("Failed to write bench.rs");

        // Create destination directory
        let dest_dir = TempDir::new().expect("Failed to create dest temp dir");
        let dest_path = dest_dir.path().join("test-crate");

        // Copy in minimal mode
        SourceCollector::copy_crate_source(source_path, &dest_path, true)
            .expect("Failed to copy crate source");

        // Verify minimal mode results
        assert!(dest_path.join("src/lib.rs").exists(), "src/lib.rs should be copied");
        assert!(
            dest_path.join("Crate.toml").exists(),
            "Cargo.toml should be copied as Crate.toml"
        );
        assert!(
            !dest_path.join("Cargo.toml").exists(),
            "Cargo.toml should be renamed, not copied"
        );
        assert!(
            !dest_path.join("build.rs").exists(),
            "build.rs should NOT be copied in minimal mode"
        );
        assert!(
            !dest_path.join("benches").exists(),
            "benches/ should NOT be copied in minimal mode"
        );
    }

    #[test]
    fn test_copy_crate_source_full_mode() {
        use std::fs;
        use tempfile::TempDir;

        // Create a mock source directory structure
        let source_dir = TempDir::new().expect("Failed to create temp dir");
        let source_path = source_dir.path();

        // Create src/ directory with a file
        fs::create_dir_all(source_path.join("src")).expect("Failed to create src dir");
        fs::write(source_path.join("src/lib.rs"), "// lib content").expect("Failed to write lib.rs");

        // Create Cargo.toml
        fs::write(source_path.join("Cargo.toml"), "[package]\nname = \"test\"")
            .expect("Failed to write Cargo.toml");

        // Create additional files that SHOULD be copied in full mode
        fs::write(source_path.join("build.rs"), "fn main() {}").expect("Failed to write build.rs");
        fs::create_dir_all(source_path.join("benches")).expect("Failed to create benches dir");
        fs::write(source_path.join("benches/bench.rs"), "// bench")
            .expect("Failed to write bench.rs");
        fs::write(source_path.join("README.md"), "# Test").expect("Failed to write README.md");

        // Create destination directory
        let dest_dir = TempDir::new().expect("Failed to create dest temp dir");
        let dest_path = dest_dir.path().join("test-crate");

        // Copy in full mode (minimal = false)
        SourceCollector::copy_crate_source(source_path, &dest_path, false)
            .expect("Failed to copy crate source");

        // Verify full mode results
        assert!(dest_path.join("src/lib.rs").exists(), "src/lib.rs should be copied");
        assert!(
            dest_path.join("Crate.toml").exists(),
            "Cargo.toml should be copied as Crate.toml"
        );
        assert!(
            !dest_path.join("Cargo.toml").exists(),
            "Cargo.toml should be renamed, not duplicated"
        );
        assert!(
            dest_path.join("build.rs").exists(),
            "build.rs SHOULD be copied in full mode"
        );
        assert!(
            dest_path.join("benches/bench.rs").exists(),
            "benches/ SHOULD be copied in full mode"
        );
        assert!(
            dest_path.join("README.md").exists(),
            "README.md SHOULD be copied in full mode"
        );
    }
}
