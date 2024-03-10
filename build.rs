extern crate bindgen;

use glob::glob;
use std::env;
use std::error::Error;
use std::path::PathBuf;

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
pub fn is_bundled_feature_enabled() -> bool {
    true
}

#[cfg(not(feature = "bundled"))]
pub fn is_bundled_feature_enabled() -> bool {
    false
}

fn _build_from_scip_dir(path: String) -> bindgen::Builder {
    let lib_dir = PathBuf::from(&path).join("lib");
    let lib_dir_path = lib_dir.to_str().unwrap();

    if lib_dir.exists() {
        println!("cargo:warning=Using SCIP from {}", lib_dir_path);
        println!("cargo:rustc-link-search={}", lib_dir_path)
    } else {
        panic!(
            "{}",
            format!(
                "{}/lib does not exist, please check your SCIP installation",
                path
            )
        );
    }

    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir_path);

    let include_dir = PathBuf::from(&path).join("include");
    let include_dir_path = include_dir.to_str().unwrap();
    let scip_header_file = PathBuf::from(&path)
        .join("include")
        .join("scip")
        .join("scip.h")
        .to_str()
        .unwrap()
        .to_owned();
    let scipdefplugins_header_file = PathBuf::from(&path)
        .join("include")
        .join("scip")
        .join("scipdefplugins.h")
        .to_str()
        .unwrap()
        .to_owned();

    bindgen::Builder::default()
        .header(scip_header_file)
        .header(scipdefplugins_header_file)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg(format!("-I{}", include_dir_path))
}

fn lib_scip_in_dir(path: &str) -> bool {
    glob(&format!("{}/lib/libscip*", path)).unwrap().count() > 0
}

fn look_in_scipoptdir_and_conda_env() -> Option<bindgen::Builder> {
    let env_vars = vec!["SCIPOPTDIR", "CONDA_PREFIX"];

    for env_var_name in env_vars {
        println!("cargo:rerun-if-env-changed={}", env_var_name);
        let env_var = env::var(env_var_name);
        if let Ok(scip_dir) = env_var {
            println!("cargo:warning=Looking for SCIP in {}", scip_dir);
            if lib_scip_in_dir(&scip_dir) {
                return Some(_build_from_scip_dir(scip_dir));
            } else {
                println!("cargo:warning=SCIP was not found in {}", scip_dir);
            }
        } else {
            println!("cargo:warning={} is not set", env_var_name);
        }
    }

    return None
}
fn main() -> Result<(), Box<dyn Error>> {
    let builder =
    if is_bundled_feature_enabled() {
        download_scip();
        _build_from_scip_dir(format!("{}/scip_install", env::var("OUT_DIR").unwrap()))
    } else {
        let builder = look_in_scipoptdir_and_conda_env();
        if builder.is_some() {

            builder.unwrap()
        } else {
            println!("cargo:warning=SCIP was not found in SCIPOPTDIR or in Conda environemnt");
            println!("cargo:warning=Looking for SCIP in system libraries");

            let headers_dir_path = "headers/";
            let headers_dir = PathBuf::from(headers_dir_path);
            let scip_header_file = PathBuf::from(&headers_dir)
                .join("scip")
                .join("scip.h")
                .to_str()
                .unwrap()
                .to_owned();
            let scipdefplugins_header_file = PathBuf::from(&headers_dir)
                .join("scip")
                .join("scipdefplugins.h")
                .to_str()
                .unwrap()
                .to_owned();

            bindgen::Builder::default()
                .header(scip_header_file)
                .header(scipdefplugins_header_file)
                .parse_callbacks(Box::new(bindgen::CargoCallbacks))
                .clang_arg(format!("-I{}", headers_dir_path))
        }
    };

    #[cfg(windows)]
    println!("cargo:rustc-link-lib=libscip");
    #[cfg(not(windows))]
    println!("cargo:rustc-link-lib=scip");

    let builder = builder
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    let bindings = builder.generate()?;
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}

#[cfg(feature = "bundled")]
fn download_scip() {
    let extract_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    if extract_path.join("scip_install").exists() {
        println!("cargo:warning=SCIP was previously downloaded, skipping download");
        return;
    }

    let info = os_info::get();
    let os = info.os_type();
    let arch = std::env::consts::ARCH;
    println!("cargo:warning=Detected OS: {}", os);
    println!("cargo:warning=Detected arch: {}", arch);

    let os_string = if os == os_info::Type::Ubuntu && arch == "x86_64" {
        "Linux-x86_64"
    } else if os == os_info::Type::Macos && arch == "x86_64" {
        "Darwin-x86_64"
    } else if os == os_info::Type::Macos && arch == "aarch64" {
        "Darwin-arm"
    } else if info.os_type() == os_info::Type::Windows && arch == "x86_64" {
        "win64-VS22"
    } else {
        panic!("Unsupported OS-arch combination: {}-{}", os, arch);
    };

    let url = format!(
        "https://scip.zib.de/download/release/SCIP-9.0.0-{os_string}.zip"
    );

    download_and_extract_zip(&url, &extract_path).unwrap_or_else(
        |e| panic!("Failed to download and extract SCIP: {}", e),
    );
}


#[cfg(not(feature = "bundled"))]
fn download_scip() {}



#[cfg(feature = "bundled")]
fn download_and_extract_zip(url: &str, extract_path: &Path) -> Result<(), Box<dyn Error>> {
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
