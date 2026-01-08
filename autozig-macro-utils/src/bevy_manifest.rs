//! Cargo manifest parsing utilities for finding AutoZig Bevy crate paths.
//!
//! This module helps procedural macros locate the correct import paths
//! for AutoZig Bevy crates in the dependency tree.

use std::env;
use std::path::{Path, PathBuf};
use toml_edit::{DocumentMut, Item};

/// Constant for the AutoZig Bevy root crate name.
pub const AUTOZIG_BEVY: &str = "autozig_bevy";

/// Represents the Cargo manifest and provides utilities to find crate paths.
pub struct BevyManifest {
    manifest: DocumentMut,
}

impl BevyManifest {
    /// Creates a new BevyManifest by reading the Cargo.toml in the current directory.
    ///
    /// # Errors
    ///
    /// Returns an error if the manifest cannot be read or parsed.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
        let manifest_path = PathBuf::from(manifest_dir).join("Cargo.toml");
        Self::from_path(&manifest_path)
    }

    /// Creates a new BevyManifest from a specific path.
    ///
    /// # Errors
    ///
    /// Returns an error if the manifest cannot be read or parsed.
    pub fn from_path(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        let manifest = contents.parse::<DocumentMut>()?;
        Ok(Self { manifest })
    }

    /// Gets the import path for a specific AutoZig Bevy crate.
    ///
    /// Returns the path segment that should be used in imports,
    /// e.g., "autozig_bevy::ecs" or "bevy_ecs" depending on how
    /// the dependency is configured.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the crate (e.g., "ecs", "app", "transform")
    pub fn get_path(&self, name: &str) -> String {
        let crate_name = format!("autozig-{}", name);
        
        // Check if this specific crate is a direct dependency
        if self.has_dependency(&crate_name) {
            return self.get_dependency_path(&crate_name);
        }

        // Check for the root autozig_bevy crate
        if self.has_dependency(AUTOZIG_BEVY) {
            return format!("{}::{}", AUTOZIG_BEVY, name.replace('-', "_"));
        }

        // Default to assuming it's available via autozig_bevy
        format!("{}::{}", AUTOZIG_BEVY, name.replace('-', "_"))
    }

    /// Gets the full path for a specific type within an AutoZig Bevy crate.
    ///
    /// # Arguments
    ///
    /// * `crate_name` - The name of the crate (e.g., "ecs", "app")
    /// * `type_name` - The name of the type (e.g., "Component", "App")
    pub fn get_type_path(&self, crate_name: &str, type_name: &str) -> String {
        format!("{}::{}", self.get_path(crate_name), type_name)
    }

    /// Checks if a dependency exists in the manifest.
    fn has_dependency(&self, name: &str) -> bool {
        self.get_dependency_item(name).is_some()
    }

    /// Gets the import path for a dependency.
    fn get_dependency_path(&self, name: &str) -> String {
        if let Some(item) = self.get_dependency_item(name) {
            // Check if there's a package rename
            if let Some(table) = item.as_table() {
                if let Some(package) = table.get("package") {
                    if let Some(package_str) = package.as_str() {
                        return package_str.replace('-', "_");
                    }
                }
            }
        }
        name.replace('-', "_")
    }

    /// Gets a dependency item from the manifest.
    fn get_dependency_item(&self, name: &str) -> Option<&Item> {
        // Check [dependencies]
        if let Some(deps) = self.manifest.get("dependencies") {
            if let Some(dep) = deps.get(name) {
                return Some(dep);
            }
        }

        // Check [dev-dependencies]
        if let Some(deps) = self.manifest.get("dev-dependencies") {
            if let Some(dep) = deps.get(name) {
                return Some(dep);
            }
        }

        // Check [build-dependencies]
        if let Some(deps) = self.manifest.get("build-dependencies") {
            if let Some(dep) = deps.get(name) {
                return Some(dep);
            }
        }

        None
    }

    /// Gets the name of the current crate from the manifest.
    pub fn get_crate_name(&self) -> Option<String> {
        self.manifest
            .get("package")
            .and_then(|package| package.get("name"))
            .and_then(|name| name.as_str())
            .map(String::from)
    }
}

impl Default for BevyManifest {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            // Return a minimal manifest if we can't read the actual one
            Self {
                manifest: "".parse().unwrap(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autozig_bevy_constant() {
        assert_eq!(AUTOZIG_BEVY, "autozig_bevy");
    }

    #[test]
    fn test_get_path_formatting() {
        // Create a minimal manifest for testing
        let manifest_content = r#"
[package]
name = "test-crate"
version = "0.1.0"

[dependencies]
autozig_bevy = "0.1"
"#;
        let manifest: DocumentMut = manifest_content.parse().unwrap();
        let bevy_manifest = BevyManifest { manifest };
        
        let path = bevy_manifest.get_path("ecs");
        assert!(path.contains("ecs"));
    }

    #[test]
    fn test_crate_name_extraction() {
        let manifest_content = r#"
[package]
name = "my-test-crate"
version = "0.1.0"
"#;
        let manifest: DocumentMut = manifest_content.parse().unwrap();
        let bevy_manifest = BevyManifest { manifest };
        
        assert_eq!(bevy_manifest.get_crate_name(), Some("my-test-crate".to_string()));
    }
}