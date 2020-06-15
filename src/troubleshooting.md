# Troubleshooting

## Published book does not show updates

Not sure how long it takes a push to github.com to show up on github.io as pages...but it may take a couple of minutes.

## Pages are not generated

Every page in the book needs to be in the SUMMARY.md in order to be built.

## Can't find crate for `somecrate`

In Rust Playground [https://play.rust-lang.org](https://play.rust-lang.org) only the current [top 100 crates](https://github.com/integer32llc/rust-playground/blob/master/compiler/base/Cargo.toml) are available to be used, so you cannot arbitrarily add code and have it run inside your book on the web page.
