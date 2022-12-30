# Koblas

A lightweight SOCKS5 proxy server, written in Rust.

## Features

* Multi-user
* Configurable
* No authentication
* Username and password authentication

## Installation

### Cargo

Make sure the current stable release of [Rust](https://rust-lang.org/tools/install) is installed.

```
cargo install koblas
```

### Manual

Make sure the current stable release of [Rust](https://rust-lang.org/tools/install) is installed.

```
git clone https://github.com/ynuwenhof/koblas.git
cd koblas
cargo install .
```

## Configuration

Koblas doesn't have a default config file location, but we recommend the following:

* Linux: `/etc/koblas/koblas.toml`
* MacOS: `/etc/koblas/koblas.toml`
* Windows: `%ProgramData%\koblas\koblas.toml`

### [server] section

Missing keys in the configuration will use their default value.

| Keys   | Description                                                    | Default           |
|--------|----------------------------------------------------------------|-------------------|
| `addr` | Socket address on which to listen for incoming TCP connections | `"127.0.0.1:1080"`|
| `auth` | Require clients to authenticate using a username and password  | `false`           |
| `anon` | Exclude sensitive information from the logs                    | `false`           |

> :warning: The default configuration allows anyone to connect without authenticating!

### Example

```toml
[server]
addr = "0.0.0.0:1080"
auth = true
anon = false

[users]
# Username = "alice", password = "QDuMGlxdhpZt"
alice = "$argon2id$v=19$m=8,t=2,p=1$bWUwSXl2M2pYNU9xcVBocw$f4gFaE7p0qWRKw"
# Username = "bob", password = "ceQvWaDGVeTv"
bob = "$argon2id$v=19$m=8,t=2,p=1$ZExzaTM3aks1WjU1a3g4UA$J+EiueHYuR/dlA"
```

## License

This project is licensed under either of the following licenses, at your option:

* [Apache License, Version 2.0](https://apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](https://github.com/ynuwenhof/koblas/blob/main/LICENSE-APACHE))
* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](https://github.com/ynuwenhof/koblas/blob/main/LICENSE-MIT))
