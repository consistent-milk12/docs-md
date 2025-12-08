//! Source locator for finding crate sources in the Cargo registry.
//!
//! This module provides utilities to locate crate source code in
//! `~/.cargo/registry/src/` based on crate name and version.

use std::path::{Path, PathBuf};

use cargo_metadata::{Metadata, MetadataCommand, Package};

use crate::error::Error;

/// Locates crate sources in the Cargo registry.
///
/// The locator can find sources either by scanning the registry directly
/// or by using `cargo metadata` to find exact paths for dependencies.
#[derive(Debug)]
pub struct SourceLocator {
    /// Path to the cargo registry source directory.
    /// Typically `~/.cargo/registry/src/`.
    registry_path: PathBuf,

    /// Cached cargo metadata (if loaded from a project).
    metadata: Option<Metadata>,
}

impl SourceLocator {
    /// Create a new `SourceLocator` with the default registry path.
    ///
    /// # Errors
    ///
    /// Returns an error if the home directory cannot be determined.
    pub fn new() -> Result<Self, Error> {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .map_err(|_| Error::SourceLocator("Could not determine home directory".into()))?;

        let registry_path = PathBuf::from(home).join(".cargo/registry/src");

        Ok(Self {
            registry_path,
            metadata: None,
        })
    }

    /// Create a `SourceLocator` with a custom registry path.
    #[must_use]
    pub const fn with_registry_path(registry_path: PathBuf) -> Self {
        Self {
            registry_path,
            metadata: None,
        }
    }

    /// Load cargo metadata from a project directory.
    ///
    /// This enables more accurate source location by using the exact
    /// paths from cargo's dependency resolution.
    ///
    /// # Errors
    ///
    /// Returns an error if cargo metadata cannot be loaded.
    pub fn load_metadata(&mut self, manifest_path: &Path) -> Result<(), Error> {
        let metadata = MetadataCommand::new()
            .manifest_path(manifest_path)
            .exec()
            .map_err(|e| Error::SourceLocator(format!("Failed to load cargo metadata: {e}")))?;

        self.metadata = Some(metadata);

        Ok(())
    }

    /// Load cargo metadata from the current directory.
    ///
    /// # Errors
    ///
    /// Returns an error if cargo metadata cannot be loaded.
    pub fn load_metadata_from_current_dir(&mut self) -> Result<(), Error> {
        let metadata = MetadataCommand::new()
            .exec()
            .map_err(|e| Error::SourceLocator(format!("Failed to load cargo metadata: {e}")))?;

        self.metadata = Some(metadata);

        Ok(())
    }

    /// Locate the source directory for a crate by name and version.
    ///
    /// First tries to use cargo metadata if available, then falls back
    /// to scanning the registry directory.
    ///
    /// # Errors
    ///
    /// Returns an error if the source cannot be found.
    pub fn locate(&self, name: &str, version: &str) -> Result<PathBuf, Error> {
        // Try metadata first (most accurate)
        if let Some(ref metadata) = self.metadata
            && let Some(path) = Self::locate_from_metadata(metadata, name, version)
        {
            return Ok(path);
        }

        // Fall back to registry scan
        self.locate_in_registry(name, version)
    }

    /// Locate source using cargo metadata.
    fn locate_from_metadata(metadata: &Metadata, name: &str, version: &str) -> Option<PathBuf> {
        metadata.packages.iter().find_map(|pkg| {
            if pkg.name == name && pkg.version.to_string() == version {
                // The manifest_path points to Cargo.toml, we want the parent directory
                pkg.manifest_path.parent().map(|p| p.to_path_buf().into())
            } else {
                None
            }
        })
    }

    /// Locate source by scanning the registry directory.
    fn locate_in_registry(&self, name: &str, version: &str) -> Result<PathBuf, Error> {
        // Registry structure: ~/.cargo/registry/src/{index-hash}/{name}-{version}/
        // The index hash varies (e.g., "index.crates.io-1949cf8c6b5b557f")

        if !self.registry_path.exists() {
            return Err(Error::SourceLocator(format!(
                "Cargo registry not found at {}",
                self.registry_path.display()
            )));
        }

        let target_dir_name = format!("{name}-{version}");

        // Scan all index directories
        for entry in std::fs::read_dir(&self.registry_path)
            .map_err(|e| Error::SourceLocator(format!("Failed to read registry: {e}")))?
        {
            let entry =
                entry.map_err(|e| Error::SourceLocator(format!("Failed to read entry: {e}")))?;
            let index_path = entry.path();

            if index_path.is_dir() {
                let crate_path = index_path.join(&target_dir_name);

                if crate_path.exists()
                    && crate_path.is_dir()
                    && (crate_path.join("src").exists() || crate_path.join("lib.rs").exists())
                {
                    return Ok(crate_path);
                }
            }
        }

        Err(Error::SourceLocator(format!(
            "Source not found for {name} v{version} in {}",
            self.registry_path.display()
        )))
    }

