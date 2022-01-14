install gh
https://github.com/cli/cli/blob/trunk/docs/install_linux.md

install rust
https://www.rust-lang.org/tools/install

install code
https://code.visualstudio.com/docs/?dv=linux64_deb

install mdbook
cargo install mdbook

install mdbook-plus  (mine)
gh clone mdbook-plus
cd mdbook-plus
./install.sh


cargo install cargo-edit
sudo apt install default-jre
sudo apt install graphviz

## install plantuml
https://github.com/plantuml/plantuml/releases/tag/v1.2021.16
https://plantuml.com/download
wget  https://github.com/plantuml/plantuml/releases/download/v1.2021.16/plantuml-1.2021.16.jar

sudo apt install libssl-dev
cargo install mdbook-plantuml

## Troubleshootking

- failed complaining about openssl library not found, fix: sudo apt install libssl-dev
- `mdbook build` complains about plantuml not installed, fix: maybe `mdbook-plantuml is not installed
- `Readme` is empty; fix `ln -s 

