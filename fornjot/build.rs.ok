// Proposed by Claude for consistent timestamps in binaries
// thus allowing binaries of the same source to be identical.
// Proposed by Claude: https://claude.ai/share/7b90d2a7-db84-475b-8359-63e3bd4aaa5a
// See https://doc.rust-lang.org/cargo/reference/build-scripts.html
// for more info on build.rs
use std::process::Command;

fn main() {
    // Tell cargo to rerun if SOURCE_DATE_EPOCH changes.
    // See also: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-env-changed
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");

    let timestamp = match std::env::var("SOURCE_DATE_EPOCH") {
        Ok(val) => {
            println!("cargo:warning=Using provided SOURCE_DATE_EPOCH={val}");
            val
        }
        Err(_) => {
            println!("cargo:warning=SOURCE_DATE_EPOCH not set, getting from git");

            let output = Command::new("git")
                .args(["log", "-1", "--pretty=%ct"])
                .output()
                .expect("Failed to get git commit time");

            let ts = String::from_utf8(output.stdout)
                .expect("Invalid UTF-8")
                .trim()
                .to_string();

            println!("cargo:warning=Got timestamp from git: {ts}");
            ts
        }
    };

    // Tell cargo to use this timestamp
    println!("cargo:rustc-env=SOURCE_DATE_EPOCH={timestamp}");
}
