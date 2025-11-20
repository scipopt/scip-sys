mod bundled;
mod from_source;

#[cfg(any(feature = "bundled", feature = "from-source"))]
mod download;

extern crate bindgen;

use glob::glob;
use std::env;
use std::error::Error;
use std::path::PathBuf;

use crate::from_source::{compile_scip, download_scip_source, is_from_source_feature_enabled};
use bundled::*;

extern crate pkg_config;

#[cfg(not(feature = "bundled"))]
pub fn is_bundled_feature_enabled() -> bool {
    false
}

fn _build_from_scip_dir(path: &str) -> bindgen::Builder {
    let lib_dir = PathBuf::from(&path).join("lib");
    let lib_dir_path = lib_dir.to_str().unwrap();

    if lib_dir.exists() {
        println!("cargo:warning=Using SCIP from {}", lib_dir_path);
        println!("cargo:rustc-link-search={}", lib_dir_path);
        println!("cargo:libdir={}", lib_dir_path);

        #[cfg(windows)]
        let lib_dir_path = PathBuf::from(&path).join("bin");
        #[cfg(windows)]
        println!("cargo:rustc-link-search={}", lib_dir_path.to_str().unwrap());
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
                return Some(_build_from_scip_dir(&scip_dir));
            } else {
                println!("cargo:warning=SCIP was not found in {}", scip_dir);
            }
        } else {
            println!("cargo:warning={} is not set", env_var_name);
        }
    }

    return None;
}

fn try_pkg_config() -> Option<bindgen::Builder> {
    println!("cargo:warning=Trying to find SCIP via pkg-config");

    match pkg_config::Config::new().probe("scip") {
        Ok(library) => {
            println!("cargo:warning=Found SCIP via pkg-config");

            // Get include paths from pkg-config
            let include_paths = library.include_paths;

            if include_paths.is_empty() {
                println!("cargo:warning=pkg-config found SCIP but no include paths provided");
                return None;
            }

            // Find scip.h and scipdefplugins.h in the include paths
            let mut scip_header = None;
            let mut scipdefplugins_header = None;

            for include_path in &include_paths {
                let scip_h = include_path.join("scip").join("scip.h");
                let scipdefplugins_h = include_path.join("scip").join("scipdefplugins.h");

                if scip_h.exists() && scip_header.is_none() {
                    scip_header = Some(scip_h);
                }
                if scipdefplugins_h.exists() && scipdefplugins_header.is_none() {
                    scipdefplugins_header = Some(scipdefplugins_h);
                }
            }

            match (scip_header, scipdefplugins_header) {
                (Some(scip_h), Some(scipdefplugins_h)) => {
                    println!("cargo:warning=Using SCIP headers from pkg-config");

                    let mut builder = bindgen::Builder::default()
                        .header(scip_h.to_str().unwrap())
                        .header(scipdefplugins_h.to_str().unwrap())
                        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

                    // Add all include paths as clang args
                    for include_path in include_paths {
                        builder = builder.clang_arg(format!("-I{}", include_path.display()));
                    }

                    Some(builder)
                }
                _ => {
                    println!("cargo:warning=pkg-config found SCIP but couldn't find scip.h or scipdefplugins.h");
                    None
                }
            }
        }
        Err(e) => {
            println!("cargo:warning=pkg-config failed to find SCIP: {}", e);
            None
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let builder = if is_bundled_feature_enabled() {
        download_scip();
        let path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("scip_install");
        _build_from_scip_dir(path.to_str().unwrap())
    } else if is_from_source_feature_enabled() {
        let source_path = download_scip_source();
        let build_path = compile_scip(source_path);
        _build_from_scip_dir(build_path.to_str().unwrap())
    } else {
        let builder = look_in_scipoptdir_and_conda_env();
        if builder.is_some() {
            builder.unwrap()
        } else {
            println!("cargo:warning=SCIP was not found in SCIPOPTDIR or in Conda environment");
            println!("cargo:warning=Looking for SCIP in system libraries");

            // Try pkg-config
            try_pkg_config().unwrap_or_else(|| {
                panic!(
                    "Could not find SCIP installation.\n\
                    Please either:\n\
                    - Set SCIPOPTDIR environment variable to point to your SCIP installation\n\
                    - Install SCIP with pkg-config support (scip.pc file)\n\
                    - Use --features bundled to download and use a bundled version\n\
                    - Use --features from-source to build SCIP from source"
                )
            })
        }
    };

    #[cfg(windows)]
    {
        println!("cargo:rustc-link-lib=libscip");
    }
    #[cfg(not(windows))]
    {
        println!("cargo:rustc-link-lib=scip");
    }

    #[cfg(feature = "from-source")]
    {
        let target = env::var("TARGET").unwrap();
        let apple = target.contains("apple");
        let linux = target.contains("linux");
        let mingw = target.contains("pc-windows-gnu");
        if apple {
            println!("cargo:rustc-link-lib=dylib=c++");
        } else if linux || mingw {
            println!("cargo:rustc-link-lib=dylib=stdc++");
        }

        #[cfg(windows)]
        {
            println!("cargo:rustc-link-lib=libsoplex");
        }
        #[cfg(not(windows))]
        {
            println!("cargo:rustc-link-lib=soplex");
        }
    }

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
