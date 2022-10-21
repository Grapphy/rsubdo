# rsubdo
Yet another subdomain enumeration tool.

Sources:
- Alienvault
- Anubis
- Crtsh
- Hackertarget
- Omnisint
- Threatminer

# Usage
```console
$ ./rsubdo example.com
```

# Build
```console
$ git clone https://github.com/Grapphy/rsubdo
$ cd rsubdo
$ cargo build --release
$ cd target/release
$ ./rsubdo example.com
```

# Output
```console
$ ./rsubdo facebook.com
edge-livestream-api-upload-shv-01-cai1.facebook.com
edgeray-shv-01-sjc2.facebook.com
livestream-edgetee-upload-shv-01-mia1.facebook.com
edge-z-1-p2-shv-01-amt2.facebook.com
liverail-pp-shv-01-lga3.facebook.com
edgeray-shv-02-dft4.facebook.com
... skipped ...
edge-snaptu-http-p2-shv-01-nrt1.facebook.com
edgeray-shv-01-ord1.facebook.com
edge-livestream-api-upload-shv-01-ams3.facebook.com
freeway-pp-shv-01-nrt1.facebook.com
atlas-proxyprotocol-shv-01-nrt1.facebook.com
edge-z-mini-shv-01-gru2.facebook.com
edge-z-mini-shv-01-lga3.facebook.com
eir.facebook.com

Scrapped 12640 subdomains from facebook.com in 2.2724979s
```