use std::process::{Command, Stdio};

const SCRIPT: &str = include_str!("../print-satty.sh");

fn main() {
    let mut child = Command::new("bash")
        .arg("-c")
        .arg(SCRIPT)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start bash process");

    let status = child.wait().expect("Failed to wait on bash process");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}
