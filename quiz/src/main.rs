use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    const QUESTIONS: [&str; 5] = [
        "What is your name?",
        "What is your favorite programming language?",
        "How many years have you been programming?",
        "What do you want to automate first?",
        "Rate your day from 1 to 10:",
    ];

    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut stdout = io::stdout();
    let mut line = String::new();

    for (i, question) in QUESTIONS.iter().enumerate() {
        writeln!(stdout, "{}. {}", i + 1, question)?;
        stdout.flush()?;
        line.clear();
        reader.read_line(&mut line)?;
        let answer = line.trim_end_matches(['\r', '\n']).trim();
        writeln!(
            stdout,
            "  → recorded: {}\n",
            if answer.is_empty() {
                "(empty answer)"
            } else {
                answer
            }
        )?;
        stdout.flush()?;
    }

    writeln!(stdout, "Thanks for your answers!")?;
    Ok(())
}
