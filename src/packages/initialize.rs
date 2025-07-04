use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Ensures the directory exists, creating it if necessary.
fn create_dir_all(path: &Path) -> io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Writes content to the file, overwriting if it already exists.
fn create_file(path: &Path, content: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Generates a customized README template
fn generate_readme(project_name: &str) -> String {
    format!(
        r#"# {0}

This is a project created using the `nikl init` command. It serves as a template for creating new projects with the Nikl.

## Getting Started
To get started, you can modify the `src/{1}.nk` file to add your own code.
You can also add any additional files or directories as needed.
"#,
        capitalize_words(project_name),
        project_name
    )
}

/// Generates a config.json with interpolated project name
fn generate_config(project_name: &str) -> String {
    format!(
        r#"{{
    "name": "{0}",
    "displayName": "{1}",
    "version": "1.0.0",
    "description": "An example project to demonstrate the use of a configuration file.",
    "authors": [
        {{
            "name": "Neko Nik",
            "email": "admin@nekonik.com"
        }}
    ],
    "license": "MIT",
    "readmeFile": "README.md",
    "licenseFile": "LICENSE",
    "repository": "https://github.com/Neko-Nik-Org/NIKL-Core",
    "homepage": "https://nekonik.com",
    "dependencies": {{
        "os": "0.0.1",
        "regex": "1.0.0"
    }},
    "keywords": [
        "example",
        "project",
        "configuration"
    ]
}}"#,
        project_name,
        capitalize_words(project_name)
    )
}

/// Capitalizes words separated by dashes or underscores
fn capitalize_words(input: &str) -> String {
    input
        .split(|c| c == '-' || c == '_')
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Generates default content for the main source file
fn generate_main() -> &'static str {
    r#"// This is a simple example of a Nikl project.
print("Hello, Neko Nik!")
"#
}

/// Creates a virtual environment for Nikl projects
pub fn create_nikl_environment(dir: &Path) -> io::Result<()> {
    // Create a virtual environment directory
    let nikl_dir = dir.join(".nikl");
    let packages_dir = nikl_dir.join("packages");

    // Create the .nikl directory and packages subdirectory
    create_dir_all(&packages_dir)?;

    // Create an info.json file to store package information
    create_file(&nikl_dir.join("info.json"), r#"{ "packages": [] }"#)
}

/// Creates the standard Source Code Package Structure for a Nikl project
fn create_source_code_package_structure(dir: &Path, project_name: &str) -> io::Result<()> {
    // Make the src directory and main source file paths
    let src_dir = dir.join("src");
    let main_file_path = src_dir.join(format!("{}.nk", project_name));

    // Create a main source file
    create_dir_all(&src_dir)?;

    // Create main source file with default content
    create_file(&main_file_path, generate_main())
}

/// Create Additional files for the Nikl project
fn create_additional_files(dir: &Path, project_name: &str) -> io::Result<()> {
    // Create README.md
    let readme_content = generate_readme(project_name);
    create_file(&dir.join("README.md"), &readme_content)?;

    // Create config.json
    let config_content = generate_config(project_name);
    create_file(&dir.join("config.json"), &config_content)?;

    // Create LICENSE file
    create_file(&dir.join("LICENSE"), "")
}

/// Creates the standard project structure for a Nikl project
pub fn create_package_structure(dir: &Path, project_name: &str) -> io::Result<()> {
    // Validate project name (basic check)
    if project_name.trim().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Project name cannot be empty.",
        ));
    } else if project_name.contains(|c: char| !c.is_alphanumeric() && c != '-' && c != '_') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Project name can only contain alphanumeric characters, dashes, and underscores.",
        ));
    }

    // Create the .nikl environment
    create_nikl_environment(dir)?;
    create_source_code_package_structure(dir, project_name)?;
    create_additional_files(dir, project_name)?;

    Ok(())
}
