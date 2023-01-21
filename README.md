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

```bash
cargo install koblas
```

#### Manual

```bash
git clone https://github.com/ynuwenhof/koblas.git
cd koblas
cargo install --path .
```

Hash passwords by running the following command:

```bash
koblas hash "correct-horse-battery-staple"
```

this will return an [Argon2id](https://en.wikipedia.org/wiki/Argon2) password hash.

After installing, you can run the server with:

```bash
koblas -a 0.0.0.0 --auth -u path/to/users.toml
```

this will bind the server to `0.0.0.0:1080`.

### Docker

Make sure the [Docker Engine](https://docs.docker.com/engine/install) is installed.

There's currently no image shared on the [Docker Hub](https://hub.docker.com) registry, you will have to build one yourself.

Run the following commands to build the image:

```bash
git clone https://github.com/ynuwenhof/koblas.git
cd koblas
docker build -t ynuwenhof/koblas:latest .
```

Hash passwords by running the following command:

```bash
docker run -it --rm ynuwenhof/koblas:latest hash "correct-horse-battery-staple"
```

this will return an [Argon2id](https://en.wikipedia.org/wiki/Argon2) password hash.

After building the image, you can run the server with:

```bash
docker run -d -p 1080:1080 \
  -v path/to/users.toml:/etc/koblas/users.toml \
  -e RUST_LOG=debug \
  -e KOBLAS_AUTHENTICATE=true \
  -e KOBLAS_ANONYMIZE=false \
  --name koblas ynuwenhof/koblas:latest
```

this will bind the server to `0.0.0.0:1080`.

## Configuration

Koblas can be configured via environment variables or command line arguments.

Missing keys will fallback to their default value.

| Key                   | Description                                             | Default     |
|-----------------------|---------------------------------------------------------|-------------|
| `KOBLAS_ADDRESS`      | Address on which to listen for incoming TCP connections | `127.0.0.1` |
| `KOBLAS_PORT`         | Port on which to listen for incoming TCP connections    | `1080`      |
| `KOBLAS_AUTHENTICATE` | Require clients to authenticate using username/password | `false`     |
| `KOBLAS_ANONYMIZE`    | Exclude sensitive information from the logs             | `false`     |
| `KOBLAS_USERS_PATH`   | File path to the list of existing users                 | `None`      |

> :warning: The default configuration allows anyone to connect without having to authenticate!

Koblas doesn't have a default users file location, but we recommend the following locations:

* Linux: `/etc/koblas/users.toml`
* MacOS: `/etc/koblas/users.toml`
* Windows: `%ProgramData%\koblas\users.toml`

### Example

```toml
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
