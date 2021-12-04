# Executable Crates

Executable or Binary Crates are binaries that are typically stored in [crates.io](http://crates.io)
and installed via `cargo install <crate>`.

Below are some executeble crates that I have found useful with associated notes.

- cargo install cargo-upadte
  - [cargo-update](https://crates.io/crates/cargo-update)
    - [example](crates/executable/cargo-update.md)
  - then `cargo install-update --list` will list all the installed executables to check if they are up to date.
  - `cargo install-update --all` will update all installed executables to check if they are up to date.
- [ripgrep](https://crates.io/crates/ripgrep)
  - better, faster grep
