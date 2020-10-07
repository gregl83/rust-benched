use std::process::{Command, Stdio};
use std::env;

fn main() {
    Command::new("cargo")
        .arg("+nightly")
        .arg("bench")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("cannot use cargo nightly build to run benchmarks");
}
