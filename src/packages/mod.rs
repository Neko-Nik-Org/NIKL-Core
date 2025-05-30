mod initialize;
mod installer;
mod builder;

pub use initialize::create_package_structure;
pub use installer::install_package;
pub use builder::create_tar_gz;

use serde::{Deserialize, Serialize};
use std::path::Path;


#[derive(Deserialize, Serialize, Debug)]
pub struct Author {
    pub name: String,
    pub email: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Dependency {
    pub name: String,
    pub version: String,
}


#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct PkgConfig {
    pub name: String,
    pub display_name: String,
    pub version: String,
    pub description: String,
    pub authors: Vec<Author>,
    pub license: String,
    pub readme_file: Option<String>,
    pub license_file: Option<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub dependencies: Vec<Dependency>,
    pub keywords: Vec<String>,
}


#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub is_local: bool,
    pub dependencies: Vec<String>,
}

impl Package {
    pub fn new(raw: String) -> Self {
        let trimmed = raw.trim().to_string();

        if Self::is_local_path(&trimmed) {
            let (name, version) = Self::parse_local(&trimmed)
                .expect("Failed to parse local package name, version, or dependencies");
            Self {
                name,
                version,
                is_local: true,
                dependencies: Vec::new(),
            }
        } else {
            let (name, version) = Self::parse_remote(&trimmed)
                .expect("Failed to parse remote package name and version");
            Self {
                name,
                version,
                is_local: false,
                dependencies: Vec::new(),
            }
        }
    }

    /// Determines if a package is local (file ends with `.tar.gz`)
    fn is_local_path(path: &str) -> bool {
        path.ends_with(".tar.gz")
    }

    fn is_local_file_available(path: &str) -> bool {
        Path::new(path).exists()
    }

    fn strip_tar_gz(name: &str) -> Option<&str> {
        name.strip_suffix(".tar.gz")
    }


    /// Parses remote package name and version from formats like `name@version` or just `name`
    fn parse_remote(s: &str) -> Result<(String, String), String> {
        let parts: Vec<&str> = s.split('@').collect();
        match parts.len() {
            2 => Ok((parts[0].to_string(), parts[1].to_string())),
            1 => Ok((parts[0].to_string(), "".to_string())),
            _ => Err("Invalid remote package format. Use 'name' or 'name@version'.".to_string()),
        }
    }

    fn parse_local_name_version(file_name: &str) -> Result<(String, String), String> {
        // Example file names: "math-1.0.0.tar.gz", "math-v1s-1.0.0.tar.gz
        let parts: Vec<&str> = file_name.split('-').collect();
        if parts.len() < 2 {
            return Err("Invalid local package file name format. Expected 'name-version.tar.gz'.".to_string());
        }
        let name = parts[..parts.len() - 1].join("-"); // Join all but the last part for the name
        let version_part = parts.last().ok_or("Failed to extract version part from file name")?;
        
        // Remove the file extension if it exists
        let version = Self::strip_tar_gz(version_part)
            .ok_or("Failed to strip file extension from version part")?
            .to_string();
        if name.is_empty() || version.is_empty() {
            return Err("Package name or version cannot be empty.".to_string());
        }

        Ok((name, version))
    }

    /// Parses local package name, version, and dependencies from a file path
    fn parse_local(path: &str) -> Result<(String, String), String> {
        Self::is_local_file_available(path)
            .then_some(())
            .ok_or_else(|| format!("Local package '{}' does not exist.", path))?;

        let file_name = Path::new(path)
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or("Failed to extract file name from path")?;
    
        let (name, version) = Self::parse_local_name_version(&file_name)
            .map_err(|e| format!("Failed to parse local package name and version: {}", e))?;

        if name.is_empty() || version.is_empty() {
            return Err("Package name or version cannot be empty.".to_string());
        }

        Ok((name, version))
    }



    fn is_already_installed(&self) -> bool {
        // Check if the package is already installed
        // This could involve checking a local database, file system, or other means
        // For now, we will just return false to indicate that the package is not installed
        false
    }

    pub fn install_package(&self) {
        // Check if the package is already installed
        if self.is_already_installed() {
            println!("Package '{}' is already installed. Skipping installation.", self.name);
            return;
        }

        if self.is_local {
            // If it's a local package, install it directly
            println!("Installing local package: {} (version: {})", self.name, self.version);
            // Here you would implement the logic to install the local package
            // For example, extracting the tar.gz file and copying files to the appropriate directories
        } else {
            // If it's not a local package, handle remote package installation
            println!("Installing remote package: {} (version: {})", self.name, self.version);
            // Here you would implement the logic to download and install the remote package (No login required)
            // This could involve fetching from a remote repository or server
        }
    }


    pub fn uninstall_package(&self) {
        // Logic to uninstall the package
        // This could involve removing files, directories, or entries from a database
        println!("Uninstalling package: {} (version: {})", self.name, self.version);
    }
}
