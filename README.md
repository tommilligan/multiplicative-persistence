# multiplicative-persistence

![CircleCI branch](https://img.shields.io/circleci/project/github/tommilligan/multiplicative-persistence/master.svg)

Search for integers with high multiplicative persistence values.

## Installation

This project uses the standard `rustup` and `cargo` toolchain. Originally tested against `rustc 1.33.0`.

## Usage

A single binary `mpersist` will be built. Run with `cargo run`. Subcommands are:

- `search`: look for the smallest integer with the largest multiplicative persistence value
- `for`: get the multiplicative persistence of any positive integer

## Examples

```bash
cargo run -- for 12  # 1
cargo run -- for 77  # 4
```

```bash
cargo run -- search
3 39
4 77
# etc.

# Search for mp values
# - starting with integers 233 digits in length
# - search for 10 rounds (i.e. search up to 243 digits in length)
# - use 4 threads
cargo run -- search -f 233 -n 10 -t 4
# this might take some time!
```
