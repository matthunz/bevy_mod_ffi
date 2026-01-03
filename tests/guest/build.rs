fn main() {
    // Get the target directory where the host shared library is built
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_dir = std::path::Path::new(&manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("target")
        .join(std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string()));

    // Tell the linker to search in the target directory for the host shared library
    println!("cargo:rustc-link-search=native={}", target_dir.display());

    // On Windows, link to the .dll.lib import library; on other platforms, link to the shared library directly
    let target = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target == "windows" {
        println!("cargo:rustc-link-lib=dylib=bevy_mod_ffi_host.dll");
    } else {
        println!("cargo:rustc-link-lib=dylib=bevy_mod_ffi_host");
    }
}
