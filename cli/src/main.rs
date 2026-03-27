use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

use axiom_interpreter::parser::{lex_tokens, parse_program};
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
        Some("ast") => show_ast(args.next()),
        Some("tokens") => show_tokens(args.next()),
        Some("bench") => bench_file(args.next(), args.next()),
        Some("examples") => examples_cmd(args.next()),
        Some("new") => scaffold_cmd(args.next()),
        Some("eval") => eval_inline(args.collect::<Vec<_>>().join(" ")),
        Some("repl") | None => repl(),
        Some("version") | Some("--version") | Some("-V") => {
            println!("axiom-cli 0.1.0");
            Ok(())
        }
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

fn show_ast(path: Option<String>) -> Result<(), String> {
    let path = path.ok_or_else(|| "usage: axiom-cli ast <file.ax>".to_string())?;
    let src = fs::read_to_string(&path).map_err(|e| format!("cannot read `{path}`: {e}"))?;
    let ast = parse_program(&src).map_err(|e| e.to_string())?;
    println!("{ast:#?}");
    Ok(())
}

fn show_tokens(path: Option<String>) -> Result<(), String> {
    let path = path.ok_or_else(|| "usage: axiom-cli tokens <file.ax>".to_string())?;
    let src = fs::read_to_string(&path).map_err(|e| format!("cannot read `{path}`: {e}"))?;
    let tokens = lex_tokens(&src).map_err(|e| e.to_string())?;
    for token in tokens {
        println!("{token}");
    }
    Ok(())
}

fn bench_file(path: Option<String>, iterations: Option<String>) -> Result<(), String> {
    let path = path.ok_or_else(|| "usage: axiom-cli bench <file.ax> [iterations]".to_string())?;
    let iters = iterations
        .as_deref()
        .unwrap_or("100")
        .parse::<usize>()
        .map_err(|_| "iterations must be an integer".to_string())?;
    let src = fs::read_to_string(&path).map_err(|e| format!("cannot read `{path}`: {e}"))?;

    let start = Instant::now();
    for _ in 0..iters {
        let mut interpreter = Interpreter::new();
        interpreter.eval_program(&src).map_err(|e| e.to_string())?;
    }
    let elapsed = start.elapsed();
    let avg = elapsed.as_secs_f64() * 1000.0 / iters as f64;
    println!(
        "bench: file={path} iterations={iters} total_ms={:.3} avg_ms={:.3}",
        elapsed.as_secs_f64() * 1000.0,
        avg
    );
    Ok(())
}

fn examples_cmd(sub: Option<String>) -> Result<(), String> {
    match sub.as_deref() {
        None | Some("list") => {
            let files = list_examples()?;
            if files.is_empty() {
                println!("no examples found in examples/ax");
            } else {
                println!("available examples:");
                for file in files {
                    println!("  {file}");
                }
            }
            Ok(())
        }
        Some("run") => {
            let mut rest = env::args().skip(3);
            let name = rest
                .next()
                .ok_or_else(|| "usage: axiom-cli examples run <name-or-file.ax>".to_string())?;
            run_file(Some(resolve_example(&name)?.to_string_lossy().to_string()))
        }
        Some(other) => Err(format!(
            "unknown examples command `{other}`. Use `examples list` or `examples run <name>`."
        )),
    }
}

fn list_examples() -> Result<Vec<String>, String> {
    let dir = Path::new("examples/ax");
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut out = Vec::new();
    for entry in fs::read_dir(dir).map_err(|e| format!("cannot read examples dir: {e}"))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("ax") {
            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                out.push(name.to_string());
            }
        }
    }
    out.sort();
    Ok(out)
}

fn resolve_example(name: &str) -> Result<PathBuf, String> {
    let direct = Path::new(name);
    if direct.exists() {
        return Ok(direct.to_path_buf());
    }

    let candidate = Path::new("examples/ax").join(name);
    if candidate.exists() {
        return Ok(candidate);
    }

    let with_ext = Path::new("examples/ax").join(format!("{name}.ax"));
    if with_ext.exists() {
        return Ok(with_ext);
    }

    Err(format!("example `{name}` not found"))
}

fn scaffold_cmd(path: Option<String>) -> Result<(), String> {
    let path = path.ok_or_else(|| "usage: axiom-cli new <name-or-path.ax>".to_string())?;
    let mut target = PathBuf::from(path);
    if target.extension().is_none() {
        target.set_extension("ax");
    }
    if target.exists() {
        return Err(format!("file `{}` already exists", target.display()));
    }

    let template = "// New Axiom program\nfn main() {\n    println(\"Hello from Axiom\")\n}\n";
    fs::write(&target, template)
        .map_err(|e| format!("cannot write `{}`: {e}", target.display()))?;
    println!("created {}", target.display());
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
    println!("  axiom-cli ast <file.ax>");
    println!("  axiom-cli tokens <file.ax>");
    println!("  axiom-cli bench <file.ax> [iterations]");
    println!("  axiom-cli examples [list]");
    println!("  axiom-cli examples run <name-or-file.ax>");
    println!("  axiom-cli new <name-or-path.ax>");
    println!("  axiom-cli eval <code>");
}
