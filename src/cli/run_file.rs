use std::fs;
use log::{debug, log_enabled, Level};

use crate::{lexer::{Lexer, LexError, Token}, parser::Parser, interpreter::Interpreter};

fn check_file_is_valid(filename: &str) -> bool {
    match fs::metadata(filename) {
        Ok(metadata) if metadata.is_file() && filename.ends_with(".nk") => {
            if metadata.len() > 0 {
                true
            } else {
                log::error!("Error: Script '{}' is empty.", filename);
                false
            }
        }
        Ok(_) => {
            log::error!("Error: File '{}' is not a valid script, it should end with .nk", filename);
            false
        }
        Err(_) => {
            log::error!("Error: File '{}' does not exist.", filename);
            false
        }
    }
}

fn read_file(filename: &str) -> Option<String> {
    if !check_file_is_valid(filename) {
        return None;
    }

    match fs::read_to_string(filename) {
        Ok(content) => Some(content),
        Err(e) => {
            log::error!("Error reading file '{}': {}", filename, e);
            None
        }
    }
}

fn tokenize_input(input: &str) -> Result<Vec<Token>, LexError> {
    let lexer = Lexer::new(input);
    lexer.tokenize()
}

fn parse_tokens(tokens: Vec<Token>) -> Result<Vec<crate::parser::Stmt>, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

fn interpret_statements(stmts: &[crate::parser::Stmt]) -> Result<(), String> {
    let mut interpreter = Interpreter::new();
    interpreter.run(stmts)
}

pub fn run_file(filename: &str) {
    if let Some(content) = read_file(filename) {
        match tokenize_input(&content) {
            Ok(tokens) => {
                if log_enabled!(Level::Debug) {
                    debug!("Parsing tokens from file: {}", filename);
                    for token in &tokens {
                        println!("{:?}", token);
                    }
                }

                match parse_tokens(tokens.clone()) {
                    Ok(stmts) => {
                        if log_enabled!(Level::Debug) {
                            debug!("Parsed statements from tokens:");
                            for stmt in &stmts {
                                debug!("{:?}", stmt);
                            }
                        }

                        match interpret_statements(&stmts) {
                            Ok(_) => log::info!("Script executed successfully."),
                            Err(e) => eprintln!("Error executing script: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Error parsing statements: {}", e),
                }
            }
            Err(e) => match e {
                LexError::UnexpectedChar(ch, line, col) => {
                    eprintln!("Unexpected character '{}' at line {}, column {}", ch, line, col);
                }
                LexError::UnterminatedString(line, col) => {
                    eprintln!("Unterminated string starting at line {}, column {}", line, col);
                }
                LexError::InvalidNumber(num, line, col) => {
                    eprintln!("Invalid number '{}' at line {}, column {}", num, line, col);
                }
            },
        }
    } else {
        eprintln!("Failed to read or validate the file.");
    }
}
