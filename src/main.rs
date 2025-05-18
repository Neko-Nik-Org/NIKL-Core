use std::env;
use nikl::cli;


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let cmd_or_file = &args[1];

        match cmd_or_file.as_str() {
            "help" => cli::print_help(),
            "init" => cli::init_package(&args[2..]),
            "build" => cli::build_package(),
            "login" => cli::login(),
            "logout" => cli::logout(),
            "publish" => cli::publish_package(),
            "install" => cli::install_package(&args[2..]),
            "uninstall" => cli::uninstall_package(&args[2..]),
            file if file.ends_with(".nk") => cli::run_file(file),
            other => eprintln!("Unknown command or invalid file: {}", other),
        }
    } else {
        if let Err(e) = cli::run_repl() {
            eprintln!("REPL exited with error: {}", e);
        }
    }
}
