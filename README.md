# Koblas

A lightweight [SOCKS5](https://datatracker.ietf.org/doc/html/rfc1928) proxy server, written in [Rust](https://rust-lang.org).

* Multi-User
* Configurable
* IPv4 & IPv6
* Blacklist & Whitelist
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
koblas -a 0.0.0.0 --auth -c /path/to/config.toml
```

this will bind the server to `0.0.0.0:1080`.

### Docker

Make sure the [Docker Engine](https://docs.docker.com/engine/install) is installed.

Pull the latest image from the [Docker Hub](https://hub.docker.com) registry with:

```bash
docker pull ynuwenhof/koblas:latest
```

Run the following commands to build the image yourself instead:

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

After pulling or building the image, you can run the server with:

```bash
docker run -d -p 1080:1080 \
  -v /path/to/config.toml:/etc/koblas/config.toml \
  -e RUST_LOG=debug \
  -e KOBLAS_NO_AUTHENTICATION=false \
  -e KOBLAS_ANONYMIZE=false \
  --name koblas ynuwenhof/koblas:latest
```

this will bind the server to `0.0.0.0:1080`.

Deploy the server with Docker Compose:

```yaml
version: "3.8"
services:
  koblas:
    image: ynuwenhof/koblas:latest
    container_name: koblas
    restart: unless-stopped
    ports:
      - 1080:1080
    environment:
      RUST_LOG: debug
      KOBLAS_LIMIT: 256
      KOBLAS_NO_AUTHENTICATION: false
      KOBLAS_ANONYMIZATION: true
    volumes:
      - /path/to/config.toml:/etc/koblas/config.toml
```

## Configuration

Koblas can be configured via environment variables or command line arguments.

Missing keys will fall back to their default value.

| Key                        | Description                                                                 | Default     |
|----------------------------|-----------------------------------------------------------------------------|-------------|
| `KOBLAS_ADDRESS`           | Address on which to listen for incoming TCP connections                     | `127.0.0.1` |
| `KOBLAS_PORT`              | Port on which to listen for incoming TCP connections                        | `1080`      |
| `KOBLAS_LIMIT`             | Maximum amount of clients to handle at once                                 | `255`       |
| `KOBLAS_NO_AUTHENTICATION` | Don't require clients to authenticate using a username/password combination | `false`     |
| `KOBLAS_ANONYMIZATION`     | Exclude sensitive information from the logs                                 | `false`     |
| `KOBLAS_CONFIG_PATH`       | File path to the config file                                                | `None`      |

> :warning: The default configuration requires everyone to connect with a pre-existing username/password combination.

Koblas doesn't have a default config file location, but we recommend the following locations:

* Linux: `/etc/koblas/config.toml`
* MacOS: `/etc/koblas/config.toml`
* Windows: `%ProgramData%\koblas\config.toml`

### Example

```toml
# All matching IPs will be automatically blocked
blacklist = [
    # Blacklist all IPs between
    # 192.168.2.0 and 192.168.2.255
    "192.168.2.0/24",
    # Blacklist a single IP
    "192.168.3.0/32",
]

# All non matching IPs will be automatically blocked
# Keep empty to disable the whitelist
whitelist = [
    # Whitelist all IPs between
    # 192.168.0.0 and 192.168.0.255
    "192.168.0.0/24",
    # Whitelist a single IP
    "192.168.1.0/32",
]

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
