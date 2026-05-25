use std::env;
use std::process::{Command, exit};

const GRIM_CMD: &str = "/usr/bin/grim -t ppm -";
const SATTY_ARGS: &str = "--filename - --fullscreen --initial-tool crop";

fn build_command_string() -> String {
    let home = env::var("HOME").expect("Could not find HOME environment variable");
    
    format!(
        "{} | GSK_RENDERER=cairo {}/.cargo/bin/satty {}", 
        GRIM_CMD, 
        home, 
        SATTY_ARGS
    )
}

fn execute_command(script: &str) {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(script)
        .spawn()
        .expect("Failed to start process");

    let status = child.wait().expect("Failed to wait on process");

    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
}

fn main() {
    let script = build_command_string();
    execute_command(&script);
}