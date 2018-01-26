use std::process::Command;
use glob::glob;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub fn copy_files(from: &str, to: &str) {
    for entry in glob(from).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let file_os_string = path.into_os_string();
                let file = file_os_string.to_str().unwrap();
                run_command("cp", vec![file, to]);
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

pub fn run_command(cmd: &str, args: Vec<&str>) {
    let command = Command::new(cmd)
        .args(args)
        .output()
        .expect("failed to execute process");

    if command.status.success() {
        println!("{} succeeded", cmd);
    } else {
        let s = String::from_utf8_lossy(&command.stderr);
        println!("{} failed, stderr: {}", cmd, s);
    }
}

pub fn create_file(target: &str, contents: &[u8]) {
    let path = Path::new(target);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(contents) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => {
            println!("successfully wrote to {}", display);
        }
    }
}
