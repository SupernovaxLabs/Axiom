mod air;
mod borrowck;
mod codegen;
mod driver;
mod lexer;
mod linker;
mod opt;
mod parser;
mod typeck;
mod utils;

use driver::CompilerDriver;

fn main() {
    let mut driver = CompilerDriver::new();
    if let Err(err) = driver.run() {
        eprintln!("axiomc bootstrap error: {err}");
        std::process::exit(1);
    }
}
