# Development Environment Prerequisites

This document outlines the system requirements and installation steps needed to develop Rewordle.

## Stack

* Rust
* Cargo
* Incredible
* WebAssembly
* macOS AppKit (via objc2)

## GitHub Authentication

This project depends on the private [incredible-alpha](https://github.com/ronilan/incredible-alpha) crate, pulled by Cargo as a git dependency over HTTPS. The first build will fail until your machine can read that repo.

The easiest way to authenticate is the [GitHub CLI](https://cli.github.com/) — download and install it from [cli.github.com](https://cli.github.com/) if you don't have it already:

```bash
gh auth login
```

This configures git's credential helper, so `git` (and Cargo fetching through it) can access the private repository without prompting.

Alternatively, create a fine-grained Personal Access Token with read access to `ronilan/incredible-alpha` and store it with your OS credential manager (macOS Keychain / Windows Credential Manager).

## macOS

1. **Install Xcode Command Line Tools**:
   ```bash
   xcode-select --install
   ```
2. **Install Rust** - Download and run the installer from [rustup.rs](https://rustup.rs/)
3. **Install wasm-pack**:
   ```bash
   cargo install wasm-pack
   ```


