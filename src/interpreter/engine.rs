use std::collections::HashSet;
use std::path::PathBuf;

use crate::parser::{Expr, Stmt};
use crate::lexer::TokenKind;
use super::environment::Environment;
use super::value::Value;


pub struct Interpreter {
    env: Environment,
    loaded_modules: HashSet<String>,
    base_path: PathBuf,
}


#[derive(Debug, Clone)]
pub enum ControlFlow {
    Value,      // A normal result (like from evaluating an expression)
    Return(Value),     // A return statement
    Break,             // For loops (Not yet implemented)
    Continue,          // For loops (Not yet implemented)
    // Yield,            // For generators (Not yet implemented)
    // Exception(String), // For exceptions (Not yet implemented)
}


impl Interpreter {
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            env: Environment::new(),
            loaded_modules: HashSet::new(),
            base_path,
        }
    }

    pub fn run(&mut self, stmts: &[Stmt]) -> Result<ControlFlow, String> {
        for stmt in stmts {
            match self.exec_stmt(stmt)? {
                ControlFlow::Value => continue,
                cf => return Ok(cf), // Return, Break, Continue — bubble up
            }
        }
        Ok(ControlFlow::Value)
    }

    fn exec_stmt(&mut self, stmt: &Stmt) -> Result<ControlFlow, String> {
        match stmt {
            Stmt::Let { name, value } => {
                let val = self.eval_expr(value)?;
                self.env.define(name, val, true)?;  // mutable
                Ok(ControlFlow::Value)
            }
            Stmt::Const { name, value } => {
                let val = self.eval_expr(value)?;
                self.env.define(name, val, false)?;  // immutable
                Ok(ControlFlow::Value)
            }
            Stmt::Function { name, params, body } => {
                let func = Value::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    closure: self.env.clone(),
                };
                self.env.define(name, func, true)?;
                Ok(ControlFlow::Value)
            }
            Stmt::Loop(body) => {
                loop {
                    for stmt in body {
                        match self.exec_stmt(stmt)? {
                            ControlFlow::Break => return Ok(ControlFlow::Value),
                            ControlFlow::Continue => break, // Skip to next iteration
                            ControlFlow::Value => continue,
                            cf => return Ok(cf), // Return bubbles up
                        }
                    }
                }
            }
            Stmt::While { condition, body } => {
                while let Value::Bool(true) = self.eval_expr(condition)? {
                    for stmt in body {
                        match self.exec_stmt(stmt)? {
                            ControlFlow::Break => return Ok(ControlFlow::Value),
                            ControlFlow::Continue => break, // Skip to next iteration
                            ControlFlow::Value => continue,
                            cf => return Ok(cf), // Return bubbles up
                        }
                    }
                }
                Ok(ControlFlow::Value)
            }
            Stmt::For { names, iterable, body } => {
                let iter_val = self.eval_expr(iterable)?;
                match iter_val {
                    Value::String(s) => {
                        // There should be only one name in the names vector
                        if names.len() != 1 {
                            return Err(format!("'for' loop requires exactly one name for type 'String', got {:?}", names));
                        }
                        let name = &names[0];
                        self.env.define(name, Value::Null, true)?; // mutable
                        for c in s.chars() {
                            self.env.assign(name, Value::String(c.to_string()))?;
                            for stmt in body {
                                match self.exec_stmt(stmt)? {
                                    ControlFlow::Break => return Ok(ControlFlow::Value),
                                    ControlFlow::Continue => break, // Skip to next iteration
                                    ControlFlow::Value => continue,
                                    cf => return Ok(cf), // Return bubbles up
                                }
                            }
                        }
                    }
                    Value::Array(elements) => {
                        // There should be only one name in the names vector
                        if names.len() != 1 {
                            return Err(format!("'for' loop requires exactly one name for type 'Array', got {:?}", names));
                        }
                        let name = &names[0];
                        self.env.define(name, Value::Null, true)?; // mutable
                        for elem in elements {
                            self.env.assign(name, elem.clone())?;
                            for stmt in body {
                                match self.exec_stmt(stmt)? {
                                    ControlFlow::Break => return Ok(ControlFlow::Value),
                                    ControlFlow::Continue => break, // Skip to next iteration
                                    ControlFlow::Value => continue,
                                    cf => return Ok(cf), // Return bubbles up
                                }
                            }
                        }
                    }
                    Value::Tuple(elements) => {
                        // There should be only one name in the names vector
                        if names.len() != 1 {
                            return Err(format!("'for' loop requires exactly one name for type 'Tuple', got {:?}", names));
                        }
                        let name = &names[0];
                        self.env.define(name, Value::Null, true)?; // mutable
                        for elem in elements {
                            self.env.assign(name, elem.clone())?;
                            for stmt in body {
                                match self.exec_stmt(stmt)? {
                                    ControlFlow::Break => return Ok(ControlFlow::Value),
                                    ControlFlow::Continue => break, // Skip to next iteration
                                    ControlFlow::Value => continue,
                                    cf => return Ok(cf), // Return bubbles up
                                }
                            }
                        }
                    }
                    Value::HashMap(pairs) => {
                        // There should be two names in the names vector, one for key and one for value
                        if names.len() != 2 {
                            return Err(format!("'for' loop requires exactly two names for type 'HashMap', got {:?}", names));
                        }
                        let key_name = &names[0];
                        let value_name = &names[1];
                        self.env.define(key_name, Value::Null, true)?; // mutable
                        self.env.define(value_name, Value::Null, true)?; // mutable
                        for (key, value) in pairs {
                            if let Value::String(s) = key {
                                self.env.assign(key_name, Value::String(s.clone()))?;
                            }
                            self.env.assign(value_name, value.clone())?;
                            for stmt in body {
                                match self.exec_stmt(stmt)? {
                                    ControlFlow::Break => return Ok(ControlFlow::Value),
                                    ControlFlow::Continue => break, // Skip to next iteration
                                    ControlFlow::Value => continue,
                                    cf => return Ok(cf), // Return bubbles up
                                }
                            }
                        }
                    }
                    _ => return Err(format!("'for' loop requires an iterable, got {:?}", iter_val)),
                }
                Ok(ControlFlow::Value)
            }
            Stmt::Expr(expr) => {
                self.eval_expr(expr)?;
                Ok(ControlFlow::Value)
            }
            Stmt::Delete(name) => {
                self.env.delete(name)?;
                Ok(ControlFlow::Value)
            }
            Stmt::Break => Ok(ControlFlow::Break),
            Stmt::Continue => Ok(ControlFlow::Continue),

            // This will create a new environment and will not update the variable in the current environment
            // Stmt::If { condition, body, else_if_branches, else_body } => {
            //     let cond_val = self.eval_expr(condition)?;
            //     if let Value::Bool(true) = cond_val {
            //         let local_env = Environment::with_parent(self.env.clone());
            //         let mut local_interp = Interpreter { env: local_env, loaded_modules: self.loaded_modules.clone() };
            //         return local_interp.run(body);
            //     } else {
            //         for (else_if_cond, else_if_body) in else_if_branches {
            //             let val = self.eval_expr(else_if_cond)?;
            //             if let Value::Bool(true) = val {
            //                 let local_env = Environment::with_parent(self.env.clone());
            //                 let mut local_interp = Interpreter { env: local_env, loaded_modules: self.loaded_modules.clone() };
            //                 return local_interp.run(else_if_body);
            //             }
            //         }

            //         if let Some(else_body) = else_body {
            //             let local_env = Environment::with_parent(self.env.clone());
            //             let mut local_interp = Interpreter { env: local_env, loaded_modules: self.loaded_modules.clone() };
            //             return local_interp.run(else_body);
            //         }
            //     }

            //     Ok(ControlFlow::Value)
            // }

            // This will update the variable in the current environment
            Stmt::If { condition, body, else_if_branches, else_body } => {
                let cond_val = self.eval_expr(condition)?;
                if let Value::Bool(true) = cond_val {
                    for stmt in body {
                        match self.exec_stmt(stmt)? {
                            ControlFlow::Value => continue,
                            cf => return Ok(cf),
                        }
                    }
                } else {
                    let mut branch_executed = false;
                    for (else_if_cond, else_if_body) in else_if_branches {
                        let val = self.eval_expr(else_if_cond)?;
                        if let Value::Bool(true) = val {
                            for stmt in else_if_body {
                                match self.exec_stmt(stmt)? {
                                    ControlFlow::Value => continue,
                                    cf => return Ok(cf),
                                }
                            }
                            branch_executed = true;
                            break;
                        }
                    }

                    if !branch_executed {
                        if let Some(else_body) = else_body {
                            for stmt in else_body {
                                match self.exec_stmt(stmt)? {
                                    ControlFlow::Value => continue,
                                    cf => return Ok(cf),
                                }
                            }
                        }
                    }
                }

                Ok(ControlFlow::Value)
            }
            Stmt::Import { path, alias } => {
                // Resolve relative to base_path of current interpreter
                let mut final_path = self.base_path.clone();
                final_path.push(path); // appends e.g., "os.nk"

                // Normalize path to avoid duplicates
                let canonical = std::fs::canonicalize(&final_path)
                    .map_err(|_| format!("Failed to read module '{}'", final_path.display()))?;

                if self.loaded_modules.contains(canonical.to_str().unwrap()) {
                    return Ok(ControlFlow::Value);
                }

                let module_code = std::fs::read_to_string(&canonical)
                    .map_err(|_| format!("Failed to read module '{}'", canonical.display()))?;

                let lexer = crate::lexer::Lexer::new(&module_code);
                let tokens = lexer
                    .tokenize()
                    .map_err(|_| format!("Failed to tokenize module '{}'", path))?;

                let mut parser = crate::parser::Parser::new(tokens);
                let module_stmts = parser.parse()?;

                let mut module_interp = Interpreter {
                    env: Environment::new(),
                    loaded_modules: HashSet::new(),
                    base_path: canonical.parent().unwrap().to_path_buf(), // <- important
                };
                module_interp.loaded_modules.insert(canonical.to_string_lossy().to_string());
                module_interp.run(&module_stmts)?;

                let exports: Vec<(Value, Value)> = module_interp.env
                    .flatten()
                    .into_iter()
                    .map(|(k, v)| (Value::String(k), v.value().clone()))
                    .collect();

                self.env.define(&alias, Value::HashMap(exports), false)?;
                self.loaded_modules.insert(canonical.to_string_lossy().to_string());

                Ok(ControlFlow::Value)
            }
            Stmt::Return(expr) => {
                let val = self.eval_expr(expr)?;
                Ok(ControlFlow::Return(val))
            }
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
                                "Function '{}' expects {} arguments, got {}",
                                name,
                                params.len(),
                                args.len()
                            ));
                        }

                        let mut local_env = Environment::with_parent(closure.clone());
                        for (param, arg_expr) in params.iter().zip(args.iter()) {
                            let arg_val = self.eval_expr(arg_expr)?;
                            local_env.define(param, arg_val, true)?;
                        }

                        let mut local_interpreter = Interpreter {
                            env: local_env,
                            loaded_modules: self.loaded_modules.clone(),
                            base_path: self.base_path.clone(),
                        };

                        match local_interpreter.run(&body)? {
                            ControlFlow::Return(val) => Ok(val),
                            _ => Ok(Value::Null),
                        }
                    }
                    Value::BuiltinFunction(f) => f(arg_values?),
                    _ => Err("Tried to call non-function".into()),
                }
            }
            Expr::DotAccess { object, property } => {
                let val = self.eval_expr(object)?;
                match val {
                    Value::HashMap(pairs) => {
                        for (k, v) in pairs {
                            if let Value::String(s) = k {
                                if s == *property {
                                    return Ok(v.clone());
                                }
                            }
                        }
                        Err(format!("Property '{}' not found", property))
                    }
                    _ => Err(format!("Dot access on non-object value: {:?}", val)),
                }
            }
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
                TokenKind::GreaterThanOrEqual => Ok(Value::Bool(l >= r)),
                TokenKind::LessThanOrEqual => Ok(Value::Bool(l <= r)),
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
                TokenKind::GreaterThanOrEqual => Ok(Value::Bool(l >= r)),
                TokenKind::LessThanOrEqual => Ok(Value::Bool(l <= r)),
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
                TokenKind::GreaterThanOrEqual => Ok(Value::Bool((*l as f64) >= *r)),
                TokenKind::LessThanOrEqual => Ok(Value::Bool((*l as f64) <= *r)),
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
                TokenKind::GreaterThanOrEqual => Ok(Value::Bool(*l >= *r as f64)),
                TokenKind::LessThanOrEqual => Ok(Value::Bool(*l <= *r as f64)),
                _ => Err(format!("Unsupported operator: {:?}", op)),
            },
            // string, bool
            (Value::String(l), Value::Bool(r)) => match op {
                TokenKind::Add => Ok(Value::String(format!("{}{}", l, if *r { "True" } else { "False" }))),
                _ => Err(format!("Unsupported operator: {:?}", op)),
            },
            // bool, string
            (Value::Bool(l), Value::String(r)) => match op {
                TokenKind::Add => Ok(Value::String(format!("{}{}", if *l { "True" } else { "False" }, r))),
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
