use std::process::Command;
use glob::glob;

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
