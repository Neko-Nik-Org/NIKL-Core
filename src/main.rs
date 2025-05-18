use std::env;
use nikl::cli;


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let cmd_or_file = &args[1];

        match cmd_or_file.as_str() {
            "help" => {
                println!("Usage:");
                println!("  nikl            # Start REPL");
                println!("  nikl <file.nk>  # Run script file");
            }
            file if file.ends_with(".nk") => {
                cli::run_file(file);
            }
            other => {
                eprintln!("Unknown command or invalid file: {}", other);
            }
        }
    } else {
        if let Err(e) = cli::run_repl() {
            eprintln!("REPL exited with error: {}", e);
        }
    }
}
