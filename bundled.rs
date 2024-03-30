use std::env;
use std::error::Error;
#[cfg(feature = "bundled")]
use tempfile::tempdir;
#[cfg(feature = "bundled")]
use std::fs::File;
#[cfg(feature = "bundled")]
use std::io::Cursor;
#[cfg(feature = "bundled")]
use std::io::Write;
#[cfg(feature = "bundled")]
use std::path::Path;
#[cfg(feature = "bundled")]
use std::path::PathBuf;


#[cfg(feature = "bundled")]
pub fn is_bundled_feature_enabled() -> bool {
    true
}


#[cfg(feature = "bundled")]
pub fn download_scip() {
    let extract_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    if extract_path.join("scip_install").exists() {
        println!("cargo:warning=SCIP was previously downloaded, skipping download");
        return;
    }

    let os = env::consts::OS;
    let arch = std::env::consts::ARCH;
    println!("cargo:warning=Detected OS: {}", os);
    println!("cargo:warning=Detected arch: {}", arch);

    let os_string = if os == "linux" && arch == "x86_64" {
        "linux"
    } else if os == "macos" && arch == "x86_64" {
        "macos"
    } else if os == "macos" && arch == "aarch64" {
        "macos-arm"
    } else if os == "windows" && arch == "x86_64" {
        "windows"
    } else {
        panic!("Unsupported OS-arch combination: {}-{}", os, arch);
    };

    let url = format!(
        "https://github.com/scipopt/scip-sys/releases/download/v0.1.9/libscip-{os_string}.zip"
    );

    download_and_extract_zip(&url, &extract_path).unwrap_or_else(
        |e| panic!("Failed to download and extract SCIP: {}", e),
    );
}


#[cfg(not(feature = "bundled"))]
pub fn download_scip() {}


#[cfg(feature = "bundled")]
pub fn download_and_extract_zip(url: &str, extract_path: &Path) -> Result<(), Box<dyn Error>> {
    // Download the ZIP file
    println!("cargo:warning=Downloading from {}", url);
    let response = reqwest::blocking::Client::new().get(url).send()?;
    let content = response.bytes()?;

    // Create a temporary file to store the ZIP
    let dir = tempdir()?;
    let zip_path = dir.path().join("scip.zip");
    let mut temp_file = File::create(&zip_path)?;
    temp_file.write_all(&content)?;
    let target_dir = PathBuf::from(extract_path);

    println!("cargo:warning=Downloaded to {:?}", zip_path);
    println!("cargo:warning=Extracting to {:?}", target_dir);
    zip_extract::extract(Cursor::new(
        std::fs::read(zip_path).unwrap(),
    ), &target_dir, false)?;

    Ok(())
}
