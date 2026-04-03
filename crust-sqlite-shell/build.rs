use std::env;
use std::path::PathBuf;

fn main() {
    // Get the manifest directory
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().unwrap();

    // Path to the compiled sqlite_noamalgam library
    let lib_path = workspace_root.join("sqlite-shell/release");

    // Link to the sqlite_noamalgam library
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=sqlite_noamalgam");

    // Enable rpath so the binary finds the library at runtime
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path.display());

    // Link system libraries that sqlite needs
    println!("cargo:rustc-link-lib=m");      // libm (math)
    println!("cargo:rustc-link-lib=pthread"); // libpthread
    println!("cargo:rustc-link-lib=dl");      // libdl (dynamic loader)
    println!("cargo:rustc-link-lib=z");       // libz (compression)
}
