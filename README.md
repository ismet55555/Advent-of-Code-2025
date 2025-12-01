# Advent-of-Code-2025

Another year, another attempt at this frustrating thing.
This time 12 days only.

Language: [Rust](https://www.rust-lang.org/)

## Setup For Each Day

Copy the `day_template/` directory and rename it as the
current day (i.e. `day_1`):

```sh
# Linux/MacOS
cp -r day_template day_1

# Windows Powershell
Copy-Item -Recurse day_template -Destination day_1
```

## Running Code For Each Day

```sh
cd day_x
cargo run -q
```

## Tests For Each Day

Tests are based on the sample input provided for each day
(i.e. `./inputs/sample_part1.txt`).

```sh
cd day_x
cargo test -q
```

## Debugging Code Issues

To actively monitor code issues use [`bacon`](https://github.com/Canop/bacon).

Install `bacon`:

```sh
cargo install --locked bacon
```

Run `bacon`:

```sh
bacon --all-features --job clippy
```
