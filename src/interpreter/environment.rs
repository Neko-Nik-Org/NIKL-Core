use std::collections::HashMap;

use super::engine::Value;
use super::builtins::{builtin_print, builtin_len};


#[derive(Debug, Clone)]
pub struct VariableEntry {
    value: Value,
    mutable: bool,
}

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, VariableEntry>,
    parent: Option<Box<Environment>>,
}


impl Environment {
    pub fn new() -> Self {
        let mut env = Self {
            values: HashMap::new(),
            parent: None,
        };

        env.define("print", Value::BuiltinFunction(builtin_print), false).unwrap();
        env.define("len", Value::BuiltinFunction(builtin_len), false).unwrap();
        env
    }

    pub fn with_parent(parent: Environment) -> Self {
        Self {
            values: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(entry) = self.values.get(name) {
            Some(entry.value.clone())
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            None
        }
    }


    pub fn define(&mut self, name: &str, value: Value, mutable: bool) -> Result<(), String> {
        if self.values.contains_key(name) {
            return Err(format!("Variable '{}' is already declared in this scope", name));
        }
        self.values.insert(name.to_string(), VariableEntry { value, mutable });
        Ok(())
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if let Some(entry) = self.values.get_mut(name) {
            if !entry.mutable {
                return Err(format!("Cannot assign to constant '{}'", name));
            }
            entry.value = value;
            return Ok(());
        } else if let Some(parent) = self.parent.as_mut() {
            return parent.assign(name, value);
        }

        Err(format!("Variable '{}' is not defined", name))
    }
}
