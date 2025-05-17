use std::fmt;
use crate::parser::Stmt;
use super::environment::Environment;


#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Array(Vec<Value>),
    HashMap(Vec<(Value, Value)>),
    Tuple(Vec<Value>),
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
        closure: Environment,
    },
    BuiltinFunction(fn(Vec<Value>) -> Result<Value, String>),
    Null,
}


impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Bool(b) => write!(f, "{}", if *b { "True" } else { "False" }),
            Value::String(s) => write!(f, "{}", s),
            Value::Null => write!(f, "None"),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", items.join(", "))
            }
            Value::Tuple(items) => {
                let elements: Vec<String> = items.iter().map(|v| v.to_string()).collect();
                write!(f, "({})", elements.join(", "))
            }
            Value::HashMap(pairs) => {
                let formatted: Vec<String> = pairs.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
                write!(f, "{{{}}}", formatted.join(", "))
            }
            Value::Function { name, .. } => write!(f, "<function {}>", name),
            Value::BuiltinFunction(_) => write!(f, "<builtin function>"),
        }
    }
}
