use crate::error::{Error, Result};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

/// Supported file formats for version targets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileFormat {
    Toml,
    Json,
}

impl FileFormat {
    /// Infer format from file extension
    pub fn from_path(path: &Path) -> Option<Self> {
        match path.extension().and_then(|e| e.to_str()) {
            Some("toml") => Some(FileFormat::Toml),
            Some("json") => Some(FileFormat::Json),
            _ => None,
        }
    }
}

/// A target file containing a version field
#[derive(Debug, Clone, Deserialize)]
pub struct Target {
    /// Path to the file (relative to repository root)
    pub file: PathBuf,
    /// Dot-separated key path (e.g., "project.version")
    pub key: String,
    /// File format (inferred from extension if not specified)
    pub format: Option<FileFormat>,
}

impl Target {
    /// Get the effective format (explicit or inferred from extension)
    pub fn effective_format(&self) -> Option<FileFormat> {
        self.format.or_else(|| FileFormat::from_path(&self.file))
    }
}

/// Git-related configuration
#[derive(Debug, Clone, Deserialize)]
pub struct GitConfig {
    /// Prefix for git tags (default: "v")
    #[serde(default = "default_tag_prefix")]
    pub tag_prefix: String,
}

fn default_tag_prefix() -> String {
    "v".to_string()
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            tag_prefix: default_tag_prefix(),
        }
    }
}

/// Main configuration structure (version.toml)
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// The authoritative version string
    pub version: String,
    /// List of target files to sync
    pub targets: Vec<Target>,
    /// Git configuration
    #[serde(default)]
    pub git: GitConfig,
}

impl Config {
    /// Load configuration from a file
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Err(Error::ConfigNotFound(path.to_path_buf()));
        }

        let content = fs::read_to_string(path)?;
        Self::parse(&content)
    }

    /// Parse configuration from a TOML string
    pub fn parse(content: &str) -> Result<Self> {
        let config: Config =
            toml_edit::de::from_str(content).map_err(|e| Error::ConfigParse(e.to_string()))?;

        // Validate: at least one target is required
        if config.targets.is_empty() {
            return Err(Error::ConfigParse(
                "At least one [[targets]] entry is required".to_string(),
            ));
        }

        Ok(config)
    }

    /// Get the full tag name (prefix + version)
    pub fn tag_name(&self) -> String {
        format!("{}{}", self.git.tag_prefix, self.version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minimal_config() {
        let content = r#"
version = "1.0.0"

[[targets]]
file = "pyproject.toml"
key = "project.version"
"#;
        let config = Config::parse(content).unwrap();
        assert_eq!(config.version, "1.0.0");
        assert_eq!(config.targets.len(), 1);
        assert_eq!(config.git.tag_prefix, "v");
    }

    #[test]
    fn test_parse_full_config() {
        let content = r#"
version = "0.7.3"

[[targets]]
file = "pyproject.toml"
key = "project.version"

[[targets]]
file = "Cargo.toml"
key = "workspace.package.version"

[[targets]]
file = "package.json"
key = "version"
format = "json"

[git]
tag_prefix = "v"
"#;
        let config = Config::parse(content).unwrap();
        assert_eq!(config.version, "0.7.3");
        assert_eq!(config.targets.len(), 3);
        assert_eq!(config.tag_name(), "v0.7.3");
    }

    #[test]
    fn test_parse_empty_targets() {
        let content = r#"
version = "1.0.0"
"#;
        let result = Config::parse(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_format_inference() {
        assert_eq!(
            FileFormat::from_path(Path::new("Cargo.toml")),
            Some(FileFormat::Toml)
        );
        assert_eq!(
            FileFormat::from_path(Path::new("package.json")),
            Some(FileFormat::Json)
        );
        assert_eq!(FileFormat::from_path(Path::new("README.md")), None);
    }
}
