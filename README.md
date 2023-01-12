# Koblas

A lightweight [SOCKS5](https://datatracker.ietf.org/doc/html/rfc1928) proxy server, written in [Rust](https://rust-lang.org).

* Multi-User
* Configurable
* No Authentication
* [Username/Password](https://datatracker.ietf.org/doc/html/rfc1929) Authentication

## Installation

### Cargo

Make sure the current stable release of [Rust](https://rust-lang.org/tools/install) is installed.

#### Registry

```
cargo install koblas
```

#### Manual

```
git clone https://github.com/ynuwenhof/koblas.git
cd koblas
cargo install --path .
```

## Configuration

Koblas doesn't have a default config file location, but we recommend the following locations:

* Linux: `/etc/koblas/koblas.toml`
* MacOS: `/etc/koblas/koblas.toml`
* Windows: `%ProgramData%\koblas\koblas.toml`

Missing keys in the configuration file will fallback to their default value.

| Keys   | Description                                                    | Default            |
|--------|----------------------------------------------------------------|--------------------|
| `addr` | Socket address on which to listen for incoming TCP connections | `"127.0.0.1:1080"` |
| `auth` | Require clients to authenticate using username/password        | `false`            |
| `anon` | Exclude sensitive information from the logs                    | `false`            |

> :warning: The default configuration allows anyone to connect without having to authenticate!
 
## License

This project is licensed under either of the following licenses, at your option:

* [Apache License, Version 2.0](https://apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](https://github.com/ynuwenhof/koblas/blob/main/LICENSE-APACHE))
* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](https://github.com/ynuwenhof/koblas/blob/main/LICENSE-MIT))
