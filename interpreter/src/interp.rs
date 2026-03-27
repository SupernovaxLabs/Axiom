use std::collections::HashMap;

use thiserror::Error;

use crate::builtins;
use crate::env::Env;
use crate::parser::{parse_program, AssignOp, BinaryOp, Expr, Stmt, UnaryOp};
use crate::value::Value;

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("parse error: {0}")]
    Parse(String),
    #[error("runtime error: {0}")]
    Runtime(String),
}

impl InterpreterError {
    pub fn parse(msg: impl Into<String>) -> Self {
        Self::Parse(msg.into())
    }

    pub fn runtime(msg: impl Into<String>) -> Self {
        Self::Runtime(msg.into())
    }
}

#[derive(Clone, Debug)]
struct FunctionDef {
    params: Vec<String>,
    body: Vec<Stmt>,
}

#[derive(Debug)]
enum ExecResult {
    Value(Value),
    Return(Value),
    Break,
    Continue,
}

pub struct Interpreter {
    env: Env,
    functions: HashMap<String, FunctionDef>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            env: Env::new(),
            functions: HashMap::new(),
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn eval_program(&mut self, src: &str) -> Result<Value, InterpreterError> {
        let statements = parse_program(src)?;
        let mut last = Value::Nil;
        for stmt in statements {
            match self.eval_stmt(&stmt)? {
                ExecResult::Value(v) => last = v,
                ExecResult::Return(v) => return Ok(v),
                ExecResult::Break => {
                    return Err(InterpreterError::runtime("`break` used outside of a loop"));
                }
                ExecResult::Continue => {
                    return Err(InterpreterError::runtime(
                        "`continue` used outside of a loop",
                    ));
                }
            }
        }
        Ok(last)
    }

    fn eval_stmt(&mut self, stmt: &Stmt) -> Result<ExecResult, InterpreterError> {
        match stmt {
            Stmt::Let {
                name,
                mutable,
                value,
            } => {
                let value = self.eval_expr(value)?;
                self.env.define(name.clone(), value.clone(), *mutable);
                Ok(ExecResult::Value(value))
            }
            Stmt::Assign { name, op, value } => {
                let value = self.eval_expr(value)?;
                let final_value = self.apply_assignment(name, op, value)?;
                Ok(ExecResult::Value(final_value))
            }
            Stmt::Fn { name, params, body } => {
                self.functions.insert(
                    name.clone(),
                    FunctionDef {
                        params: params.clone(),
                        body: body.clone(),
                    },
                );
                Ok(ExecResult::Value(Value::Nil))
            }
            Stmt::Return(expr) => Ok(ExecResult::Return(self.eval_expr(expr)?)),
            Stmt::Block(stmts) => {
                self.env.enter_scope();
                let result = self.eval_block(stmts);
                self.env.exit_scope();
                result
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.eval_expr(condition)?.is_truthy() {
                    self.eval_stmt(then_branch)
                } else if let Some(else_branch) = else_branch {
                    self.eval_stmt(else_branch)
                } else {
                    Ok(ExecResult::Value(Value::Nil))
                }
            }
            Stmt::While { condition, body } => {
                let mut last = Value::Nil;
                while self.eval_expr(condition)?.is_truthy() {
                    match self.eval_stmt(body)? {
                        ExecResult::Value(v) => last = v,
                        ExecResult::Return(v) => return Ok(ExecResult::Return(v)),
                        ExecResult::Break => break,
                        ExecResult::Continue => continue,
                    }
                }
                Ok(ExecResult::Value(last))
            }
            Stmt::For { name, iter, body } => {
                let iterable = self.eval_expr(iter)?;
                let mut last = Value::Nil;
                match iterable {
                    Value::Array(items) => {
                        for item in items {
                            self.env.enter_scope();
                            self.env.define(name.clone(), item, false);
                            let step = self.eval_stmt(body);
                            self.env.exit_scope();
                            match step? {
                                ExecResult::Value(v) => last = v,
                                ExecResult::Return(v) => return Ok(ExecResult::Return(v)),
                                ExecResult::Break => break,
                                ExecResult::Continue => continue,
                            }
                        }
                    }
                    Value::Text(text) => {
                        for ch in text.chars() {
                            self.env.enter_scope();
                            self.env
                                .define(name.clone(), Value::Text(ch.to_string()), false);
                            let step = self.eval_stmt(body);
                            self.env.exit_scope();
                            match step? {
                                ExecResult::Value(v) => last = v,
                                ExecResult::Return(v) => return Ok(ExecResult::Return(v)),
                                ExecResult::Break => break,
                                ExecResult::Continue => continue,
                            }
                        }
                    }
                    Value::Range {
                        start,
                        end,
                        inclusive,
                    } => {
                        let end_bound = if inclusive { end + 1 } else { end };
                        for n in start..end_bound {
                            self.env.enter_scope();
                            self.env
                                .define(name.clone(), Value::Number(n as f64), false);
                            let step = self.eval_stmt(body);
                            self.env.exit_scope();
                            match step? {
                                ExecResult::Value(v) => last = v,
                                ExecResult::Return(v) => return Ok(ExecResult::Return(v)),
                                ExecResult::Break => break,
                                ExecResult::Continue => continue,
                            }
                        }
                    }
                    _ => {
                        return Err(InterpreterError::runtime(
                            "for-loop iterable must be an array, string, or numeric range",
                        ));
                    }
                }
                Ok(ExecResult::Value(last))
            }
            Stmt::Break => Ok(ExecResult::Break),
            Stmt::Continue => Ok(ExecResult::Continue),
            Stmt::Expr(expr) => Ok(ExecResult::Value(self.eval_expr(expr)?)),
        }
    }

