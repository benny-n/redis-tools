<div align="center">
  <h1>redis-tools</h1>
  <p>
    Tools for dumping (and restoring) redis databases into files.
  </p>
  <h2>

![LICENSE](https://img.shields.io/github/license/benny-n/hangoff)

</h2>

</p>

</div>

# Usage

```bash
$ redis-dump -u [REDIS_URI]               # Dump to STDOUT
$ redis-dump -u [REDIS_URI] > dump.json   # Dump into file
```

### Other ways to pass the URI:

```bash
# Take uri from env
$ REDIS_URI=redis://localhost:6379 redis-dump > dump.json

# Take the default uri (redis://localhost:6379)
$ redis-dump > dump.json
```

# Development with Cargo

## Build

```bash
$ cargo build
```

## Run

```bash
# Run redis-dump
$ ./target/debug/redis-dump --help
# Run redis-restore
$ ./target/debug/redis-restore --help
```

## Build & Run

```bash
# Build & Run redis-dump
$ cargo run --bin redis-dump -- --help
# Build & Run redis-restore
$ cargo run --bin redis-restore -- --help
```

# Development with Docker

## Build

```bash
$ docker build . -t redis-tools
```

## Run

```bash
$ docker run --rm -v :/redis-tools --help
```
