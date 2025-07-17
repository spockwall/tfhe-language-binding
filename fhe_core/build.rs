use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // Determine the correct feature for tfhe based on os architecture
    let target = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
    let mut tfhe_features = Vec::new();

    match target.as_str() {
        "x86_64" => {
            if os_family == "unix" {
                tfhe_features.push("x86_64-unix");
            } else if os_family == "windows" {
                tfhe_features.push("x86_64");
            }
        }
        "aarch64" => {
            if os_family == "unix" {
                tfhe_features.push("aarch64-unix");
            }
        }
        _ => {
            panic!("Unsupported os architecture: {}, build failed!", target);
        }
    }

    // Create a file to store the version information
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("tfhe_version.rs");

    // Run `cargo metadata` and capture the output
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version=1")
        .output()
        .expect("Failed to execute cargo metadata");

    // Find the version of the tfhe crate in cargo.lock
    let metadata = String::from_utf8(output.stdout).expect("cargo metadata output not valid UTF-8");
    let mut tfhe_version = "unknown";
    for package in metadata.split("{").skip(1) {
        if package.contains("\"name\":\"tfhe\"") {
            for line in package.split(",") {
                if line.contains("\"version\":\"") {
                    tfhe_version = line.split("\"").nth(3).unwrap();
                    break;
                }
            }
        }
    }

    // Write the version information to a file
    fs::write(
        dest_path,
        format!("pub const TFHE_VERSION: &str = \"{}\";\n", tfhe_version),
    )
    .expect("Unable to write tfhe_version.rs");
}
