[**Русская версия**](README.ru.md)

# feedin

A small Rust library for driving interactive CLI programs from utilities: the process starts, and lines are fed to stdin one after another. Handy for building wrappers around legacy apps that used question-and-answer style prompts.

## Example

The interactive demo program `quiz` in this repository reads five answers from stdin.

Usage:

```rust
use feedin::Program;

fn main() -> Result<(), feedin::FeedinError> {
    let quiz = Program::new("./target/debug/quiz");
    let result = quiz.run(vec![
        "Alice".into(),
        "Rust".into(),
        "5".into(),
        "deploy".into(),
        "8".into(),
    ])?;
    assert_eq!(result.exit_code, Some(0));
    Ok(())
}
```

Build the demo: `cargo build -p quiz` — the binary is `target/debug/quiz` (from the workspace root).
