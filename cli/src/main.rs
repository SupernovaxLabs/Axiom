use std::env;
use std::fs;
use std::io::{self, Write};

use axiom_interpreter::parser::parse_program;
use axiom_interpreter::Interpreter;

fn main() {
    if let Err(err) = run() {
        eprintln!("axiom: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("run") => run_file(args.next()),
        Some("check") => check_file(args.next()),
        Some("eval") => eval_inline(args.collect::<Vec<_>>().join(" ")),
        Some("repl") | None => repl(),
        Some("help") | Some("--help") | Some("-h") => {
            print_help();
            Ok(())
        }
        Some(other) => Err(format!("unknown command `{other}`. Try `axiom-cli help`.")),
    }
}

fn run_file(path: Option<String>) -> Result<(), String> {
    let path = path.ok_or_else(|| "usage: axiom-cli run <file.ax>".to_string())?;
    let src = fs::read_to_string(&path).map_err(|e| format!("cannot read `{path}`: {e}"))?;
    let mut interpreter = Interpreter::new();
    let value = interpreter.eval_program(&src).map_err(|e| e.to_string())?;
    println!("{value}");
    Ok(())
}

fn check_file(path: Option<String>) -> Result<(), String> {
    let path = path.ok_or_else(|| "usage: axiom-cli check <file.ax>".to_string())?;
    let src = fs::read_to_string(&path).map_err(|e| format!("cannot read `{path}`: {e}"))?;
    parse_program(&src).map_err(|e| e.to_string())?;
    println!("syntax OK: {path}");
    Ok(())
}

fn eval_inline(code: String) -> Result<(), String> {
    if code.is_empty() {
        return Err("usage: axiom-cli eval <code>".to_string());
    }
    let mut interpreter = Interpreter::new();
    let value = interpreter.eval_program(&code).map_err(|e| e.to_string())?;
    println!("{value}");
    Ok(())
}

fn repl() -> Result<(), String> {
    let mut interpreter = Interpreter::new();
    let mut line = String::new();

    println!("Axiom CLI REPL (type `:quit` to exit)");
    loop {
        print!("axiom> ");
        io::stdout().flush().map_err(|e| e.to_string())?;
        line.clear();
        io::stdin()
            .read_line(&mut line)
            .map_err(|e| format!("read error: {e}"))?;

        let input = line.trim();
        if input.is_empty() {
            continue;
        }
        if input == ":quit" {
            break;
        }

        match interpreter.eval_program(input) {
            Ok(v) => println!("{v}"),
            Err(e) => eprintln!("{e}"),
        }
    }
    Ok(())
}

fn print_help() {
    println!("Axiom CLI\n");
    println!("Usage:");
    println!("  axiom-cli repl");
    println!("  axiom-cli run <file.ax>");
    println!("  axiom-cli check <file.ax>");
    println!("  axiom-cli eval <code>");
}
