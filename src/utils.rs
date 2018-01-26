use std::process::Command;
use glob::glob;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub fn copy_file(from: &str, to: &str) {
    run_command("cp", vec![from, to]);
}

pub fn copy_files(from: &str, to: &str) {
    for entry in glob(from).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let file_os_string = path.into_os_string();
                let file = file_os_string.to_str().unwrap();
                copy_file(file, to);
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

pub fn ln_files(from: &str, to: &str) {
    for entry in glob(from).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let file_os_string = path.into_os_string();
                let file = file_os_string.to_str().unwrap();
                run_command("ln", vec!["-sfv", file, to]);
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

pub fn run_command(cmd: &str, args: Vec<&str>) {
    let command = Command::new(cmd)
        .args(&args)
        .output()
        .expect("failed to execute process");

    if command.status.success() {
        println!("Finished: {} {:?}", cmd, args);
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

pub fn create_nginx_config(upstream_server: &str, server_name: &str, root: &str) -> String {
    format!(
        "upstream {server_name} {{
    server {upstream_server};
}}

server {{
    listen 80;
    server_name {server_name};
    root {root};

    try_files $uri/index.html $uri @{server_name};

    location @{server_name} {{
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header Host $http_host;
        proxy_redirect off;

        proxy_pass http://{server_name};
    }}
}}",
        upstream_server = upstream_server,
        server_name = server_name,
        root = root
    )
}
