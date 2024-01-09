
use std::io::Write;

pub fn start_repl() {
    println!("Welcome to the NikL interpreter!");
    loop {
        // Print prompt
        print!("> ");
        // Flush stdout
        std::io::stdout().flush().unwrap();
        // Read line
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input_cleaned = input.trim().to_string();
        if input_cleaned == "exit()" || input_cleaned == "exit();" || input == "" {
            println!("Bye!");
            return ();
        } else if input_cleaned == "help()" || input_cleaned == "help();" {
            println!("NikL Interpreter Help");
            println!("exit() - Exit the interpreter");
            println!("help() - Print this help message");
            continue;
        } else {
            // This is where we will interpret the input
            // We will use the lexer to get tokens
            // We will use the parser to parse the tokens
            // We will use the interpreter to interpret the parsed tokens
            // We will use the compiler to compile the parsed tokens
            // We will use the VM to run the compiled tokens
        }

        // Print input
        println!("You typed: '{}'", input);
    }
}