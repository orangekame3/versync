pub mod json;
pub mod toml;

use crate::config::FileFormat;
use crate::error::Result;
use std::path::Path;

/// Read the version value from a file at the specified key path
pub fn read_version(path: &Path, key: &str, format: FileFormat) -> Result<String> {
    match format {
        FileFormat::Toml => toml::read_version(path, key),
        FileFormat::Json => json::read_version(path, key),
    }
}

/// Write the version value to a file at the specified key path
pub fn write_version(path: &Path, key: &str, version: &str, format: FileFormat) -> Result<()> {
    match format {
        FileFormat::Toml => toml::write_version(path, key, version),
        FileFormat::Json => json::write_version(path, key, version),
    }
}
