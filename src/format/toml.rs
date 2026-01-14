use crate::error::{Error, Result};
use std::fs;
use std::path::Path;
use toml_edit::{DocumentMut, Item};

/// Read the version value from a TOML file at the specified key path
pub fn read_version(path: &Path, key: &str) -> Result<String> {
    let content =
        fs::read_to_string(path).map_err(|_| Error::TargetNotFound(path.to_path_buf()))?;

    let doc: DocumentMut =
        content
            .parse()
            .map_err(|e: toml_edit::TomlError| Error::TargetParse {
                file: path.to_path_buf(),
                message: e.to_string(),
            })?;

    get_value(&doc, path, key)
}

/// Write the version value to a TOML file at the specified key path
/// Preserves comments and formatting
pub fn write_version(path: &Path, key: &str, version: &str) -> Result<()> {
    let content =
        fs::read_to_string(path).map_err(|_| Error::TargetNotFound(path.to_path_buf()))?;

    let mut doc: DocumentMut =
        content
            .parse()
            .map_err(|e: toml_edit::TomlError| Error::TargetParse {
                file: path.to_path_buf(),
                message: e.to_string(),
            })?;

    set_value(&mut doc, path, key, version)?;

    fs::write(path, doc.to_string())?;
    Ok(())
}

/// Get a string value from a TOML document at the specified key path
fn get_value(doc: &DocumentMut, path: &Path, key: &str) -> Result<String> {
    let keys: Vec<&str> = key.split('.').collect();
    let mut current: &Item = doc.as_item();

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

/// Set a string value in a TOML document at the specified key path
fn set_value(doc: &mut DocumentMut, path: &Path, key: &str, version: &str) -> Result<()> {
    let keys: Vec<&str> = key.split('.').collect();
    let mut current: &mut Item = doc.as_item_mut();

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
    if !target.is_str() {
        return Err(Error::ValueNotString {
            file: path.to_path_buf(),
            key: key.to_string(),
        });
    }

    *target = toml_edit::value(version);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_simple_key() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, r#"version = "1.0.0""#).unwrap();

        let result = read_version(file.path(), "version").unwrap();
        assert_eq!(result, "1.0.0");
    }

    #[test]
    fn test_read_nested_key() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
[project]
name = "test"
version = "2.0.0"
"#
        )
        .unwrap();

        let result = read_version(file.path(), "project.version").unwrap();
        assert_eq!(result, "2.0.0");
    }

    #[test]
    fn test_read_deeply_nested_key() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
[workspace.package]
version = "3.0.0"
"#
        )
        .unwrap();

        let result = read_version(file.path(), "workspace.package.version").unwrap();
        assert_eq!(result, "3.0.0");
    }

    #[test]
    fn test_write_preserves_comments() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"# This is a comment
[project]
name = "test"  # project name
version = "1.0.0"  # version number
"#
        )
        .unwrap();

        write_version(file.path(), "project.version", "2.0.0").unwrap();

        let content = fs::read_to_string(file.path()).unwrap();
        assert!(content.contains("# This is a comment"));
        assert!(content.contains("# project name"));
        assert!(content.contains("\"2.0.0\""));
    }

    #[test]
    fn test_key_not_found() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, r#"version = "1.0.0""#).unwrap();

        let result = read_version(file.path(), "nonexistent");
        assert!(matches!(result, Err(Error::KeyNotFound { .. })));
    }
}
