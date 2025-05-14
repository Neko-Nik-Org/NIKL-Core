use std::env;
use std::fs;
use rustyline::{Editor, history::FileHistory};
use tokio;

use crate::lexer::{Lexer, LexError};

mod lexer;


fn check_file_is_valid(filename: &str) -> bool {
    // Check if the file exists and is a regular file
    // Also check if the file is not empty and ends with .nk
    if let Ok(metadata) = fs::metadata(filename) {
        if metadata.is_file() && filename.ends_with(".nk") {
            let file_size = metadata.len();
            if file_size > 0 {
                return true;
            } else {
                log::error!("Error: Script '{}' is empty.", filename);
                return false;
            }
        } else {
            log::error!("Error: File '{}' is not a valid script, it should end with .nk", filename);
            return false;
        }
    } else {
        log::error!("Error: File '{}' does not exist.", filename);
        return false;
    }
}


fn read_file(filename: &str) -> String {
    // Validate the file name
    if !check_file_is_valid(filename) {
        return String::new();
    }

    // Read the file content and return it as a String
    match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            log::error!("Error reading file '{}': {}", filename, e);
            String::new()
        }
    }
}


#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // TODO: Add support for package management and other commands
        // nikl <None> means REPL
        // nikl help
        // nikl run <script_name> or nikl <script_name> (Decide)
        // nikl install <pkg_name>
        // nikl init . Or dir have a toml file
        // nikl login, logout, publish, ???

        // If a file is provided, read and tokenize it
        let filename = &args[1];
        let content = read_file(filename);
        if content.is_empty() {
            log::error!("Error: No valid content to process.");
            return;
        }

        let lexer = Lexer::new(content.as_str());
        match lexer.tokenize() {
            Ok(tokens) => {
                for token in tokens {
                    println!("{:?}", token);
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
        // Otherwise, REPL mode
        println!("Welcome to Nikl REPL!");
        println!("To exit, type 'exit' or press Ctrl+D");

        // Initialize the Rustyline editor with FileHistory, unwrapping the Result
        let mut rl = Editor::<(), FileHistory>::new()
            .expect("Failed to initialize rustyline editor");

        // Optionally, you can load a history file, if desired
        if rl.load_history("~/.nikl_history").is_ok() {
            println!("Loaded history from file.");
        }

        loop {
            // Prompt user for input
            let input = rl.readline(">>> ");

            match input {
                Ok(input) => {
                    let input = input.trim();

                    // Save to history
                    rl.add_history_entry(input.to_string())
                        .expect("Failed to add entry to history");

                    if input == "exit" {
                        break;
                    }

                    // Here you would normally parse and evaluate the input
                    // For now, we just echo it back
                    println!("You entered: {}", input);
                    
                    // Tokenize the input
                    let lexer = Lexer::new(input);
                    match lexer.tokenize() {
                        Ok(tokens) => {
                            for token in tokens {
                                println!("{:?}", token);
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
                Err(_) => {
                    // Handle error, for example, if the user interrupts
                    break;
                }
            }
        }

        // Optionally, you can save the history to a file on exit
        if rl.save_history("~/.nikl_history").is_ok() {
            println!("Saved history to file.");
        }
    }
}
