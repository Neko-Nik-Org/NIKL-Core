use std::collections::HashMap;

use crate::parser::{Expr, Stmt};
use crate::lexer::TokenKind;


#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
        closure: Environment,
    },
    Null, // for statements with no return (like print)
}

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, VariableEntry>,
    parent: Option<Box<Environment>>,
}

#[derive(Debug, Clone)]
pub struct VariableEntry {
    value: Value,
    mutable: bool,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            parent: None,
        }
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
            Stmt::Print(expr) => {
                let val = self.eval_expr(expr)?;
                match val {
                    Value::Bool(b) => println!("{}", if b { "True" } else { "False" }),
                    Value::Integer(i) => println!("{}", i),
                    Value::Float(f) => println!("{}", f),
                    Value::String(s) => println!("{}", s),
                    Value::Null => println!("None"),
                    Value::Function { name, .. } => {println!("<function {} at {:#x}>", name, &name as *const _ as usize)},
                }
                Ok(Some(Value::Null))
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
            // _ => Err("Unsupported statement in basic interpreter".to_string()), // TODO: Print a more specific error message with the line number etc
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
                    _ => Err("Tried to call non-function".into()),
                }
            }
            // _ => Err("Unsupported expression in basic interpreter".to_string()),
        }
    }

    fn eval_binary_op(&self, left: &Value, op: &TokenKind, right: &Value) -> Result<Value, String> {

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




// --- Test cases ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;


    fn run_interpreter(input: &str) -> Result<(), String> {
        let lexer = Lexer::new(input);
        match lexer.tokenize() {
            Ok(tokens) => {
                let mut parser = Parser::new(tokens);
                let stmts = parser.parse().map_err(|e| e.to_string())?;
                let mut interpreter = Interpreter::new();
                interpreter.run(&stmts)
            },
            Err(_) => {Err(format!("Lexer error"))}
        }
    }


    #[test]
    fn test_variable_declaration_and_assignment() {
        let input = r#"
            let x = 10
            let y = 20
            x = x + y
            print(x)    // should print 30
        "#;

        let result = run_interpreter(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_constants() {
        let input = r#"
            const x = 42
            print(x)
        "#;

        let result = run_interpreter(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_assignment_error_on_const() {
        let input = r#"
            const x = 5;
            x = 10;
        "#;

        let result = run_interpreter(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_binary_operations() {
        let input = r#"
            let a = 5 + 2 * 3
            let b = a - 4 / 2
            print(b)    // should print 9
        "#;

        let result = run_interpreter(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_if_statement_true_branch() {
        let input = r#"
            let x = 10
            if (x > 5) {
                print("greater")
            } else {
                print("less")
            }
        "#;

        let result = run_interpreter(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_if_statement_false_branch() {
        let input = r#"
            let x = 3
            if (x > 5) {
                print("greater")
            } else {
                print("less")
            }
        "#;

        let result = run_interpreter(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_function_definition_and_call() {
        let input = r#"
            fn add(a, b) {
                return a + b
            }

            let result = add(3, 4)
            print(result)   // should print 7
        "#;

        let result = run_interpreter(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_function_with_if_and_return() {
        let input = r#"
            fn max(a, b) {
                if (a > b) {
                    return a
                } else {
                    return b
                }
            }

            let m = max(7, 4)
            print(m)    // should print 7
        "#;

        let result = run_interpreter(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_nested_function_calls() {
        let input = r#"
            fn square(x) {
                return x * x
            }

            fn double(x) {
                return x + x
            }

            let result = square(double(3))
            print(result)   // should print 36
        "#;

        let result = run_interpreter(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_variable_shadowing_in_nested_scope() {
        let input = r#"
            let x = 5
            fn foo() {
                let x = 10
                print(x)    // should print 10
            }
            foo()
            print(x)        // should print 5
        "#;

        let result = run_interpreter(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_division_by_zero() {
        let input = r#"
            let a = 10 / 0
            print(a)
        "#;

        let result = run_interpreter(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_boolean_logic_operations() {
        let input = r#"
            let a = True and False
            let b = not a
            let c = b or False
            print(c)    // should print True
        "#;

        let result = run_interpreter(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_function_closure_scope() {
        let input = r#"
            let x = 100

            fn show() {
                print(x)    // should print 100 because of closure
            }

            show()
        "#;

        let result = run_interpreter(input);
        assert!(result.is_ok());
    }
}
