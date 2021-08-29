# Crates

Crates are libraries or binaries that are typically stored in [crates.io](http://crates.io)
An alterative index that for me is easier to find crates of interest is: [lib.rs](http://lib.rs)

Below are some crates that I have found useful with associated notes.

## Coding
- [anymap](https://crates.io/crates/anymap)
  - [example](crates/anymap.md)

There are many cool crate executables and the running cargo install to install them is a breeze (installation will take some time depending on the number of dependencies).

## Subprocess/Shell
- [duct](https://docs.rs/crate/duct/0.13.5)
  - This will run *shell* commands for you and manage stdout/stderr.
  - [example](crates/duct.md)
  
## MdBook related

- cargo install [mdbook-mermaid](https://github.com/badboy/mdbook-mermaid)
  - cool flowcharts and other graphs for your book

## Executable Crates
- cargo install cargo-upadte
  - [cargo-update](https://crates.io/crates/cargo-update)
    - [example](crates/executable/cargo-update.md)
  - then `cargo install-update --list` will list all the installed executables to check if they are up to date.
  - `cargo install-update --all` will update all installed executables to check if they are up to date.
- [ripgrep](https://crates.io/crates/ripgrep)
  - better, faster grep
