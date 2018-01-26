# taco-dev

A command line tool to setup a local domain environment for development using `nginx` and `dnsmasq`.

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
$ taco-dev dnsmasq --domain=test
```

To verify:

```sh
$ ping teco-dev.test
PING teco-dev.test (127.0.0.1): 56 data bytes
```

2. Setup `nginx`

To tell `nginx` to proxy a request to port `80`, we need to defined the upstream server. It can be either a local server running on a specific port `localhost:8080` or a unix socket object `/tmp/example.test`.

```sh
$ taco-dev nginx --upstream=127.0.0.1:8000 --domain=taco-dev --root=/Users/fraserxu/projects/taco-dev;
```

This will add a `taco-dev.conf` in `nginx/servers` directory.

```nginx
upstream taco-dev.test {
    server unix:/tmp/taco-dev;
}

server {
    listen 80;
    server_name taco-dev.test;
    root /Users/fraserxu/projects/taco-dev;

    try_files $uri/index.html $uri @taco-dev.test;

    location @taco-dev.test {
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header Host $http_host;
        proxy_redirect off;

        proxy_pass http://taco-dev.test;
    }
}
```

3. :tada:

```sh
$ open http://taco-dev.test
```

### License

MIT
