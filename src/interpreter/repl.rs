
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
        }

        // Print input
        println!("You typed: '{}'", input);
    }
}