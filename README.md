# MiniGrep

A simple `grep` implementated in Rust.

> Note: I made this toy tool to practice my Rust skills. It is not meant to be used professionally.

## Usage

### Using `cargo` (preferred):

```shell
cargo run -- <QUERY> <FILE_PATH>
```

### Using `rustc`:

1. compile the binary

```shell
rustc --edition=2021 --crate-type=lib -o libminigrep.rlib src/lib.rs
rustc --edition=2021 --extern minigrep=libminigrep.rlib -o minigrep src/main.rs
```

2. run the binary
```shell
./minigrep <QUERY> <FILE_PATH>
```

### Note:

For case insensitive search, set the environment variable `IGNORE_CASE=1`

