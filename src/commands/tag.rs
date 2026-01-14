use crate::commands::check::check_silent;
use crate::config::Config;
use crate::error::{Error, Result};
use crate::git;

/// Create a git tag based on the configuration
///
/// Prerequisites:
/// 1. Must be inside a git repository
/// 2. versync check must pass
/// 3. Working tree and index must be clean
/// 4. Tag must not already exist
pub fn tag(config: &Config, quiet: bool) -> Result<()> {
    // 1. Ensure we're in a git repository
    git::ensure_git_repository()?;

    // 2. Ensure all versions match
    if !check_silent(config)? {
        return Err(Error::VersionMismatch);
    }

    // 3. Ensure working tree and index are clean
    git::ensure_clean()?;

    // 4. Ensure tag doesn't exist
    let tag_name = config.tag_name();
    git::ensure_tag_not_exists(&tag_name)?;

    // Create the tag
    let message = format!("Release {}", config.version);
    git::create_annotated_tag(&tag_name, &message)?;

    if !quiet {
        println!("CREATED TAG {}", tag_name);
    }

    Ok(())
}
