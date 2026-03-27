use crate::interp::InterpreterError;
use crate::value::Value;

pub fn is_builtin(name: &str) -> bool {
    matches!(name, "print" | "println" | "assert" | "len")
}

pub fn call(name: &str, args: &[Value]) -> Result<Value, InterpreterError> {
    match name {
        "print" => {
            for arg in args {
                print!("{arg}");
            }
            Ok(Value::Nil)
        }
        "println" => {
            for arg in args {
                print!("{arg}");
            }
            println!();
            Ok(Value::Nil)
        }
        "assert" => {
            if args.len() != 1 {
                return Err(InterpreterError::runtime(
                    "assert expects exactly one argument",
                ));
            }
            if !args[0].is_truthy() {
                return Err(InterpreterError::runtime("assertion failed"));
            }
            Ok(Value::Nil)
        }
        "len" => {
            if args.len() != 1 {
                return Err(InterpreterError::runtime(
                    "len expects exactly one argument",
                ));
            }
            match &args[0] {
                Value::Text(s) => Ok(Value::Number(s.len() as f64)),
                Value::Array(items) => Ok(Value::Number(items.len() as f64)),
                _ => Err(InterpreterError::runtime(
                    "len expects a string or array argument",
                )),
            }
        }
        _ => Err(InterpreterError::runtime(format!(
            "unknown builtin function `{name}`"
        ))),
    }
}
