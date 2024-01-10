use std::fs;
use std::path::Path;

use crate::lexer::make_tokens;
use crate::expressions;

pub fn run_file(arg: &str) {
    let path = Path::new(arg);

    // If argument is a folder path
    if path.is_dir() {
        let main_file_path = path.join("main.nikl");

        // Check if "main.nikl" exists in the given folder path
        if main_file_path.exists() {
            // Read the content of "main.nikl"
            let file_data =
                fs::read_to_string(main_file_path).expect("Error: Could not read file");
            // run it
            println!("Running folder");
            run(&file_data);
        } else {
            // Print error if "main.nikl" is not found
            println!("Error: 'main.nikl' not found in the specified folder");
        }
    }
    // If argument is a file path
    else if path.is_file() && path.extension().unwrap() == "nikl" {
        // Read the content of the file
        let file_data =
            fs::read_to_string(path).expect("Error: Could not read file");
        // Run the file
        println!("Running file");
        run(&file_data);
    }
    // If argument is not a file or folder
    else {
        // Print error
        println!("Error: Argument given is not a file or folder nyaa~");
    }
}

// Run the file
fn run(file_data: &str) {
    let mut lexer = make_tokens::Lexer::new(file_data);
    let token = lexer.get_tokens();
    let mut parser = expressions::Parser::new(token.clone());
    let parsed_tokens = parser.parse();
    println!("Lexer tokens: {:?}", token);
    println!("Parser tokens: {:?}", parsed_tokens);
}
