extern crate bindgen;

use flate2::read::GzDecoder;
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::process::Stdio;
use tar::Archive;

use glob::glob;

// Download libscip files from GitHub releases
fn download_lib_files() -> Result<String, Box<dyn Error>> {
    let info = os_info::get();
    println!("cargo:warning=Detected OS: {}", info.os_type());

    let os = match info.os_type() {
        os_info::Type::Ubuntu => "linux-gnu-cxx11",
        os_info::Type::Macos => "apple-darwin",
        os_info::Type::Windows => "w64-mingw32-cxx11",
        os => panic!("Unsupported OS: {}", os),
    };

    let arch = std::env::consts::ARCH;
    println!("cargo:warning=Detected arch: {}", arch);

    let url_base = "https://github.com/JuliaBinaryWrappers/SCIP_jll.jl/releases/download/SCIP-v800.0.301%2B0/SCIP.v800.0.301.";
    let url = format!("{}{}-{}.tar.gz", url_base, arch, os);

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = PathBuf::from(out_dir);
    let out_dir = out_dir.join("lib");
    let out_dir = out_dir.to_str().unwrap();

    // Download libscip files using curl
    println!("cargo:warning=Downloading libscip files from {}", url);
    let output = std::process::Command::new("curl")
        .args(&["-L", &url, "-o", "libscip.tar.gz"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .map_err(|e| format!("Failed to download libscip files: {}", e))?;

    if !output.status.success() {
        panic!("Failed to download libscip files");
    }

    // Extract libscip files
    println!("cargo:warning=Extracting libscip files to {}", out_dir);
    let tar_gz = File::open("libscip.tar.gz")?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(out_dir)?;

    println!("cargo:warning=libscip files extracted to {}", out_dir);
    println!("cargo:warning=cleaning up libscip.tar.gz");
    std::fs::remove_file("libscip.tar.gz")?;

    Ok(out_dir.to_owned())
}

fn _build_from_scip_dir(path: String) -> bindgen::Builder {
    let lib_dir = PathBuf::from(&path).join("lib");
    let lib_dir_path = lib_dir.to_str().unwrap();

    if lib_dir.exists() {
        println!("cargo:warning=Using SCIP from {}", lib_dir_path);
        println!("cargo:rustc-link-search={}", lib_dir_path);
    } else {
        panic!(
            "{}",
            format!(
                "{}/lib does not exist, please check your SCIP installation",
                path
            )
        );
    }
    println!("cargo:rustc-link-search={}/lib", lib_dir_path);
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

fn main() -> Result<(), Box<dyn Error>> {
    let env_vars = vec!["SCIPOPTDIR", "CONDA_PREFIX"];
    let mut builder = bindgen::Builder::default();
    let mut found_scip = false;
    for env_var_name in env_vars {
        println!("cargo:rerun-if-env-changed={}", env_var_name);
        let env_var = env::var(env_var_name);
        if let Ok(scip_dir) = env_var {
            println!("cargo:warning=Looking for SCIP in {}", scip_dir);
            if lib_scip_in_dir(&scip_dir) {
                builder = _build_from_scip_dir(scip_dir);
                found_scip = true;
                break;
            } else {
                println!("cargo:warning=SCIP was not found in {}", scip_dir);
            }
        } else {
            println!("cargo:warning={} is not set", env_var_name);
        }
    }

    if !found_scip {
        println!("cargo:warning=SCIP was not found in SCIPOPTDIR or in Conda environemnt, downloading SCIP from GitHub releases");
        let lib_path = download_lib_files().unwrap();
        builder = _build_from_scip_dir(lib_path);
    }

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
