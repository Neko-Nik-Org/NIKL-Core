use std::collections::HashMap;

use super::value::Value;
use super::builtins::{
    builtin_print,
    builtin_len,
    builtin_str,
    builtin_int,
    builtin_float,
    builtin_bool,
    builtin_exit,
    builtin_type,
    builtin_input
};


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


impl VariableEntry {
    pub fn value(&self) -> &Value {
        &self.value
    }
}


impl Environment {
    pub fn new() -> Self {
        let mut env = Self {
            values: HashMap::new(),
            parent: None,
        };

        env.define("print", Value::BuiltinFunction(builtin_print), false).unwrap();
        env.define("len", Value::BuiltinFunction(builtin_len), false).unwrap();
        env.define("str", Value::BuiltinFunction(builtin_str), false).unwrap();
        env.define("int", Value::BuiltinFunction(builtin_int), false).unwrap();
        env.define("float", Value::BuiltinFunction(builtin_float), false).unwrap();
        env.define("bool", Value::BuiltinFunction(builtin_bool), false).unwrap();
        env.define("exit", Value::BuiltinFunction(builtin_exit), false).unwrap();
        env.define("type", Value::BuiltinFunction(builtin_type), false).unwrap();
        env.define("input", Value::BuiltinFunction(builtin_input), false).unwrap();
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

    pub fn flatten(&self) -> HashMap<String, VariableEntry> {
        let mut map = HashMap::new();
        if let Some(parent) = &self.parent {
            map.extend(parent.flatten());
        }
        map.extend(self.values.clone());
        map
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
            Ok(())
        } else if let Some(parent) = self.parent.as_mut() {
            parent.assign(name, value)
        } else {
            Err(format!("Variable '{}' is not defined", name))
        }
    }

    pub fn delete(&mut self, name: &str) -> Result<(), String> {
        if self.values.remove(name).is_some() {
            Ok(())
        } else if let Some(parent) = self.parent.as_mut() {
            parent.delete(name)
        } else {
            Err(format!("Variable '{}' is not defined", name))
        }
    }
}
