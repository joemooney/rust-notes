# Dependencies for building the Book

See `setup.sh` where I have cobbled some of the installation commands together into a script.

## Install rust

<https://www.rust-lang.org/tools/install>

## Install graphvix

`sudo apt install default-jre`
`sudo apt install graphviz`

## NPM Modules (optional)

`npm i -g live-server`

## Install mdbook

`cargo install mdbook`

## Install mdbook-plus  (mine)

- `gh clone mdbook-plus`
- `cd mdbook-plus`
- `./install.sh`

## Install plantuml

<https://github.com/plantuml/plantuml/releases/tag/v1.2021.16>
<https://plantuml.com/download>

- `wget  https://github.com/plantuml/plantuml/releases/download/v1.2021.16/plantuml-1.2021.16.jar`
- `sudo apt install libssl-dev`
- `cargo install mdbook-plantuml`

## Troubleshootking

- failed complaining about openssl library not found, fix: sudo apt install libssl-dev
- `mdbook build` complains about plantuml not installed, fix: maybe `mdbook-plantuml is not installed
- `Readme` is empty; fix `cp README.md md/README.md`

___

## Other Goodies

A place to note any package of interest to install on a new machine.

### Install gh

github's official command line tool.

<https://github.com/cli/cli/blob/trunk/docs/install_linux.md>

### Install vs code

<https://code.visualstudio.com/docs/?dv=linux64_deb>

### Miscellaneous Packages/Crates

- `cargo install cargo-edit`
- `cargo install mdcat` - nice cat command for md files.
- `sudo apt install jq` - json query tool, it also pretty prints json piped to it

To get focus to follow mouse :

- `sudo apt install gnome-tweaks` - then run `gnome-tweaks` and enable `Secondary Focus` under `Windows`

### npm

Install nvm as recommended by npm <https://docs.npmjs.com/downloading-and-installing-node-js-and-npm>

- `curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash`

Install npm/node via `nvm`

- `nvm install node`

### wrangler

- `cargo install wrangler`
