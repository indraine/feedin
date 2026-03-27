//! Drive interactive CLIs from tests or tooling: spawn a process, pipe scripted input to stdin,
//! then collect the exit status.
//!
//! # Quick start
//!
//! Create a [`Program`] with the executable path, then call [`Program::run`] with a vector of
//! strings (one string per stdin write, each followed by a newline — typical for `read_line`
//! loops).
//!
//! ```no_run
//! use feedin::Program;
//!
//! fn main() -> Result<(), feedin::FeedinError> {
//!     let quiz = Program::new("./quiz");
//!     let result = quiz.run(vec![
//!         "Alice".into(),
//!         "Rust".into(),
//!         "5".into(),
//!         "deployments".into(),
//!         "8".into(),
//!     ])?;
//!     assert_eq!(result.exit_code, Some(0));
//!     Ok(())
//! }
//! ```
//!
//! # Custom working directory or raw stdin chunks
//!
//! Use [`Program::with_cwd`] so the child runs from another directory. Use
//! [`Program::with_append_newline`] with `false` if you must send raw bytes without `\n` between writes.
//!
//! ```no_run
//! use feedin::Program;
//! use std::path::PathBuf;
//!
//! fn main() -> Result<(), feedin::FeedinError> {
//!     let quiz = Program::new("./quiz").with_cwd(PathBuf::from("/path/to/project"));
//!     let result = quiz.run(vec!["Alice".into(), "Rust".into()])?;
//!     assert_eq!(result.exit_code, Some(0));
//!     Ok(())
//! }
//! ```
//!
//! The working directory for [`Program::run`] is [`std::env::current_dir`] at call time when no
//! explicit cwd was set (falls back to `"."` if that fails).

use std::{
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

use thiserror::Error;

/// Executable to run with piped stdin, optional fixed working directory, and newline policy.
#[derive(Debug, Clone)]
pub struct Program {
    command: PathBuf,
    cwd: Option<PathBuf>,
    append_newline: bool,
}

impl Program {
    /// Executable path (relative to the child’s working directory, or absolute).
    ///
    /// Default: working directory is [`std::env::current_dir`] at each [`Program::run`] (fallback
    /// `"."`); each stdin write is followed by `\n`.
    pub fn new(command: impl Into<PathBuf>) -> Self {
        Self {
            command: command.into(),
            cwd: None,
            append_newline: true,
        }
    }

    /// Run the child in this directory instead of resolving [`std::env::current_dir`] on each
    /// [`Program::run`].
    pub fn with_cwd(mut self, cwd: impl Into<PathBuf>) -> Self {
        self.cwd = Some(cwd.into());
        self
    }

    /// If `false`, do not append `\n` after each element of `inputs` written to stdin.
    pub fn with_append_newline(mut self, yes: bool) -> Self {
        self.append_newline = yes;
        self
    }

    /// Spawns the program, writes each `inputs` element to stdin in order, closes stdin, then
    /// waits for exit. When newline appending is enabled (the default), `\n` is written after each
    /// chunk.
    ///
    /// # Errors
    ///
    /// Returns [`FeedinError::StartProcess`] if spawning fails, [`FeedinError::StdinWrite`] if writing
    /// stdin fails, or [`FeedinError::WaitProcess`] if waiting for the child fails.
    pub fn run(&self, inputs: Vec<String>) -> Result<RunResult, FeedinError> {

        let cwd = self.cwd.clone().unwrap_or_else(|| {
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        });

        let mut process = Command::new(&self.command)
            .stdin(Stdio::piped())
            .current_dir(&cwd)
            .spawn()
            .map_err(|source| FeedinError::StartProcess {
                command: self.command.display().to_string(),
                source,
            })?;

        if let Some(stdin) = process.stdin.as_mut() {
            for input in &inputs {
                stdin
                    .write_all(input.as_bytes())
                    .map_err(FeedinError::StdinWrite)?;
                if self.append_newline {
                    stdin.write_all(b"\n").map_err(FeedinError::StdinWrite)?;
                }
            }
        }

        let status = process.wait().map_err(FeedinError::WaitProcess)?;

        Ok(RunResult {
            exit_code: status.code(),
        })
    }
}

/// Outcome after the child has exited (or been waited on).
#[derive(Debug, Clone)]
pub struct RunResult {
    /// `None` if the process was killed by a signal on Unix.
    pub exit_code: Option<i32>,
}

#[derive(Debug, Error)]
pub enum FeedinError {
    #[error("failed to start process `{command}`: {source}")]
    StartProcess {
        command: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to write process stdin: {0}")]
    StdinWrite(#[source] std::io::Error),
    #[error("failed to wait for process: {0}")]
    WaitProcess(#[source] std::io::Error),
}

#[cfg(test)]
mod tests;
