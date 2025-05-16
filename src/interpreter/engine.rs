use std::fmt;

use crate::parser::{Expr, Stmt};
use crate::lexer::TokenKind;
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
    Null, // for statements with no return (like print)
}


pub struct Interpreter {
    env: Environment,
}


impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    pub fn run(&mut self, stmts: &[Stmt]) -> Result<(), String> {
        for stmt in stmts {
            self.exec_stmt(stmt)?;
        }
        Ok(())
    }

    fn exec_stmt(&mut self, stmt: &Stmt) -> Result<Option<Value>, String> {
        match stmt {
            Stmt::Let { name, value } => {
                let val = self.eval_expr(value)?;
                self.env.define(name, val, true)?;  // mutable
                Ok(None)
            }
            Stmt::Const { name, value } => {
                let val = self.eval_expr(value)?;
                self.env.define(name, val, false)?;  // immutable
                Ok(None)
            }
            Stmt::Function { name, params, body } => {
                let func = Value::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    closure: self.env.clone(),
                };
                self.env.define(name, func, true)?;
                Ok(None)
            }
            Stmt::Expr(expr) => {
                self.eval_expr(expr)?;
                Ok(None)
            }
            Stmt::If { condition, body, else_body } => {
                let cond_val = self.eval_expr(condition)?;
                if let Value::Bool(true) = cond_val {
                    let local_env = Environment::with_parent(self.env.clone());
                    let mut local_interp = Interpreter { env: local_env };
                    local_interp.run(body)?;
                } else if let Some(else_body) = else_body {
                    let local_env = Environment::with_parent(self.env.clone());
                    let mut local_interp = Interpreter { env: local_env };
                    local_interp.run(else_body)?;
                }
                Ok(None)
            }
            Stmt::Return(expr) => Ok(Some(self.eval_expr(expr)?)),
            // _ => Err("Unsupported statement in basic interpreter".to_string()), // TODO: Give a more specific error message with the line number etc
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Integer(i) => Ok(Value::Integer(*i)),
            Expr::Float(f) => Ok(Value::Float(*f)),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Array(elements) => {
                let mut values = Vec::new();
                for elem in elements {
                    self.eval_expr(elem).map(|v| values.push(v))?;
                }
                Ok(Value::Array(values))
            }
            Expr::HashMap(pairs) => {
                let mut values = Vec::new();
                for (key, value) in pairs {
                    self.eval_expr(key).and_then(|k| {
                        self.eval_expr(value).map(|v| values.push((k, v)))
                    })?;
                }
                Ok(Value::HashMap(values))
            }
            Expr::Tuple(elements) => {
                let mut values = Vec::new();
                for elem in elements {
                    self.eval_expr(elem).map(|v| values.push(v))?;
                }
                Ok(Value::Tuple(values))
            }
            Expr::Identifier(name) => self
                .env
                .get(name)
                .ok_or_else(|| format!("Undefined variable '{}'", name)),
            Expr::Assign { name, value } => {
                let val = self.eval_expr(value)?;
                self.env.assign(name, val.clone())?;
                Ok(val)
            }
            Expr::BinaryOp { left, op, right } => {
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                self.eval_binary_op(&l, op, &r)
            }
            Expr::UnaryOp { op, expr } => {
                let val = self.eval_expr(expr)?;
                self.eval_unary_op(op, &val)
            }
            Expr::Call { function, args } => {
                let func_val = self.eval_expr(function)?;
                let arg_values: Result<Vec<Value>, String> = args.iter().map(|arg| self.eval_expr(arg)).collect();

                match func_val {
                    Value::Function { name, params, body, closure } => {
                        if params.len() != args.len() {
                            return Err(format!(
                                "Function '{}' expects {} arguments, but got {}",
                                name,
                                params.len(),
                                args.len()
                            ));
                        }

                        let mut local_env = Environment::with_parent(closure);

                        for (param, arg_expr) in params.iter().zip(args.iter()) {
                            let arg_val = self.eval_expr(arg_expr)?;
                            local_env.define(param, arg_val, true)?;
                        }

                        let mut local_interpreter = Interpreter { env: local_env };

                        for stmt in body {
                            if let Stmt::Return(ret_expr) = stmt {
                                return Ok(local_interpreter.eval_expr(&ret_expr)?);
                            } else {
                                local_interpreter.exec_stmt(&stmt)?;
                            }
                        }

                        Ok(Value::Bool(true)) // default return value
                    }
                    Value::BuiltinFunction(f) => {
                        f(arg_values?)
                    }
                    _ => Err("Tried to call non-function".into()),
                }
            }
            // _ => Err("Unsupported expression in basic interpreter".to_string()),
        }
    }

    fn eval_binary_op(&self, left: &Value, op: &TokenKind, right: &Value) -> Result<Value, String> {
        // Helper function to handle division to avoid division by zero
        fn divide(left: Value, right: Value) -> Result<Value, String> {
            match (left, right) {
                (Value::Integer(l), Value::Integer(r)) => {
                    if r == 0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(Value::Integer(l / r))
                    }
                }
                (Value::Float(l), Value::Float(r)) => {
                    if r == 0.0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(Value::Float(l / r))
                    }
                }
                (Value::Integer(l), Value::Float(r)) => {
                    if r == 0.0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(Value::Float(l as f64 / r))
                    }
                }
                (Value::Float(l), Value::Integer(r)) => {
                    if r == 0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(Value::Float(l / r as f64))
                    }
                }
                _ => Err("Invalid division operation".to_string()),
            }
        }

        match (left, right) {
            // int, int
            (Value::Integer(l), Value::Integer(r)) => match op {
                TokenKind::Add => Ok(Value::Integer(l + r)),
                TokenKind::Subtract => Ok(Value::Integer(l - r)),
                TokenKind::Multiply => Ok(Value::Integer(l * r)),
                TokenKind::Divide => Ok(divide(Value::Integer(*l), Value::Integer(*r))?),
                TokenKind::Equals => Ok(Value::Bool(l == r)),
                TokenKind::NotEqual => Ok(Value::Bool(l != r)),
                TokenKind::LessThan => Ok(Value::Bool(l < r)),
                TokenKind::GreaterThan => Ok(Value::Bool(l > r)),
                _ => Err(format!("Unsupported operator: {:?}", op)),
            },
            // float, float
            (Value::Float(l), Value::Float(r)) => match op {
                TokenKind::Add => Ok(Value::Float(l + r)),
                TokenKind::Subtract => Ok(Value::Float(l - r)),
                TokenKind::Multiply => Ok(Value::Float(l * r)),
                TokenKind::Divide => Ok(divide(Value::Float(*l), Value::Float(*r))?),
                TokenKind::Equals => Ok(Value::Bool(l == r)),
                TokenKind::NotEqual => Ok(Value::Bool(l != r)),
                TokenKind::LessThan => Ok(Value::Bool(l < r)),
                TokenKind::GreaterThan => Ok(Value::Bool(l > r)),
                _ => Err(format!("Unsupported operator: {:?}", op)),
            },
            // string, string
            (Value::String(l), Value::String(r)) => match op {
                TokenKind::Add => Ok(Value::String(format!("{}{}", l, r))),
                TokenKind::Equals => Ok(Value::Bool(l == r)),
                TokenKind::NotEqual => Ok(Value::Bool(l != r)),
                _ => Err(format!("Unsupported operator: {:?}", op)),
            },
            // bool, bool
            (Value::Bool(l), Value::Bool(r)) => match op {
                TokenKind::And => Ok(Value::Bool(*l && *r)),
                TokenKind::Or => Ok(Value::Bool(*l || *r)),
                TokenKind::Equals => Ok(Value::Bool(l == r)),
                TokenKind::NotEqual => Ok(Value::Bool(l != r)),
                _ => Err(format!("Unsupported operator: {:?}", op)),
            },
            // int, float
            (Value::Integer(l), Value::Float(r)) => match op {
                TokenKind::Add => Ok(Value::Float(*l as f64 + *r)),
                TokenKind::Subtract => Ok(Value::Float(*l as f64 - *r)),
                TokenKind::Multiply => Ok(Value::Float(*l as f64 * *r)),
                TokenKind::Divide => Ok(divide(Value::Integer(*l), Value::Float(*r))?),
                TokenKind::Equals => Ok(Value::Bool(*l as f64 == *r)),
                TokenKind::NotEqual => Ok(Value::Bool(*l as f64 != *r)),
                TokenKind::LessThan => Ok(Value::Bool((*l as f64) < *r)),
                TokenKind::GreaterThan => Ok(Value::Bool((*l as f64) > *r)),
                _ => Err(format!("Unsupported operator: {:?}", op)),
            },
            // float, int
            (Value::Float(l), Value::Integer(r)) => match op {
                TokenKind::Add => Ok(Value::Float(*l + *r as f64)),
                TokenKind::Subtract => Ok(Value::Float(*l - *r as f64)),
                TokenKind::Multiply => Ok(Value::Float(*l * *r as f64)),
                TokenKind::Divide => Ok(divide(Value::Float(*l), Value::Integer(*r))?),
                TokenKind::Equals => Ok(Value::Bool(*l == *r as f64)),
                TokenKind::NotEqual => Ok(Value::Bool(*l != *r as f64)),
                TokenKind::LessThan => Ok(Value::Bool(*l < *r as f64)),
                TokenKind::GreaterThan => Ok(Value::Bool(*l > *r as f64)),
                _ => Err(format!("Unsupported operator: {:?}", op)),
            },
            _ => Err(format!("Type error: {:?} {:?} {:?}", left, op, right)),
        }
    }

    fn eval_unary_op(&self, op: &TokenKind, val: &Value) -> Result<Value, String> {
        match (op, val) {
            (TokenKind::Subtract, Value::Integer(i)) => Ok(Value::Integer(-i)),
            (TokenKind::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
            _ => Err(format!("Unsupported unary operation: {:?} {:?}", op, val)),
        }
    }
}


impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Bool(b) => write!(f, "{}", b),
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
                let formatted_pairs: Vec<String> = pairs
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect();
                write!(f, "{{{}}}", formatted_pairs.join(", "))
            }
            Value::Function { name, .. } => write!(f, "<function {} at {:p}>", name, name),
            Value::BuiltinFunction(_) => write!(f, "<builtin function>"),
        }
    }
}
