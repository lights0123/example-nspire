# A Rust and Ndless example

## Prerequisites
- Ndless toolchain installed and added to path
- Rustup installed
- Latest Rust Nightly installed
- `jq`
- Unix-like (tested on Linux, most likely Mac and Cygwin will work as well)

Complete install script:
```bash
curl https://sh.rustup.rs -sSf | sh # skip if rustup already installed
rustup install nightly # skip if nightly already installed
cargo install cargo-make
```

## Building
To create a debug version, run `rustup run nightly cargo make dev`. To create
a production version, run `rustup run nightly cargo make release`. Binaries will
be in `target/armv5te-nspire-eabi/(debug or release)/name.tns`.


You may skip `rustup run nightly` if you set nightly as your default compiler
(`rustup default nightly`).
## Additional Binary Options
In your `Cargo.toml`, the following options can be added:
```toml
[package.metadata.zehn]
name = "Hello World" # Name passed to genzehn
compress = true      # Compress binary with genzehn
```

# Make your own app
This project is set up using Cargo workspaces, which is not ideal for single-application
setups. Instead, simply copy any of the directories into its own repository and 
write code there.
