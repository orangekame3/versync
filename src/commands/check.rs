use crate::config::{Config, Target};
use crate::error::{Error, Result};
use crate::format;

/// Result of checking a single target
#[derive(Debug)]
pub enum CheckResult {
    Ok {
        file: String,
        key: String,
    },
    Mismatch {
        file: String,
        key: String,
        expected: String,
        actual: String,
    },
}

impl CheckResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, CheckResult::Ok { .. })
    }
}

impl std::fmt::Display for CheckResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckResult::Ok { file, key } => {
                write!(f, "OK {} {}", file, key)
            }
            CheckResult::Mismatch {
                file,
                key,
                expected,
                actual,
            } => {
                write!(f, "MISMATCH {} {}: {} != {}", file, key, actual, expected)
            }
        }
    }
}

/// Check a single target file
fn check_target(target: &Target, expected_version: &str) -> Result<CheckResult> {
    let format = target
        .effective_format()
        .ok_or_else(|| Error::UnknownFormat(target.file.clone()))?;

    let actual_version = format::read_version(&target.file, &target.key, format)?;

    let file = target.file.display().to_string();
    let key = target.key.clone();

    if actual_version == expected_version {
        Ok(CheckResult::Ok { file, key })
    } else {
        Ok(CheckResult::Mismatch {
            file,
            key,
            expected: expected_version.to_string(),
            actual: actual_version,
        })
    }
}

/// Check all targets in the configuration
///
/// Returns a list of check results and whether all checks passed
pub fn check(config: &Config, quiet: bool) -> Result<bool> {
    let mut all_ok = true;
    let mut results = Vec::new();

    for target in &config.targets {
        match check_target(target, &config.version) {
            Ok(result) => {
                if !result.is_ok() {
                    all_ok = false;
                }
                results.push(result);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    if !quiet {
        for result in &results {
            println!("{}", result);
        }
    }

    Ok(all_ok)
}

/// Check all targets without printing (for internal use)
pub fn check_silent(config: &Config) -> Result<bool> {
    for target in &config.targets {
        let format = target
            .effective_format()
            .ok_or_else(|| Error::UnknownFormat(target.file.clone()))?;

        let actual_version = format::read_version(&target.file, &target.key, format)?;

        if actual_version != config.version {
            return Ok(false);
        }
    }

    Ok(true)
}
