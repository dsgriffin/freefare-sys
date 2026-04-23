use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-env-changed=LIBFREEFARE_LIB_DIR");
    println!("cargo:rerun-if-env-changed=LIBFREEFARE_PATH");

    link_library(
        "freefare",
        &[
            env::var_os("LIBFREEFARE_LIB_DIR"),
            env::var_os("LIBFREEFARE_PATH"),
            if cfg!(target_os = "macos") {
                Some("/opt/homebrew/opt/libfreefare/lib".into())
            } else {
                None
            },
            if cfg!(target_os = "macos") {
                Some("/usr/local/opt/libfreefare/lib".into())
            } else {
                None
            },
            if cfg!(target_os = "linux") {
                Some("/usr/lib".into())
            } else {
                None
            },
            if cfg!(target_os = "linux") {
                Some("/usr/local/lib".into())
            } else {
                None
            },
        ],
    );
}

fn link_library(name: &str, candidates: &[Option<std::ffi::OsString>]) {
    let mut found_search_path = false;

    for candidate in candidates.iter().flatten() {
        let path = Path::new(candidate);
        if path.exists() {
            println!("cargo:rustc-link-search=native={}", path.display());
            found_search_path = true;
            break;
        }
    }

    if !found_search_path {
        println!(
            "cargo:warning=No explicit search path found for lib{name}; relying on the platform linker"
        );
    }

    println!("cargo:rustc-link-lib={name}");
}
