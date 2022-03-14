# Welcome 👋

This repository houses various pieces of functionality used across Fennel Labs' Rust-based projects.

# Testing
To avoid testing thread issues with the shared database files, run tests with the following command:
```
cargo test -- --test-threads 1
```

# Dependencies
Prior to testing, you'll need to install OpenSSL and LLVM, as well as assorted other dependencies on Linux.

## Ubuntu
```
apt-get install unzip curl build-essential protobuf-compiler clang libclang-dev libclang1 llvm llvm-dev clang-tools
```

## macOS
```
brew install openssl cmake llvm
```

## Windows
- [openssl on windows](https://github.com/fennelLabs/fennel-lib/issues/1)
- [llvm on windows](https://community.chocolatey.org/packages/llvm)