    fn apply_assignment(
        &mut self,
        name: &str,
        op: &AssignOp,
        value: Value,
    ) -> Result<Value, InterpreterError> {
        let result = match op {
            AssignOp::Set => value,
            AssignOp::Add => {
                self.eval_materialized_binary(&BinaryOp::Add, self.read_variable(name)?, value)?
            }
            AssignOp::Sub => {
                self.eval_materialized_binary(&BinaryOp::Sub, self.read_variable(name)?, value)?
            }
            AssignOp::Mul => {
                self.eval_materialized_binary(&BinaryOp::Mul, self.read_variable(name)?, value)?
            }
            AssignOp::Div => {
                self.eval_materialized_binary(&BinaryOp::Div, self.read_variable(name)?, value)?
            }
            AssignOp::Mod => {
                self.eval_materialized_binary(&BinaryOp::Mod, self.read_variable(name)?, value)?
            }
        };

        self.env.assign(name, result.clone())?;
        Ok(result)
    }

    fn read_variable(&self, name: &str) -> Result<Value, InterpreterError> {
        self.env
            .get(name)
            .cloned()
            .ok_or_else(|| InterpreterError::runtime(format!("undefined variable `{name}`")))
    }

    fn eval_block(&mut self, stmts: &[Stmt]) -> Result<ExecResult, InterpreterError> {
        let mut last = Value::Nil;
        for stmt in stmts {
            match self.eval_stmt(stmt)? {
                ExecResult::Value(v) => last = v,
                ExecResult::Return(v) => return Ok(ExecResult::Return(v)),
                ExecResult::Break => return Ok(ExecResult::Break),
                ExecResult::Continue => return Ok(ExecResult::Continue),
            }
        }
        Ok(ExecResult::Value(last))
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, InterpreterError> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::Text(s) => Ok(Value::Text(s.clone())),
            Expr::Nil => Ok(Value::Nil),
            Expr::Array(items) => {
                let mut out = Vec::with_capacity(items.len());
                for item in items {
                    out.push(self.eval_expr(item)?);
                }
                Ok(Value::Array(out))
            }
            Expr::Ident(name) => {
                self.env.get(name).cloned().ok_or_else(|| {
                    InterpreterError::runtime(format!("undefined variable `{name}`"))
                })
            }
            Expr::Unary { op, expr } => {
                let value = self.eval_expr(expr)?;
                match op {
                    UnaryOp::Neg => match value {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => Err(InterpreterError::runtime("unary - expects number")),
                    },
                    UnaryOp::Not => Ok(Value::Bool(!value.is_truthy())),
                }
            }
            Expr::Binary { left, op, right } => self.eval_binary(op, left, right),
            Expr::Call { name, args } => self.eval_call(name, args),
            Expr::Index { target, index } => {
                let target = self.eval_expr(target)?;
                let index = self.eval_expr(index)?;
                self.eval_index(target, index)
            }
            Expr::Range {
                start,
                end,
                inclusive,
            } => {
                let start = self.eval_expr(start)?.as_number().ok_or_else(|| {
                    InterpreterError::runtime("range start must evaluate to a number")
                })? as i64;
                let end = self.eval_expr(end)?.as_number().ok_or_else(|| {
                    InterpreterError::runtime("range end must evaluate to a number")
                })? as i64;
                Ok(Value::Range {
                    start,
                    end,
                    inclusive: *inclusive,
                })
            }
        }
    }

    fn eval_index(&self, target: Value, index: Value) -> Result<Value, InterpreterError> {
        let idx = index
            .as_number()
            .ok_or_else(|| InterpreterError::runtime("index must be numeric"))?
            as usize;
        match target {
            Value::Array(items) => items.get(idx).cloned().ok_or_else(|| {
                InterpreterError::runtime(format!("array index out of bounds: {idx}"))
            }),
            Value::Text(s) => s
                .chars()
                .nth(idx)
                .map(|ch| Value::Text(ch.to_string()))
                .ok_or_else(|| {
                    InterpreterError::runtime(format!("string index out of bounds: {idx}"))
                }),
            _ => Err(InterpreterError::runtime(
                "indexing requires array or string",
            )),
        }
    }

    fn eval_call(&mut self, name: &str, args: &[Expr]) -> Result<Value, InterpreterError> {
        let mut values = Vec::with_capacity(args.len());
        for arg in args {
            values.push(self.eval_expr(arg)?);
        }

        if builtins::is_builtin(name) {
            return builtins::call(name, &values);
        }

        let def = self.functions.get(name).cloned().ok_or_else(|| {
            InterpreterError::runtime(format!("unknown function or builtin `{name}`"))
        })?;

        if def.params.len() != values.len() {
            return Err(InterpreterError::runtime(format!(
                "function `{name}` expected {} args, got {}",
                def.params.len(),
                values.len()
            )));
        }

        self.env.enter_scope();
        for (param, value) in def.params.iter().zip(values.into_iter()) {
            self.env.define(param.clone(), value, false);
        }

        let result = match self.eval_block(&def.body)? {
            ExecResult::Value(v) | ExecResult::Return(v) => v,
            ExecResult::Break => {
                return Err(InterpreterError::runtime("`break` used outside of a loop"))
            }
            ExecResult::Continue => {
                return Err(InterpreterError::runtime(
                    "`continue` used outside of a loop",
                ))
            }
        };
        self.env.exit_scope();
        Ok(result)
    }

    fn eval_binary(
        &mut self,
        op: &BinaryOp,
        left: &Expr,
        right: &Expr,
    ) -> Result<Value, InterpreterError> {
        match op {
            BinaryOp::And => {
                let l = self.eval_expr(left)?;
                if !l.is_truthy() {
                    return Ok(Value::Bool(false));
                }
                Ok(Value::Bool(self.eval_expr(right)?.is_truthy()))
            }
            BinaryOp::Or => {
                let l = self.eval_expr(left)?;
                if l.is_truthy() {
                    return Ok(Value::Bool(true));
                }
                Ok(Value::Bool(self.eval_expr(right)?.is_truthy()))
            }
            _ => {
                let left = self.eval_expr(left)?;
                let right = self.eval_expr(right)?;
                self.eval_materialized_binary(op, left, right)
            }
        }
    }

    fn eval_materialized_binary(
        &self,
        op: &BinaryOp,
        left: Value,
        right: Value,
    ) -> Result<Value, InterpreterError> {
        match op {
            BinaryOp::Add => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                (Value::Text(a), Value::Text(b)) => Ok(Value::Text(a + &b)),
                _ => Err(InterpreterError::runtime(
                    "`+` expects number+number or text+text",
                )),
            },
            BinaryOp::Sub => num_op("-", left, right, |a, b| a - b),
            BinaryOp::Mul => num_op("*", left, right, |a, b| a * b),
            BinaryOp::Div => num_op("/", left, right, |a, b| a / b),
            BinaryOp::Mod => num_op("%", left, right, |a, b| a % b),
            BinaryOp::Eq => Ok(Value::Bool(left == right)),
            BinaryOp::Ne => Ok(Value::Bool(left != right)),
            BinaryOp::Gt => cmp_op(">", left, right, |a, b| a > b),
            BinaryOp::Ge => cmp_op(">=", left, right, |a, b| a >= b),
            BinaryOp::Lt => cmp_op("<", left, right, |a, b| a < b),
            BinaryOp::Le => cmp_op("<=", left, right, |a, b| a <= b),
            BinaryOp::And | BinaryOp::Or => unreachable!("handled in short-circuit path"),
        }
    }
}

