#[cfg(feature = "bundled")]
use crate::download::download_and_extract_zip;
#[cfg(feature = "bundled")]
use std::env;
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

    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    println!("cargo:warning=Detected OS: {}", os);
    println!("cargo:warning=Detected arch: {}", arch);

    let os_string = if os == "linux" && arch == "x86_64" {
        "linux"
    } else if os == "linux" && arch == "aarch64" {
        "linux-arm"
    }
    else if os == "macos" && arch == "x86_64" {
        "macos-intel"
    } else if os == "macos" && arch == "aarch64" {
        "macos-arm"
    } else if os == "windows" && arch == "x86_64" {
        "windows"
    } else {
        panic!("Unsupported OS-arch combination: {}-{}", os, arch);
    };

    // if debug mode is enabled, download the debug version of SCIP
    #[cfg(debug_assertions)]
    let debug_str = "-debug";
    #[cfg(not(debug_assertions))]
    let debug_str = "";

    let url = format!(
        "https://github.com/scipopt/scipoptsuite-deploy/releases/download/v0.8.0/libscip-{os_string}{debug_str}.zip",
    );

    download_and_extract_zip(&url, &extract_path)
        .unwrap_or_else(|e| panic!("Failed to download and extract SCIP: {}", e));
}

#[cfg(not(feature = "bundled"))]
pub fn download_scip() {}
