use std::path::PathBuf;
use thiserror::Error;

/// Exit codes for versync commands
pub mod exit_code {
    /// All versions match (check) or operation succeeded (apply/tag)
    pub const SUCCESS: i32 = 0;
    /// Version mismatch detected (check only)
    pub const MISMATCH: i32 = 1;
    /// Execution error (config error, parse failure, etc.)
    pub const ERROR: i32 = 2;
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Config file not found: {0}")]
    ConfigNotFound(PathBuf),

    #[error("Failed to read config file: {0}")]
    ConfigRead(#[from] std::io::Error),

    #[error("Failed to parse config file: {0}")]
    ConfigParse(String),

    #[error("Target file not found: {0}")]
    TargetNotFound(PathBuf),

    #[error("Failed to parse target file '{file}': {message}")]
    TargetParse { file: PathBuf, message: String },

    #[error("Key not found in '{file}': {key}")]
    KeyNotFound { file: PathBuf, key: String },

    #[error("Value at key '{key}' in '{file}' is not a string")]
    ValueNotString { file: PathBuf, key: String },

    #[error("Unknown file format for: {0}")]
    UnknownFormat(PathBuf),

    #[error("Git command failed: {0}")]
    GitCommand(String),

    #[error("Not inside a git repository")]
    NotGitRepository,

    #[error("Working tree is not clean")]
    DirtyWorkingTree,

    #[error("Index has staged changes")]
    DirtyIndex,

    #[error("Tag already exists: {0}")]
    TagExists(String),

    #[error("Version mismatch detected, run 'versync check' for details")]
    VersionMismatch,
}

pub type Result<T> = std::result::Result<T, Error>;
