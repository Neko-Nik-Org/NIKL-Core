use std::path::Path;
use super::initialize::create_nikl_environment;
use super::Package;



fn check_and_create_nikl_directory() {
    // See if ".nikl" directory exists and in that directory
    let nikl_dir = Path::new(".nikl");

    if !nikl_dir.exists() {
        println!("Creating .nikl directory for package management...");
        
        // Create the .nikl directory
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        create_nikl_environment(current_dir.as_path()).expect("Failed to create .nikl environment");
    } else {
        let packages_dir = nikl_dir.join("packages");
        if !packages_dir.exists() {
            std::fs::create_dir(&packages_dir).expect("Failed to create packages directory");
        }

        let info_file = nikl_dir.join("info.json");
        if !info_file.exists() {
            std::fs::File::create(&info_file).expect("Failed to create info.json file");
        } else {
            // TODO: Check if it has the correct structure
            // For now, we assume it has the correct structure
            // TODO: In future, we can add more checks to validate the structure
            // For now, we just print a message
            // This is a placeholder for future validation logic
        }
    }
}


pub fn install_package(full_package_name: &str) {
    // Check if virtual environment is valid
    check_and_create_nikl_directory();

    // Parse and install the package
    Package::new(full_package_name.to_string()).install_package();
}
