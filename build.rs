use std::{os::unix::process::CommandExt, process::Command};

fn main() {
    Command::new("make")
        .arg("-C")
        .arg("./src/hw/")
        .exec();
}