fn num_op(
    op: &str,
    left: Value,
    right: Value,
    f: impl FnOnce(f64, f64) -> f64,
) -> Result<Value, InterpreterError> {
    match (left.as_number(), right.as_number()) {
        (Some(a), Some(b)) => Ok(Value::Number(f(a, b))),
        _ => Err(InterpreterError::runtime(format!(
            "`{op}` expects number operands"
        ))),
    }
}

fn cmp_op(
    op: &str,
    left: Value,
    right: Value,
    f: impl FnOnce(f64, f64) -> bool,
) -> Result<Value, InterpreterError> {
    match (left.as_number(), right.as_number()) {
        (Some(a), Some(b)) => Ok(Value::Bool(f(a, b))),
        _ => Err(InterpreterError::runtime(format!(
            "`{op}` expects number operands"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluates_math() {
        let mut interpreter = Interpreter::new();
        let out = interpreter
            .eval_program("let x = 10; let y = x * 4 - 5; y")
            .expect("evaluation should succeed");
        assert_eq!(out, Value::Number(35.0));
    }

    #[test]
    fn supports_arrays_indexing_and_len() {
        let mut interpreter = Interpreter::new();
        let out = interpreter
            .eval_program("let arr = [10, 20, 30]; arr[1] + len(arr)")
            .expect("evaluation should succeed");
        assert_eq!(out, Value::Number(23.0));
    }

    #[test]
    fn supports_compound_assignment() {
        let mut interpreter = Interpreter::new();
        let out = interpreter
            .eval_program("var x = 20; x %= 6; x += 10; x")
            .expect("evaluation should succeed");
        assert_eq!(out, Value::Number(12.0));
    }

    #[test]
    fn supports_if_else_and_comparisons() {
        let mut interpreter = Interpreter::new();
        let out = interpreter
            .eval_program("var x = 0; if 3 > 2 { x = 42; } else { x = 5; } x")
            .expect("evaluation should succeed");
        assert_eq!(out, Value::Number(42.0));
    }

    #[test]
    fn supports_while_loops() {
        let mut interpreter = Interpreter::new();
        let out = interpreter
            .eval_program("var i = 0; var sum = 0; while i < 5 { sum = sum + i; i = i + 1; } sum")
            .expect("evaluation should succeed");
        assert_eq!(out, Value::Number(10.0));
    }

    #[test]
    fn respects_immutable_bindings() {
        let mut interpreter = Interpreter::new();
        let err = interpreter
            .eval_program("let x = 1; x = 2;")
            .expect_err("should reject assignment to let");
        assert!(err.to_string().contains("immutable"));
    }

    #[test]
    fn supports_block_comments() {
        let mut interpreter = Interpreter::new();
        let out = interpreter
            .eval_program("var x = 1; /* outer /* inner */ end */ x += 4; x")
            .expect("evaluation should succeed");
        assert_eq!(out, Value::Number(5.0));
    }

    #[test]
    fn supports_user_functions_and_recursion() {
        let mut interpreter = Interpreter::new();
        let out = interpreter
            .eval_program(
                "fn fib(n) { if n <= 1 { return n; } return fib(n - 1) + fib(n - 2); } fib(8)",
            )
            .expect("evaluation should succeed");
        assert_eq!(out, Value::Number(21.0));
    }

    #[test]
    fn supports_for_loops_with_break_and_continue() {
        let mut interpreter = Interpreter::new();
        let out = interpreter
            .eval_program(
                "var sum = 0; for x in [1,2,3,4,5] { if x == 3 { continue; } if x == 5 { break; } sum += x; } sum",
            )
            .expect("evaluation should succeed");
        assert_eq!(out, Value::Number(7.0));
    }

    #[test]
    fn supports_for_loops_over_strings() {
        let mut interpreter = Interpreter::new();
        let out = interpreter
            .eval_program("var count = 0; for ch in \"abc\" { count += 1; } count")
            .expect("evaluation should succeed");
        assert_eq!(out, Value::Number(3.0));
    }

    #[test]
    fn supports_for_loops_over_exclusive_numeric_ranges() {
        let mut interpreter = Interpreter::new();
        let out = interpreter
            .eval_program("var sum = 0; for i in 0..5 { sum += i; } sum")
            .expect("evaluation should succeed");
        assert_eq!(out, Value::Number(10.0));
    }

    #[test]
    fn supports_for_loops_over_inclusive_numeric_ranges() {
        let mut interpreter = Interpreter::new();
        let out = interpreter
            .eval_program("var sum = 0; for i in 1..=4 { sum += i; } sum")
            .expect("evaluation should succeed");
        assert_eq!(out, Value::Number(10.0));
    }
}
