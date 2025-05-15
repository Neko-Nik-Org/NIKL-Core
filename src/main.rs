use std::env;
use std::fs;
use rustyline::{Editor, history::FileHistory};
use log::{debug, log_enabled, Level};
use rustyline::error::ReadlineError;

use tokio;


use crate::lexer::{Lexer, LexError};
use crate::interpreter::Interpreter;
use crate::parser::Parser;

mod lexer;
mod parser;
mod interpreter;

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

fn create_history_file_if_not_exists(filename: &str) -> std::io::Result<()> {
    let path = std::path::Path::new(filename);
    if !path.exists() {
        fs::create_dir_all(path.parent().unwrap())?;
        fs::File::create(path)?;
    }
    Ok(())
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

fn tokenize_input(input: &str) -> Result<Vec<crate::lexer::Token>, LexError> {
    let lexer = Lexer::new(input);
    lexer.tokenize()
}

fn parse_tokens(tokens: Vec<crate::lexer::Token>) -> Result<Vec<crate::parser::Stmt>, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

fn interpret_statements(stmts: &[crate::parser::Stmt]) -> Result<(), String> {
    let mut interpreter = Interpreter::new();
    interpreter.run(stmts)
}


fn run_file(filename: &str) {
    if let Some(content) = read_file(filename) {
        match tokenize_input(&content) {
            Ok(tokens) => {
                if log_enabled!(Level::Debug) {
                    debug!("Parsing tokens from file: {}", filename);
                    for token in &tokens {
                        println!("{:?}", token);
                    }
                }

                // Pass tokens.clone() to parser because parse_tokens wants ownership
                match parse_tokens(tokens.clone()) {
                    Ok(stmts) => {
                        if log_enabled!(Level::Debug) {
                            debug!("Parsed statements from tokens:");
                            for stmt in &stmts {
                                debug!("{:?}", stmt);
                            }
                        }

                        // Execute the parsed statements
                        match interpret_statements(&stmts) {
                            Ok(_) => log::info!("Script executed successfully."),
                            Err(e) => eprintln!("Error executing script: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Error parsing statements: {}", e),
                }
            }
            // error handling unchanged
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

fn run_repl() -> rustyline::Result<()> {
    println!("Welcome to Nikl REPL!");
    println!("To exit, type 'exit' or press Ctrl+D");

    let mut rl = Editor::<(), FileHistory>::new()?;
    create_history_file_if_not_exists("/tmp/.nikl_history")?;
    if rl.load_history("/tmp/.nikl_history").is_ok() {
        log::debug!("Loaded history from file");
    }

    let mut interpreter = Interpreter::new();

    loop {
        let readline = rl.readline(">>> ");

        match readline {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }
                if input == "exit" {
                    break;
                }
                rl.add_history_entry(input)?;

                match tokenize_input(input) {
                    Ok(tokens) => {
                        if log_enabled!(Level::Debug) {   
                            for token in &tokens {
                                debug!("{:?}", token);
                            }
                        }
                        match parse_tokens(tokens.clone()) {
                            Ok(stmts) => {
                                match interpreter.run(&stmts) {
                                    Ok(_) => (),
                                    Err(e) => eprintln!("Runtime error: {}", e),
                                }
                            }
                            Err(e) => eprintln!("Parse error: {}", e),
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
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C pressed — print message or just continue
                println!("Keyboard Interrupt");  // optional
                continue;        // ignore and continue reading input
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D pressed — exit gracefully
                println!("Exiting REPL.");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    if rl.save_history("/tmp/.nikl_history").is_ok() {
        log::debug!("Saved history to file");
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let cmd_or_file = &args[1];

        // Simple CLI dispatch - extend as needed for other commands
        match cmd_or_file.as_str() {
            "help" => {
                println!("Usage:");
                println!("  nikl            # Start REPL");
                println!("  nikl <file.nk>  # Run script file");
                // add more commands like install, init, etc. here
            }
            file if file.ends_with(".nk") => {
                run_file(file);
            }
            other => {
                eprintln!("Unknown command or invalid file: {}", other);
            }
        }
    } else {
        // Run REPL if no arguments
        if let Err(e) = run_repl() {
            eprintln!("REPL exited with error: {}", e);
        }
    }
}
