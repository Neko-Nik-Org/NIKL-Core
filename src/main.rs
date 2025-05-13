use std::env;
use std::fs;
use rustyline::{Editor, history::FileHistory};
use tokio;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // If there's a file to run
        let filename = &args[1];
        let content = fs::read_to_string(filename).expect("Unable to read file");

        // Here you would normally parse and evaluate the content
        // For now, we just print it
        println!("Running file: {}", filename);
        println!("Content:\n{}", content);
    } else {
        // Otherwise, REPL
        println!("NekoNik REPL - Type 'exit' to quit");

        // Initialize the Rustyline editor with FileHistory, unwrapping the Result
        let mut rl = Editor::<(), FileHistory>::new()
            .expect("Failed to initialize rustyline editor");

        // Optionally, you can load a history file, if desired
        if rl.load_history("history.txt").is_ok() {
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
