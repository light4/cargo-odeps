# cargo-odeps

Easy manage rust crate deps by `cargo odeps`.

[![CI](https://github.com/light4/cargo-odeps/actions/workflows/test.yaml/badge.svg)](https://github.com/light4/cargo-odeps/actions/workflows/test.yaml)
[![build-and-release](https://github.com/light4/cargo-odeps/actions/workflows/build-and-release.yaml/badge.svg)](https://github.com/light4/cargo-odeps/actions/workflows/build-and-release.yaml)

## Install

```bash
# from crates.io
cargo install cargo-odeps --force
# from git repo
cargo install --git https://github.com/light4/cargo-odeps.git --force
```

## Usage

```bash
~ on  master 🕙 22:28:47
❯ cargo odeps --help
Easy manage rust crate deps by `cargo odeps`

Usage: cargo odeps [OPTIONS]

Options:
  -u, --upgrade            upgrade all outdated
  -o, --outdated           show outdated
  -l, --no-ignore-local    don't ignore deps from local space
  -i, --ignore <IGNORE>    ignore from upgrade
  -p, --project <PROJECT>  project only
  -v, --verbose            show details
  -h, --help               Print help
```
