extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let package_name = env::var("CARGO_PKG_NAME").unwrap();

    let config = cbindgen::Config::from_file(config_path()).unwrap();
    let output_file = target_dir()
        .join("bindings")
        .join(format!("{}.h", package_name))
        .display()
        .to_string();

    cbindgen::generate_with_config(&crate_dir, config)
      .unwrap()
      .write_to_file(&output_file);
}

fn config_path() -> std::path::PathBuf {
    std::path::PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("cbindgen.toml")
}

fn target_dir() -> std::path::PathBuf {
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        std::path::PathBuf::from(target)
    } else {
        std::path::PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
    }
}