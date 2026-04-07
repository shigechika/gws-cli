// Fork-only: embed git describe output for fork version identification.
// This file does not exist in upstream and will not cause merge conflicts.

use std::process::Command;

fn main() {
    // Re-run if git HEAD changes (new commit or checkout)
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    println!("cargo:rerun-if-changed=../../.git/refs/");

    let describe = Command::new("git")
        .args(["describe", "--tags", "--always", "--dirty"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=GWS_FORK_DESCRIBE={describe}");
}
