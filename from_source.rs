use std::path::PathBuf;
#[cfg(feature = "from-source")]
use std::env;
#[cfg(feature = "from-source")]
use crate::download::download_and_extract_zip;

#[cfg(feature = "from-source")]
pub fn is_from_source_feature_enabled() -> bool {
    true
}

#[cfg(not(feature = "from-source"))]
pub fn is_from_source_feature_enabled() -> bool {
    false
}


#[cfg(not(feature = "from-source"))]
pub fn download_scip_source() -> PathBuf {
    unimplemented!("Cannot download SCIP source code without the `from-source` feature")
}


#[cfg(feature = "from-source")]
pub fn download_scip_source() -> PathBuf {
    let url = "https://github.com/scipopt/scip-sys/releases/download/v0.1.9/scipoptsuite-9.0.0.zip";
    let target = env::var("OUT_DIR").unwrap();
    let target = std::path::Path::new(&target);
    if target.join("scipoptsuite-9.0.0").exists() {
        println!("cargo:warning=SCIP was previously downloaded, skipping download");
    } else {
        download_and_extract_zip(url, &*target).expect("Failed to download SCIP");
    }
    target.join("scipoptsuite-9.0.0")
}


#[cfg(feature = "from-source")]
pub fn compile_scip(source_path: PathBuf) -> PathBuf {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = std::path::Path::new(&out_dir);
    let lib_path = out_dir.join("lib");
    if lib_path.exists() {
        println!("cargo:warning=SCIP was previously compiled, skipping compilation");
        return out_dir.to_path_buf();
    }

    use cmake::Config;
    let mut dst = Config::new(source_path);

    dst.define("AUTOBUILD", "ON").build()
}

#[cfg(not(feature = "from-source"))]
pub fn compile_scip(_source_path: PathBuf) -> PathBuf {
    unimplemented!("Cannot compile SCIP without the `from-source` feature")
}