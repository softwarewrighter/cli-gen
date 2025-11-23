use std::env;
use std::fs;
use std::path::Path;

// This build script generates version information for the CLI
// It uses environment variables and git information to create version strings

fn main() {
    // Set version information as environment variables for the main application
    println!("cargo:rustc-env=CLI_VERSION={{ version }}");
    println!("cargo:rustc-env=CLI_NAME={{ name }}");
    println!("cargo:rustc-env=CLI_GIT_HASH={{ git_hash }}");
    
    // Generate version module
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("version.rs");
    let version_info = format!(
        r#"pub const VERSION: &str = "{}";
           pub const NAME: &str = "{}";
           pub const GIT_HASH: &str = "{}";"#,
        env!("CARGO_PKG_VERSION"),
        "{{ name }}",
        get_git_hash()
    );
    
    fs::write(&dest_path, version_info).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}

fn get_git_hash() -> String {
    if let Ok(output) = std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
    {
        if output.status.success() {
            return String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
    }
    "unknown".to_string()
}