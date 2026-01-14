use crate::error::{Error, Result};
use serde_json::Value;
use std::fs;
use std::path::Path;

/// Read the version value from a JSON file at the specified key path
pub fn read_version(path: &Path, key: &str) -> Result<String> {
    let content =
        fs::read_to_string(path).map_err(|_| Error::TargetNotFound(path.to_path_buf()))?;

    let json: Value = serde_json::from_str(&content).map_err(|e| Error::TargetParse {
        file: path.to_path_buf(),
        message: e.to_string(),
    })?;

    get_value(&json, path, key)
}

/// Write the version value to a JSON file at the specified key path
/// Uses pretty print with 2-space indentation
pub fn write_version(path: &Path, key: &str, version: &str) -> Result<()> {
    let content =
        fs::read_to_string(path).map_err(|_| Error::TargetNotFound(path.to_path_buf()))?;

    let mut json: Value = serde_json::from_str(&content).map_err(|e| Error::TargetParse {
        file: path.to_path_buf(),
        message: e.to_string(),
    })?;

    set_value(&mut json, path, key, version)?;

    // Pretty print with 2-space indentation and trailing newline
    let output = serde_json::to_string_pretty(&json).map_err(|e| Error::TargetParse {
        file: path.to_path_buf(),
        message: e.to_string(),
    })?;

    fs::write(path, format!("{}\n", output))?;
    Ok(())
}

/// Get a string value from a JSON value at the specified key path
fn get_value(json: &Value, path: &Path, key: &str) -> Result<String> {
    let keys: Vec<&str> = key.split('.').collect();
    let mut current = json;

    for k in &keys {
        current = current.get(k).ok_or_else(|| Error::KeyNotFound {
            file: path.to_path_buf(),
            key: key.to_string(),
        })?;
    }

    current
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| Error::ValueNotString {
            file: path.to_path_buf(),
            key: key.to_string(),
        })
}

/// Set a string value in a JSON value at the specified key path
fn set_value(json: &mut Value, path: &Path, key: &str, version: &str) -> Result<()> {
    let keys: Vec<&str> = key.split('.').collect();
    let mut current = json;

    for k in &keys[..keys.len() - 1] {
        current = current.get_mut(k).ok_or_else(|| Error::KeyNotFound {
            file: path.to_path_buf(),
            key: key.to_string(),
        })?;
    }

    let last_key = keys.last().ok_or_else(|| Error::KeyNotFound {
        file: path.to_path_buf(),
        key: key.to_string(),
    })?;

    let target = current
        .get_mut(last_key)
        .ok_or_else(|| Error::KeyNotFound {
            file: path.to_path_buf(),
            key: key.to_string(),
        })?;

    // Ensure it's currently a string before replacing
    if !target.is_string() {
        return Err(Error::ValueNotString {
            file: path.to_path_buf(),
            key: key.to_string(),
        });
    }

    *target = Value::String(version.to_string());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_simple_key() {
        let mut file = NamedTempFile::with_suffix(".json").unwrap();
        writeln!(file, r#"{{"version": "1.0.0"}}"#).unwrap();

        let result = read_version(file.path(), "version").unwrap();
        assert_eq!(result, "1.0.0");
    }

    #[test]
    fn test_read_nested_key() {
        let mut file = NamedTempFile::with_suffix(".json").unwrap();
        writeln!(
            file,
            r#"{{
  "package": {{
    "name": "test",
    "version": "2.0.0"
  }}
}}"#
        )
        .unwrap();

        let result = read_version(file.path(), "package.version").unwrap();
        assert_eq!(result, "2.0.0");
    }

    #[test]
    fn test_write_pretty_prints() {
        let mut file = NamedTempFile::with_suffix(".json").unwrap();
        writeln!(file, r#"{{"version":"1.0.0","name":"test"}}"#).unwrap();

        write_version(file.path(), "version", "2.0.0").unwrap();

        let content = fs::read_to_string(file.path()).unwrap();
        // Should be pretty printed with 2-space indentation
        assert!(content.contains("  \"version\": \"2.0.0\""));
        assert!(content.ends_with('\n'));
    }

    #[test]
    fn test_key_not_found() {
        let mut file = NamedTempFile::with_suffix(".json").unwrap();
        writeln!(file, r#"{{"version": "1.0.0"}}"#).unwrap();

        let result = read_version(file.path(), "nonexistent");
        assert!(matches!(result, Err(Error::KeyNotFound { .. })));
    }
}
