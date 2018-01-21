extern crate clap;
extern crate glob;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use clap::{App, Arg, SubCommand};
use glob::glob;

static DNSMASQ_DEFAULT_CONFIG: &'static str = 
"address=/.test/127.0.0.1";

fn copy_files(from: &str, to: &str) {
    let mut from_paths: Vec<&str> = Vec::new();
    for entry in glob(from).expect("Failed to read glob pattern") {
        match entry {
            // how to push the mathched path to the from_paths array
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }
}

fn run_command(cmd: &str, args: Vec<&str>) {
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

fn add_dnsmasq_to_launchctl() {
    // copy_files("/usr/local/opt/dnsmasq/*.plist", "to");
    // why is `cp` not working?
    // run_command("cp", vec!["/usr/local/opt/dnsmasq/*.plist", "/Library/LaunchDaemons"]);
    // run_command("launchctl", vec!["load", "/Library/LaunchDaemons/homebrew.mxcl.dnsmasq.plist"]);
}

fn main() {
    let app = App::new("TacoDev")
        .version("0.1.0")
        .author("Fraser Xu <xvfeng123@gmail.com>")
        .about("Taco Dev")
        .subcommand(SubCommand::with_name("dnsmasq")
            .about("Set up dnsmasq")
            .arg(Arg::with_name("domain")
                .short("d")
                .long("domain")
                .value_name("DOMAIN")
                .help("Local domain to setup."))
        )
        .subcommand(SubCommand::with_name("nginx")
            .about("Set up NGINX")
            .arg(Arg::with_name("upstream")
                .short("u")
                .long("upstream")
                .value_name("UPSTREAM")
                .help("Nginx upstream server, can be either localhost or a local socket file."))
        );

    let matches = app.get_matches();
    if let Some(matches) = matches.subcommand_matches("dnsmasq") {
        println!("Setting up dnsmasq");
        let domain = matches.value_of("domain").unwrap_or("test");
        println!("Domain: {}", domain);

        let path = Path::new("/usr/local/etc/dnsmasq.conf2");
        let display = path.display();
        
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
        };


        match file.write_all(DNSMASQ_DEFAULT_CONFIG.as_bytes()) {
            Err(why) => {
                panic!("couldn't write to {}: {}", display, why.description())
            },
            Ok(_) => {
                println!("successfully wrote to {}", display);
                add_dnsmasq_to_launchctl();
            }
        }
        println!("Writing file to: {}", display);
    }

    if let Some(matches) = matches.subcommand_matches("nginx") {
        println!("Setting up NGINX");
        let upstream_server = matches.value_of("upstream").unwrap_or("127.0.0.1:8000");
        println!("Upstream server: {}", upstream_server);
    }
} 