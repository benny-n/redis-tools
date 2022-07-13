
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