
cargo build -p javy-core --target=wasm32-wasi -r
# Disable link-time optimization when developing for faster compiles
CARGO_PROFILE_RELEASE_LTO=off
cargo build  -p javy-cli -r