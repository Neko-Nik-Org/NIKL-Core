use std::{
    env,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

use flate2::write::GzEncoder;
use flate2::Compression;
use serde::Deserialize;
use walkdir::WalkDir;
use tar::Builder;


#[derive(Deserialize)]
struct Config {
    name: String,
    version: String,
    #[serde(rename = "readmeFile")]
    #[serde(default)]
    readme_file: Option<String>,
    #[serde(rename = "licenseFile")]
    #[serde(default)]
    license_file: Option<String>,
}


pub fn create_tar_gz() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let config = read_and_validate_config(&current_dir)?;
    validate_required_files(&current_dir, &config)?;

    let tar_gz_name = format!("{}-{}.tar.gz", config.name, config.version);
    if Path::new(&tar_gz_name).exists() {
        panic!("File {} already exists. Please remove it before creating a new package.", tar_gz_name);
    }
    println!("Creating {}...", tar_gz_name);

    let tar_gz_file = File::create(&tar_gz_name)?;
    let encoder = GzEncoder::new(tar_gz_file, Compression::default());
    let mut archive = Builder::new(encoder);

    add_nk_files(&mut archive, &config.name)?;
    add_metadata_files(&mut archive, &config)?;
    println!("Created {} successfully.", tar_gz_name);
    Ok(())
}


fn read_and_validate_config(current_dir: &Path) -> io::Result<Config> {
    let config_path = current_dir.join("config.json");
    if !config_path.exists() {
        panic!("config.json not found");
    }

    let config_text = fs::read_to_string(&config_path)?;
    let config: Config = serde_json::from_str(&config_text)
        .expect("Invalid config.json structure or format");

    Ok(config)
}


fn validate_required_files(current_dir: &Path, config: &Config) -> io::Result<()> {
    let package_src_file = current_dir.join("src").join(format!("{}.nk", config.name));
    if !package_src_file.exists() {
        panic!("Required file src/{}.nk not found", config.name);
    }

    let nikl_info = current_dir.join(".nikl/info.json");
    if !nikl_info.exists() {
        panic!("Required file .nikl/info.json not found");
    }

    Ok(())
}


fn add_nk_files(archive: &mut Builder<GzEncoder<File>>, package_name: &str) -> io::Result<()> {
    for entry in WalkDir::new("src").into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("nk") && path.is_file() {
            let relative_path = path.strip_prefix("src").unwrap();
            let mut archive_path = PathBuf::from(package_name);
            archive_path.push(relative_path);
            archive.append_path_with_name(path, archive_path)?;
        }
    }
    Ok(())
}


fn add_metadata_files(
    archive: &mut Builder<GzEncoder<File>>,
    config: &Config,
) -> io::Result<()> {
    archive.append_path_with_name("config.json", "config.json")?;
    archive.append_path_with_name(".nikl/info.json", ".nikl/info.json")?;

    if let Some(readme) = &config.readme_file {
        if Path::new(readme).exists() {
            archive.append_path_with_name(readme, readme)?;
        }
    }

    if let Some(license) = &config.license_file {
        if Path::new(license).exists() {
            archive.append_path_with_name(license, license)?;
        }
    }

    // TODO: Return the readme and license files, to be used in the upload process
    Ok(())
}
