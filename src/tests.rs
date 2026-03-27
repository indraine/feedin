use super::Program;
use std::path::PathBuf;

/// Before running tests, build the `quiz` binary from the workspace root (this crate’s `Cargo.toml` directory):
///
/// ```text
/// cargo build -p quiz
/// ```
///
/// The test runs `target/debug/quiz` relative to the package root (same as the workspace root here).
#[cfg(target_os = "windows")]
const QUIZ_DEBUG_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/target/debug/quiz.exe");

#[cfg(not(target_os = "windows"))]
const QUIZ_DEBUG_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/target/debug/quiz");

#[test]
fn run_quiz() {
    let quiz = Program::new(PathBuf::from(QUIZ_DEBUG_PATH)).with_append_newline(true);
    let result = quiz
        .run(vec![
            "Alice".into(),
            "Rust".into(),
            "5".into(),
            "deployments".into(),
            "8".into(),
        ])
        .expect("run");

    assert_eq!(result.exit_code, Some(0));
}
