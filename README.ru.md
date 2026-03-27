# feedin

> [English version](README.md)

Небольшая библиотека на Rust для запуска интерактивных CLI из утилит: процесс стартует, в stdin по очереди передаются строки. Подойдет для написания менеджеров поверх древних программ, в которых использовались вопрос-ответные системы.

## Пример

Интерактивная тестовая программа `quiz` в этом репозитории читает пять ответов из stdin. 

Пример использования:

```rust
use feedin::Program;

fn main() -> Result<(), feedin::FeedinError> {
    let quiz = Program::new("./target/debug/quiz");
    let result = quiz.run(vec![
        "Алиса".into(),
        "Rust".into(),
        "5".into(),
        "деплой".into(),
        "8".into(),
    ])?;
    assert_eq!(result.exit_code, Some(0));
    Ok(())
}
```

Сборка демо-примера: `cargo build -p quiz` — бинарник: `target/debug/quiz` (от корня workspace).
