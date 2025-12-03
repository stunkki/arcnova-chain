use std::process::Command;
use std::env;
use std::fs;

fn main() {
    // Get Git SHA
    let git_sha = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".into());

    // Build timestamp
    let build_time = chrono::Utc::now().to_rfc3339();

    // Create output directory for env vars
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = format!("{}/build_info.rs", out_dir);

    // Generate Rust code containing constants
    let contents = format!(
        r#"
        pub const GIT_SHA: &str = "{git_sha}";
        pub const BUILD_TIME: &str = "{build_time}";
    "#
    );

    fs::write(dest_path, contents).unwrap();

    // Re-run if Git HEAD changed
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/heads/");
}