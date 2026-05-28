# Grep, now in Rust

A Rust implementation of [GNU Grep](https://www.gnu.org/software/grep/).
This project is an initial release and may contain bugs.

## Building

Download Rust at: https://rustup.rs/

```shell
# Check out this repository
git clone https://github.com/uutils/grep
cd grep

# Build a release version
cargo build --release

# Run!
./target/release/grep --help

# Run tests (if needed; after making changes)
cargo test
```

## Known Issues

* Does not take `LANG`, etc., into account for handling file encodings (non-UTF8 matches are treated as binary)

## Contributing

To contribute to uutils, please see [CONTRIBUTING](CONTRIBUTING.md).

## License

uutils is licensed under the MIT License - see the `LICENSE` file for details

GNU Grep is licensed under the GPL 3.0 or later.
