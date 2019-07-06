
use std::process::Command;
use std::str;
use std::{thread, time};

use colored::*;


fn main() {
    let node_id = 2;
    let color = "Green";


    let mut child = Command::new("/bin/bash")
                .arg("-c")
                .arg("cargo run --manifest-path ../application/Cargo.toml -- 2 hosts.txt Green")
                .spawn()
                .expect("failed to execute process");


    thread::sleep(time::Duration::from_millis(5000));
    child.kill();
}
