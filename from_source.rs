#[cfg(feature = "from-source")]
use crate::download::download_and_extract_zip;
#[cfg(feature = "from-source")]
use std::env;
use std::path::PathBuf;

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
    let scip_version = "9.2.4";
    let url = format!("https://github.com/scipopt/scip-sys/releases/download/v0.1.9/scipoptsuite-{scip_version}.zip");
    let target = env::var("OUT_DIR").unwrap();
    let target = std::path::Path::new(&target);
    if target.join(format!("scipoptsuite-{scip_version}")).exists() {
        println!("cargo:warning=SCIP was previously downloaded, skipping download");
    } else {
        download_and_extract_zip(&url, &*target).expect("Failed to download SCIP");
    }
    target.join(format!("scipoptsuite-{scip_version}"))
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

    let target = env::var("TARGET").unwrap();
    let is_emscripten = target.contains("emscripten");

    use cmake::Config;
    let mut dst = Config::new(source_path);

    if is_emscripten {
        let emsdk = env::var("EMSDK").expect("EMSDK env var must be set for emscripten builds");
        let toolchain = format!(
            "{}/upstream/emscripten/cmake/Modules/Platform/Emscripten.cmake",
            emsdk
        );
        dst.define("CMAKE_TOOLCHAIN_FILE", &toolchain);
        dst.define("CMAKE_CROSSCOMPILING", "TRUE");
        dst.define("TPI", "none");
        dst.define("CMAKE_CXX_FLAGS", "-fwasm-exceptions");
        dst.define("CMAKE_C_FLAGS", "-fwasm-exceptions");
    }

    if is_emscripten {
        dst.define("BUILD_TESTING", "OFF");
    }

    let dst = dst
        .define("IPOPT", "OFF")
        .define("ZIMPL", "OFF")
        .define("GMP", "OFF")
        .define("READLINE", "OFF")
        .define("BOOST", "OFF")
        .define("AUTOBUILD", "OFF")
        .define("PAPILO", "OFF")
        .define("SYM", "snauty")
        .define("ZLIB", "OFF")
        .define("SHARED", "OFF")
        .define("GCG", "OFF")
        .define("UG", "OFF")
        .define("SANITIZE_ADDRESS", "OFF")
        .define("SANITIZE_MEMORY", "OFF")
        .define("SANITIZE_UNDEFINED", "OFF")
        .define("SANITIZE_THREAD", "OFF")
        .build();

    dst
}

#[cfg(not(feature = "from-source"))]
pub fn compile_scip(_source_path: PathBuf) -> PathBuf {
    unimplemented!("Cannot compile SCIP without the `from-source` feature")
}
