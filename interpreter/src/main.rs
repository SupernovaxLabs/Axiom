use std::io::{self, Write};

use axiom_interpreter::Interpreter;

fn main() {
    let mut interpreter = Interpreter::new();
    let mut line = String::new();

    loop {
        print!("axiom> ");
        io::stdout().flush().expect("flush should succeed");
        line.clear();
        if io::stdin().read_line(&mut line).is_err() {
            eprintln!("failed to read line");
            break;
        }

        if line.trim().eq_ignore_ascii_case("exit") {
            break;
        }

        match interpreter.eval_program(&line) {
            Ok(v) => println!("{v}"),
            Err(e) => eprintln!("{e}"),
        }
    }
}
