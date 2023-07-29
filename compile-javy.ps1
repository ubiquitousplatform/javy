$env:QUICKJS_WASM_SYS_WASI_SDK_MAJOR_VERSION = "20"
$env:QUICKJS_WASM_SYS_WASI_SDK_MINOR_VERSION = "0.m"

# TODO: install chocolatey and then choco install make cmake llvm
# TODO: may need to also install mingw?

cargo build -p javy-core --target=wasm32-wasi -r
# Disable link-time optimization when developing for faster compiles
$env:CARGO_PROFILE_RELEASE_LTO = "off"
cargo build  -p javy-cli -r