use std::env;
use std::fs;
use rustyline::{Editor, history::FileHistory};
use tokio;

use crate::lexer::lex;

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


#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Check if the file exists and is a valid script
        let filename = &args[1];
        if !check_file_is_valid(filename) {
            return;
        }

        // If there's a file to run
        let content = fs::read_to_string(filename).expect("Unable to read file");

        // Here you would normally parse and evaluate the content
        // For now, we just print it
        log::debug!("Running script: {}", filename);
        log::debug!("File content: {}", content);
        let tokens = lex(&content);
        log::debug!("Tokens: {:?}", tokens);

    } else {
        // Otherwise, REPL mode
        println!("Welcome to Nik-Lang REPL!");
        println!("To exit, type 'exit' or press Ctrl+D");

        // Initialize the Rustyline editor with FileHistory, unwrapping the Result
        let mut rl = Editor::<(), FileHistory>::new()
            .expect("Failed to initialize rustyline editor");

        // Optionally, you can load a history file, if desired
        if rl.load_history("~/.nik_history").is_ok() {
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
                    let tokens = lex(input);
                    log::debug!("Tokens: {:?}", tokens);
        
                }
                Err(_) => {
                    // Handle error, for example, if the user interrupts
                    break;
                }
            }
        }

        // Optionally, you can save the history to a file on exit
        if rl.save_history("history.txt").is_ok() {
            println!("Saved history to file.");
        }
    }
}
