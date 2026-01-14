use crate::config::{Config, Target};
use crate::error::{Error, Result};
use crate::format;

/// Result of applying version to a single target
#[derive(Debug)]
pub enum ApplyResult {
    Updated {
        file: String,
        key: String,
        old_version: String,
        new_version: String,
    },
    NoChange {
        file: String,
    },
}

impl std::fmt::Display for ApplyResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplyResult::Updated {
                file,
                key,
                old_version,
                new_version,
            } => {
                write!(
                    f,
                    "UPDATED {} {}: {} -> {}",
                    file, key, old_version, new_version
                )
            }
            ApplyResult::NoChange { file } => {
                write!(f, "NO CHANGE {}", file)
            }
        }
    }
}

/// Apply version to a single target file
fn apply_target(target: &Target, new_version: &str) -> Result<ApplyResult> {
    let format = target
        .effective_format()
        .ok_or_else(|| Error::UnknownFormat(target.file.clone()))?;

    let current_version = format::read_version(&target.file, &target.key, format)?;
    let file = target.file.display().to_string();

    if current_version == new_version {
        return Ok(ApplyResult::NoChange { file });
    }

    format::write_version(&target.file, &target.key, new_version, format)?;

    Ok(ApplyResult::Updated {
        file,
        key: target.key.clone(),
        old_version: current_version,
        new_version: new_version.to_string(),
    })
}

/// Apply version to all targets in the configuration
pub fn apply(config: &Config, quiet: bool) -> Result<()> {
    let mut results = Vec::new();

    for target in &config.targets {
        let result = apply_target(target, &config.version)?;
        results.push(result);
    }

    if !quiet {
        for result in &results {
            println!("{}", result);
        }
    }

    Ok(())
}
