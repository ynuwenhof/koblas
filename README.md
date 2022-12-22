# koblas

A lightweight, configurable and high performance socks5 proxy server.

* No authentication.
* Username and password authentication.

## Configuration

```toml
[server]
addr = "127.0.0.1:1080"
auth = true

[users]
alice = "$argon2id$v=19$m=4096,t=3,p=1$ixDChRX/PY3HTP3b+majlQ$QFuft73Gl6ETGO1NFmh+ZXyzaqL6IeVOMQYe+k16lk4"
bob = "$argon2id$v=19$m=4096,t=3,p=1$31S8wbUC6BbX3xaJSpTLBQ$7YZbzCsX23vRQZZLR5ptGT3wt+U1Uj53r4jsvGU+6zQ"
```

## License

This project is licensed under either of the following licenses, at your option:

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](https://github.com/ynuwenhof/koblas/blob/main/LICENSE-APACHE))
* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](https://github.com/ynuwenhof/koblas/blob/main/LICENSE-MIT))
