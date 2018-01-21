extern crate clap;

use clap::{App, Arg, SubCommand};

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
    }

    if let Some(matches) = matches.subcommand_matches("nginx") {
        println!("Setting up NGINX");
        let upstream_server = matches.value_of("upstream").unwrap_or("127.0.0.1:8000");
        println!("Upstream server: {}", upstream_server);
    }
} 