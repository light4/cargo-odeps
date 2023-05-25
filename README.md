# cargo-deps

Easy manage rust crate deps by `cargo deps`.

[![CI](https://github.com/light4/cargo-deps/actions/workflows/test.yaml/badge.svg)](https://github.com/light4/cargo-deps/actions/workflows/test.yaml)
[![build-and-release](https://github.com/light4/cargo-deps/actions/workflows/build-and-release.yaml/badge.svg)](https://github.com/light4/cargo-deps/actions/workflows/build-and-release.yaml)

## Install

```bash
# from crates.io
cargo install cargo-deps --force
# from git repo
cargo install --git https://github.com/light4/cargo-deps.git --force
```

## Usage

```bash
~ on  master 🕙 22:28:47
❯ cargo deps --help
Easy manage rust crate deps by `cargo deps`

Usage: cargo deps [OPTIONS]

Options:
  -u, --upgrade          upgrade all outdated
  -o, --outdated         show outdated
  -l, --no-ignore-local  don't ignore deps from local space
  -i, --ignore <IGNORE>  ignore from upgrade
  -v, --verbose          show details
  -h, --help             Print help
```
