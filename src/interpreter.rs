use std::collections::HashMap;

use crate::parser::{Expr, Stmt};
use crate::lexer::TokenKind;


#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Null, // for statements with no return (like print)
}

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: &str, val: Value) {
        self.values.insert(name.to_string(), val);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.values.get(name).cloned()
    }
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
            Stmt::Let { name, value } | Stmt::Const { name, value } => {
                let val = self.eval_expr(value)?;
                self.env.set(name, val);
                Ok(None)
            }
            Stmt::Print(expr) => {
                let val = self.eval_expr(expr)?;
                println!("{:?}", val);  // TODO: Use a proper format function to print values: Bool(false) -> False
                Ok(Some(Value::Null))
            }
            Stmt::Expr(expr) => {
                self.eval_expr(expr)?;
                Ok(None)
            }
            Stmt::If { condition, body, else_body } => {
                let cond_val = self.eval_expr(condition)?;
                if let Value::Bool(true) = cond_val {
                    self.run(body)?;
                } else if let Some(else_body) = else_body {
                    self.run(else_body)?;
                }
                Ok(None)
            }
            Stmt::Return(expr) => Ok(Some(self.eval_expr(expr)?)),
            _ => Err("Unsupported statement in basic interpreter".to_string()), // TODO: Print a more specific error message with the line number etc
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Integer(i) => Ok(Value::Integer(*i)),
            Expr::Float(f) => Ok(Value::Float(*f)),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Identifier(name) => self
                .env
                .get(name)
                .ok_or_else(|| format!("Undefined variable '{}'", name)),
            Expr::Assign { name, value } => {
                let val = self.eval_expr(value)?;
                self.env.set(name, val.clone());
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
            _ => Err("Unsupported expression in basic interpreter".to_string()),
        }
    }

    fn eval_binary_op(&self, left: &Value, op: &TokenKind, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Integer(l), Value::Integer(r)) => match op {
                TokenKind::Add => Ok(Value::Integer(l + r)),
                TokenKind::Subtract => Ok(Value::Integer(l - r)),
                TokenKind::Multiply => Ok(Value::Integer(l * r)),
                TokenKind::Divide => Ok(Value::Integer(l / r)),
                TokenKind::Equals => Ok(Value::Bool(l == r)),
                TokenKind::NotEqual => Ok(Value::Bool(l != r)),
                TokenKind::LessThan => Ok(Value::Bool(l < r)),
                TokenKind::GreaterThan => Ok(Value::Bool(l > r)),
                _ => Err(format!("Unsupported operator: {:?}", op)),
            },
            (Value::Bool(l), Value::Bool(r)) => match op {
                TokenKind::And => Ok(Value::Bool(*l && *r)),
                TokenKind::Or => Ok(Value::Bool(*l || *r)),
                TokenKind::Equals => Ok(Value::Bool(l == r)),
                TokenKind::NotEqual => Ok(Value::Bool(l != r)),
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
