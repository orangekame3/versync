use crate::error::{Error, Result};
use std::process::Command;

/// Check if we're inside a git repository
pub fn is_inside_work_tree() -> Result<bool> {
    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .map_err(|e| Error::GitCommand(format!("Failed to execute git: {}", e)))?;

    Ok(output.status.success())
}

/// Check if the working tree is clean (no unstaged changes)
pub fn is_working_tree_clean() -> Result<bool> {
    let output = Command::new("git")
        .args(["diff", "--quiet"])
        .output()
        .map_err(|e| Error::GitCommand(format!("Failed to execute git diff: {}", e)))?;

    Ok(output.status.success())
}

/// Check if the index is clean (no staged changes)
pub fn is_index_clean() -> Result<bool> {
    let output = Command::new("git")
        .args(["diff", "--cached", "--quiet"])
        .output()
        .map_err(|e| Error::GitCommand(format!("Failed to execute git diff --cached: {}", e)))?;

    Ok(output.status.success())
}

/// Check if a tag already exists
pub fn tag_exists(tag: &str) -> Result<bool> {
    let output = Command::new("git")
        .args(["tag", "--list", tag])
        .output()
        .map_err(|e| Error::GitCommand(format!("Failed to execute git tag --list: {}", e)))?;

    if !output.status.success() {
        return Err(Error::GitCommand("git tag --list failed".to_string()));
    }

    // If the tag exists, the output will contain the tag name
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(!stdout.trim().is_empty())
}

/// Create an annotated tag
pub fn create_annotated_tag(tag: &str, message: &str) -> Result<()> {
    let output = Command::new("git")
        .args(["tag", "-a", tag, "-m", message])
        .output()
        .map_err(|e| Error::GitCommand(format!("Failed to execute git tag: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::GitCommand(format!(
            "Failed to create tag: {}",
            stderr.trim()
        )));
    }

    Ok(())
}

/// Ensure we're in a git repository
pub fn ensure_git_repository() -> Result<()> {
    if !is_inside_work_tree()? {
        return Err(Error::NotGitRepository);
    }
    Ok(())
}

/// Ensure the working tree and index are clean
pub fn ensure_clean() -> Result<()> {
    if !is_working_tree_clean()? {
        return Err(Error::DirtyWorkingTree);
    }
    if !is_index_clean()? {
        return Err(Error::DirtyIndex);
    }
    Ok(())
}

/// Ensure a tag doesn't already exist
pub fn ensure_tag_not_exists(tag: &str) -> Result<()> {
    if tag_exists(tag)? {
        return Err(Error::TagExists(tag.to_string()));
    }
    Ok(())
}
