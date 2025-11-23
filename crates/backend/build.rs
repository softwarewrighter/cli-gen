use std::process::Command;

fn main() {
    // Get commit SHA from git
    let commit_sha = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Get current timestamp in ISO 8601 format
    let build_time = chrono::Utc::now().to_rfc3339();

    // Get hostname
    let build_host = hostname::get()
        .ok()
        .and_then(|h| h.into_string().ok())
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=BUILD_COMMIT_SHA={}", commit_sha);
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);
    println!("cargo:rustc-env=BUILD_HOST={}", build_host);

    // Re-run if git HEAD changes
    println!("cargo:rerun-if-changed=../../.git/HEAD");
}
