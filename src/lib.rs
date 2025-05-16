//! NIKL-Core: A lightweight scripting language runtime.
//!
//! This is the main library entry point for the NIKL interpreter engine.
//! It exposes the interpreter modules and public APIs required to embed or extend the language.

#![warn(missing_docs)]

pub mod lexer;
pub mod parser;
pub mod interpreter;

pub use interpreter::engine::Interpreter;
pub use interpreter::environment::Environment;
pub use lexer::token::{Token, TokenKind};
pub use parser::ast::{Expr, Stmt};

/// Run a script string using the interpreter.
///
/// # Arguments
/// * `source` - A string slice of the script to interpret.
///
/// # Example
/// ```
/// use nikl::run_script;
///
/// run_script("print(\"Hello from NIKL!\")");
/// ```
pub fn run_script(source: &str) -> Result<(), String> {
    let lexer = lexer::Lexer::new(source);
    match lexer.tokenize() {
        Ok(tokens) => {
            let mut parser = parser::Parser::new(tokens);
            let stmts = parser.parse().map_err(|e| e.to_string())?;
            let mut interpreter = Interpreter::new();
            interpreter.run(&stmts)
        },
        Err(_) => Err(format!("Lexer error")),
    }
}
