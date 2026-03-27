use std::collections::HashMap;

use crate::interp::InterpreterError;
use crate::value::Value;

#[derive(Debug, Clone)]
struct Binding {
    value: Value,
    mutable: bool,
}

#[derive(Default, Debug)]
pub struct Env {
    scopes: Vec<HashMap<String, Binding>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn define(&mut self, name: String, value: Value, mutable: bool) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, Binding { value, mutable });
        }
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), InterpreterError> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(binding) = scope.get_mut(name) {
                if !binding.mutable {
                    return Err(InterpreterError::runtime(format!(
                        "cannot assign to immutable binding `{name}`"
                    )));
                }
                binding.value = value;
                return Ok(());
            }
        }
        Err(InterpreterError::runtime(format!(
            "undefined variable `{name}`"
        )))
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).map(|binding| &binding.value))
    }
}
