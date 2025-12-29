fn main() {
    // Get the target directory where the host DLL import library is built
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_dir = std::path::Path::new(&manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("target")
        .join(std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string()));

    // Tell the linker to search in the target directory for the host DLL import library
    println!("cargo:rustc-link-search=native={}", target_dir.display());
    println!("cargo:rustc-link-lib=dylib=bevy_mod_ffi_host.dll");
}
