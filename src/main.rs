mod interpreter;
mod lexer;

use lexer::make_tokens;

fn main() {

    main_2();
    return;





    // If there are arguments passed, run them
    // Currently only file path are supported or a project folder path
    // If folder path is passed, run "main.nikl"
    // If file path is passed, run it
    // If no file is found, print error

    // Get arguments
    let args: Vec<String> = std::env::args().collect();

    println!("{:?}", args);

    // If one argument is passed
    if args.len() == 2 {
        // Get argument
        let arg: &String = &args[1];
        // Interpret the file
        interpreter::run_file::run_file(arg);
        
    } else if args.len() > 2 {
        // TODO: If the args is of length 3, check if the second argument is "-o" and the third argument is a file path
        // If so, compile the project and output the binary to the file path
        // Print error
        println!("Error: Too many arguments");
    } else {
        // If no arguments are passed, start the REPL (Read-Eval-Print-Loop)
        interpreter::repl::start_repl();
    }
}



pub fn main_2() {
    let mut lexer = make_tokens::Lexer::new("cat = 19+ 2 * 3 - (4.2 ** 6);");
    
    loop {
        let token = lexer.get_token();
        println!("{:?}", token);
        if token.token_type == make_tokens::TokenType::Eof {
            break;
        }
    }
}