    /// Get all packages from the loaded metadata.
    ///
    /// Returns `None` if metadata hasn't been loaded.
    #[must_use]
    pub fn packages(&self) -> Option<&[Package]> {
        self.metadata.as_ref().map(|m| m.packages.as_slice())
    }

    /// Get the workspace root from loaded metadata.
    ///
    /// Returns `None` if metadata hasn't been loaded.
    #[must_use]
    pub fn workspace_root(&self) -> Option<&Path> {
        self.metadata
            .as_ref()
            .map(|m| m.workspace_root.as_std_path())
    }

    /// Find all dependency sources for a workspace.
    ///
    /// Returns a list of (name, version, path) tuples for all dependencies
    /// that have sources in the registry.
    ///
    /// # Errors
    ///
    /// Returns an error if metadata hasn't been loaded.
    pub fn all_dependency_sources(&self) -> Result<Vec<(String, String, PathBuf)>, Error> {
        let metadata = self.metadata.as_ref().ok_or_else(|| {
            Error::SourceLocator("Metadata not loaded. Call load_metadata first.".into())
        })?;

        let mut sources = Vec::new();

        for pkg in &metadata.packages {
            // Skip workspace members (they're not in the registry)
            if metadata.workspace_members.contains(&pkg.id) {
                continue;
            }

            let version = pkg.version.to_string();

            // Try to locate the source
            if let Ok(path) = self.locate(&pkg.name, &version) {
                sources.push((pkg.name.to_string(), version, path));
            }
        }

        Ok(sources)
    }

    /// List all available crate versions in the registry.
    ///
    /// Returns a list of (name, version) tuples.
    ///
    /// # Errors
    ///
    /// Returns an error if the registry cannot be read.
    pub fn list_registry_crates(&self) -> Result<Vec<(String, String)>, Error> {
        if !self.registry_path.exists() {
            return Err(Error::SourceLocator(format!(
                "Cargo registry not found at {}",
                self.registry_path.display()
            )));
        }

        let mut crates = Vec::new();

        for index_entry in std::fs::read_dir(&self.registry_path)
            .map_err(|e| Error::SourceLocator(format!("Failed to read registry: {e}")))?
        {
            let index_entry = index_entry
                .map_err(|e| Error::SourceLocator(format!("Failed to read entry: {e}")))?;
            let index_path = index_entry.path();

            if !index_path.is_dir() {
                continue;
            }

            for crate_entry in std::fs::read_dir(&index_path)
                .map_err(|e| Error::SourceLocator(format!("Failed to read index: {e}")))?
            {
                let crate_entry = crate_entry
                    .map_err(|e| Error::SourceLocator(format!("Failed to read entry: {e}")))?;
                let crate_path = crate_entry.path();

                if !crate_path.is_dir() {
                    continue;
                }

                if let Some(dir_name) = crate_path.file_name().and_then(|n| n.to_str())
                    && let Some((name, version)) = ParseUtils::parse_crate_dir_name(dir_name)
                {
                    crates.push((name.to_string(), version.to_string()));
                }
            }
        }

        Ok(crates)
    }
}

struct ParseUtils;

impl ParseUtils {
    /// Parse a crate directory name like "serde-1.0.228" into (name, version).
    pub fn parse_crate_dir_name(dir_name: &str) -> Option<(&str, &str)> {
        // Find the last hyphen followed by a digit (start of version)
        let bytes = dir_name.as_bytes();
        let mut last_hyphen_before_version = None;

        for (i, &byte) in bytes.iter().enumerate() {
            if byte == b'-' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit() {
                last_hyphen_before_version = Some(i);
            }
        }

        last_hyphen_before_version.map(|pos| {
            let name = &dir_name[..pos];
            let version = &dir_name[pos + 1..];

            (name, version)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::source::locator::ParseUtils;

    #[test]
    fn test_parse_crate_dir_name() {
        assert_eq!(
            ParseUtils::parse_crate_dir_name("serde-1.0.228"),
            Some(("serde", "1.0.228"))
        );

        assert_eq!(
            ParseUtils::parse_crate_dir_name("serde_json-1.0.145"),
            Some(("serde_json", "1.0.145"))
        );

        assert_eq!(
            ParseUtils::parse_crate_dir_name("my-crate-name-0.1.0"),
            Some(("my-crate-name", "0.1.0"))
        );

        assert_eq!(
            ParseUtils::parse_crate_dir_name("tokio-1.48.0"),
            Some(("tokio", "1.48.0"))
        );

        // Edge cases
        assert_eq!(ParseUtils::parse_crate_dir_name("nocrate"), None);
        assert_eq!(ParseUtils::parse_crate_dir_name("a-1"), Some(("a", "1")));
    }
}
