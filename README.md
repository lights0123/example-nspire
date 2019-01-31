# A Rust and Ndless example

## Prerequisites
- Rustup installed
- Latest Rust Nightly installed
- `jq`
- Unix-like (tested on Linux, most likely Mac and Cygwin will work as well)

Complete install script:
```bash
curl https://sh.rustup.rs -sSf | sh
rustup install nightly
rustup default nightly
rustup component add rust-src
cargo install cargo-xbuild
```

## Building
Run `./build.sh`

## Roadmap
- Add release build script
