mod repl;
mod run_file;

pub use repl::run_repl;
pub use run_file::run_file;


pub fn print_help() {
    println!("Usage:");
    println!("  nikl            # Start REPL");
    println!("  nikl <file.nk>  # Run script file");
    println!("  nikl init <dir> # Initialize a new package");
    println!("  nikl build      # Build the current package");
    println!("  nikl login      # Login to your account");
    println!("  nikl logout     # Logout from the current user");
    println!("  nikl publish    # Publish the current package");
    println!("  nikl install <pkg>    # Install a package");
    println!("  nikl uninstall <pkg>  # Uninstall a package");
    println!("  nikl help       # Show this help message");
}


pub fn init_package(args: &[String]) {
    if args.len() != 1 {
        eprintln!("Usage: nikl init <dir>");
        return;
    }
    let dir = &args[0];
    println!("Initializing package in directory: {}", dir);

    // Validate directory
    let dir = std::path::Path::new(dir);
    if !dir.exists() {
        println!("Directory does not exist. Creating it...");
        if let Err(e) = std::fs::create_dir_all(dir) {
            eprintln!("Failed to create directory: {}", e);
            return;
        }
    }

    // Check if the directory is empty
    if dir.read_dir().map_or(false, |mut entries| entries.next().is_some()) {
        eprintln!("Directory is not empty. Please choose an empty directory.");
        return;
    }

    // Get project name from the directory name
    let project_name = dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("nikl_project");

    // Create the package structure
    println!("Creating package structure...");
    match crate::packages::create_package_structure(dir, project_name) {
        Ok(_) => println!("Package structure created successfully."),
        Err(e) => eprintln!("Failed to create package structure: {}", e),
    }
}


pub fn build_package() {
    println!("Building the current package...");
    todo!("Implement package building logic");
}

pub fn login() {
    println!("Logging in...");
    todo!("Implement login logic");
}

pub fn logout() {
    println!("Logging out...");
    todo!("Implement logout logic");
}

pub fn publish_package() {
    println!("Publishing the current package...");
    todo!("Implement package publishing logic");
}

pub fn install_package(args: &[String]) {
    if args.len() != 1 {
        eprintln!("Usage: nikl install <pkg>");
        return;
    }
    let pkg = &args[0];
    println!("Installing package: {}", pkg);
    todo!("Implement package installation logic");
}

pub fn uninstall_package(args: &[String]) {
    if args.len() != 1 {
        eprintln!("Usage: nikl uninstall <pkg>");
        return;
    }
    let pkg = &args[0];
    println!("Uninstalling package: {}", pkg);
    todo!("Implement package uninstallation logic");
}
