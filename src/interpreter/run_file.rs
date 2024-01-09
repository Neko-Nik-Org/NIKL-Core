
pub fn run_file(arg: &str) {
    // If argument is a folder path
    if std::path::Path::new(arg).is_dir() {
        // Run "main.nikl"
        println!("Running main.nikl");
    }
    // If argument is a file path
    else if std::path::Path::new(arg).is_file() {
        // Run file
        println!("Running file");
    }
    // If argument is not a file or folder
    else {
        // Print error
        println!("Error: Argument is not a file or folder");
    }
}