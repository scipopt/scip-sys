use std::error::Error;
use tempfile::tempdir;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use zip_extract::extract;


pub fn download_and_extract_zip(url: &str, extract_path: &Path) -> Result<(), Box<dyn Error>> {
    // Download the ZIP file
    println!("cargo:warning=Downloading from {}", url);
    let resp = ureq::get(url).timeout(Duration::from_secs(300)).call()?;
    let mut content: Vec<u8> = Vec::new();
    resp.into_reader().read_to_end(&mut content)?;

    // Create a temporary file to store the ZIP
    let dir = tempdir()?;
    let zip_path = dir.path().join("scip.zip");



    let mut temp_file = File::create(&zip_path)?;
    temp_file.write_all(&content)?;
    let target_dir = PathBuf::from(extract_path);

    println!("cargo:warning=Downloaded to {:?}", zip_path);
    println!("cargo:warning=Extracting to {:?}", target_dir);
    for entry in std::fs::read_dir(&target_dir)? {
        println!("cargo:warning=Extracted file: {:?}", entry?.path());
    }
    extract(Cursor::new(
        std::fs::read(zip_path).unwrap(),
    ), &target_dir, false)?;

    Ok(())
}
