use std::process::{Command, Stdio};

fn main() {
    Command::new("cargo")
        .arg("bench")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("cannot use cargo nightly build to run benchmarks");
}
