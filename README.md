<h1 align="center">koblas</h1>

## About

A lightweight, configurable and high performance socks5 proxy server.

* No authentication.
* Username and password authentication.

## Configuration

Koblas doesn't create the config file for you and has no default config file location.

Recommended config file locations:

* Linux: `/etc/koblas/koblas.toml`
* Windows: `%ProgramData%\koblas\koblas.toml`

If no config file is provided, the default fallback config will be used.

```toml
[server]
addr = "127.0.0.1:1080"
auth = false
```

> :warning: The default config allows anyone to connect without authenticating.

```toml
[server]
addr = "127.0.0.1:1080"
auth = true

[users]
# username = "alice", password = "QDuMGlxdhpZt"
alice = "$argon2id$v=19$m=8,t=2,p=1$bWUwSXl2M2pYNU9xcVBocw$f4gFaE7p0qWRKw"
# username = "bob", password = "ceQvWaDGVeTv"
bob = "$argon2id$v=19$m=8,t=2,p=1$ZExzaTM3aks1WjU1a3g4UA$J+EiueHYuR/dlA"
```

## License

This project is licensed under either of the following licenses, at your option:

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](https://github.com/ynuwenhof/koblas/blob/main/LICENSE-APACHE))
* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](https://github.com/ynuwenhof/koblas/blob/main/LICENSE-MIT))
