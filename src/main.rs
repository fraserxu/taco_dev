extern crate clap;
extern crate taco_dev;

use std::fmt;
use clap::{App, Arg, SubCommand};

use taco_dev::utils::{copy_files, create_file, run_command};

fn setup_dnsmasq(domain: &str) {
    let dns_conf = fmt::format(format_args!("address=/.{}/127.0.0.1", domain));
    create_file("/usr/local/etc/dnsmasq.conf3", dns_conf.as_bytes());
    // Need to figure out how to `sudo` properly
    copy_files("/usr/local/opt/dnsmasq/*.plist", "/Library/LaunchDaemons");
    run_command(
        "launchctl",
        vec!["load", "/Library/LaunchDaemons/homebrew.mxcl.dnsmasq.plist"],
    );
}

fn setup_nginx(upstream_server: &str, server_name: &str, root: &str) {
    println!("setting up nginx: {}", upstream_server);
    // ln -sfv /usr/local/opt/nginx/*.plist ~/Library/LaunchAgents
    // launchctl load -w ~/Library/LaunchAgents/homebrew.mxcl.nginx.plist
    run_command(
        "launchctl",
        vec![
            "load",
            "-w",
            "~/Library/LaunchAgents/homebrew.mxcl.nginx.plist",
        ],
    );
}

fn main() {
    let app = App::new("TacoDev")
        .version("0.1.0")
        .author("Fraser Xu <xvfeng123@gmail.com>")
        .about("Taco Dev")
        .subcommand(
            SubCommand::with_name("dnsmasq")
                .about("Set up dnsmasq")
                .arg(
                    Arg::with_name("domain")
                        .short("d")
                        .long("domain")
                        .value_name("DOMAIN")
                        .help("Local domain to setup."),
                ),
        )
        .subcommand(
            SubCommand::with_name("nginx").about("Set up NGINX").arg(
                Arg::with_name("upstream")
                    .short("u")
                    .long("upstream")
                    .value_name("UPSTREAM")
                    .help("Nginx upstream server, can be either localhost or a local socket file."),
            ).arg(
                Arg::with_name("server")
                    .short("s")
                    .long("server")
                    .value_name("SERVER")
                    .help("The name for the server to setup."),
            ).arg(
                Arg::with_name("root")
                    .short("r")
                    .long("root")
                    .value_name("ROOT")
                    .help("Root path for the server."),
            ),
        );

    let matches = app.get_matches();
    if let Some(matches) = matches.subcommand_matches("dnsmasq") {
        println!("Setting up dnsmasq...");

        let domain = matches.value_of("domain").unwrap_or("test");
        setup_dnsmasq(domain);
    }

    if let Some(matches) = matches.subcommand_matches("nginx") {
        println!("Setting up nginx...");
        let upstream_server = matches.value_of("upstream").unwrap_or("127.0.0.1:8000");
        let server_name = matches.value_of("server").unwrap_or("taco-dev");
        let root = matches.value_of("root").unwrap_or("");

        setup_nginx(upstream_server, server_name, root);
    }
}
