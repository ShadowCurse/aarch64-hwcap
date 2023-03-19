# HWCAP checker for aarch64

Reads values of `HWCAP` and `HWCAP2` and outputs all features in them.

## Build

```bash
$ cargo build --release
```

## Run

```bash
$ cargo run --release
```

Optionaly can pretty print the table with `-t` option

```bash
$ cargo run --release -- -t
```
