// src/utils.rs

// Include the necessary build info variables
include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

pub fn print_build_info() {
    println!("ArcNova Chain Build Info:");
    println!(" ├ Git SHA: {}", GIT_SHA);
    println!(" └ Built:   {}", BUILD_TIME);
}