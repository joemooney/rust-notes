# Cargo

- To create a new project crate:
  -  ```cargo new mycrate```
  -  ```cd mycrate```
  -  git init .
  -  hub new  (hub is deprecated, github now has a `gh` command)

- To upgrade all your your existing crates you have installed run ```cargo install-update -a```

Say you have an existing directory, an empty git repo.
<br>To turn this into a Rust project you run:

```cargo init . --bin```

If you then execute ```cargo run``` it will compile and run it, printing out **Hello World!**


## Cargo Add-ons
```cargo install cargo-edit```
this allows you to ```cargo add/rm <crate>``` and ```cargo upgrade --dry-run```