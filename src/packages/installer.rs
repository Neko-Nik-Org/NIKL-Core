pub fn install_package(package_name: &str) {
    // TODO: Implement package installation logic
    todo!("Implement package installation logic");
    // Check if we are in a correct directory and has ".nikl" directory and "info.json" file in it
    // If not, Ask user to run "nikl init" for creating a package repository or just ask to just create a virtual environment
    // Check if the package is already installed by checking in "info.json" file
    // If the package is already installed, print a message and return
    // If not installed, there are two options:
    // 1. Given package name is a local path (math-0.1.0.tar.gz file)
    // 2. Given package name is a remote package ("math" or "math@0.1.0")
    // If local path, install it
    // If remote package, download it from the server and install it (No login required)
}
