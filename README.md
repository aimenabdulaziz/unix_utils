# Rust Command Line Utilities

This project implements several common Unix command line utilities in Rust. Each utility is implemented as a separate binary.

## Description

This project includes the following utilities:

1. **recho** - Display a line of text
2. **rcat** - Concatenate and print files
3. **rhead** - Display first lines of files
4. **rmv** - Move files
5. **rcp** - Copy files

A separate `README.md` file is included for each utility.

## How to build and run the project

### Prerequisites

- Rust and Cargo (latest stable version)

### Building

To build all utilities, run:

```bash
cargo build --release
```

To build a specific utility, navigate to its directory and run:

```bash
cd recho
cargo build --release
```

### Running

After building, you can run each utility from its respective directory:

```bash
# From the main project directory
./target/release/recho "Hello, world!"
./target/release/rcat file.txt
./target/release/rhead -n 5 file.txt
./target/release/rmv source.txt target.txt
./target/release/rcp source.txt target.txt
```

Or you can run them directly with cargo:

```bash
cargo run --bin recho -- "Hello, world!"
cargo run --bin rcat -- file.txt
cargo run --bin rhead -- -n 5 file.txt
cargo run --bin rmv -- source.txt target.txt
cargo run --bin rcp -- source.txt target.txt
```

ach utility supports the `--help` flag to display usage information and available options:

```bash
./target/release/recho --help
./target/release/rcat --help
./target/release/rhead --help
./target/release/rmv --help
./target/release/rcp --help
```

## Assumptions

1. All utilities handle UTF-8 encoded text files.
2. For `rcat` and `rhead` , when no files are specified, they read from standard input.
3. For `rmv` and `rcp`, both source and target must be specified.
4. For `rmv` and `rcp`, if the target is a directory, the source file will be moved/copied into that directory.
5. Error messages are printed to stderr, while normal output goes to stdout.
6. All utilities return appropriate exit codes (0 for success, non-zero for errors).
