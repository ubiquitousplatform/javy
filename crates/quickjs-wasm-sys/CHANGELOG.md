# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

- Added: `QUICKJS_WASM_SYS_WASI_SDK_MAJOR_VERSION` and `QUICKJS_WASM_SYS_WASI_SDK_MINOR_VERSION` build-time environment variables to control which version of the WASI SDK to use.
- Fixed: Changing the `QUICKJS_WASM_SYS_WASI_SDK_PATH` build time environment variable will trigger a rebuild of the crate.

## 1.0.0 - 2023-05-16

No changes from 0.1.2. Just updating version to show we're confident in the existing bindings.
