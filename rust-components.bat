rustup target add wasm32-unknown-unknown
rustup target add x86_64-pc-windows-msvc
rustup component add rust-src clippy rustfmt rust-analysis rust-analyzer-preview
cargo install cargo-play cargo-edit watchexec-cli wasm-pack cargo-udeps cargo-expand twiggy --force
