# taco-dev
A command line tool to setup a local domain environment for development using nginx and dnsmasq.

**Status:** Not ready.

```
TacoDev 0.1.0
Fraser Xu <xvfeng123@gmail.com>
Taco Dev

USAGE:
    taco-dev [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    dnsmasq    Set up dnsmasq
    help       Prints this message or the help of the given subcommand(s)
    nginx      Set up NGINX
```

### Requirements

* [`dnsmasq`](http://www.thekelleys.org.uk/dnsmasq/doc.html) - Dnsmasq provides network infrastructure for small networks: DNS, DHCP, router advertisement and network boot. 
* [`nginx`](https://nginx.org/en/) - HTTP and reverse proxy server.

### Usage

1. Setup `dnsmasq`

```sh
$ taco-dev ndsmasq --domain=test
```

2. Setup `nginx`

```sh
$ taco-dev nginx --upstream=127.0.0.1:8000 --domain=taco-dev
```

3. :tada: 

```sh
$ open http://taco-dev.test 
```

### License
MIT