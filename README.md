# Rust Beginner's Guide
### **by Joe Mooney**

A beginner's guide to programming in Rust written by a beginner and a beginner's guide to writing an *mdbook* also by a beginner mdbook user. 

This book is generated using a gitbook-like clone called *mdbook*. The book is stored in github and the book serves the additional purpose of documenting how to create such a book.

Included are commands and instructions for creating a new empty book and how to go about publishing the book on github.

In addition to `mdbook` there some add-ons to mdbook stored in this repo. There is a preprocessor that makes it easy to generate an interactive quiz.

## Installation
- `cargo install mdbook`
- If you get an error about cannot find crti.o: `sudo apt install libc-dev`

## Viewing the Book 

- View the book in github here:  [https://joemooney.github.io/rust-notes/](https://joemooney.github.io/rust-notes/)
- View the book locally: [file:///home/jpm/rust/rust-notes/book/index.html](file:///home/jpm/rust/rust-notes/book/index.html)
- View locally while making updates: ```mdbook watch --open```

## Goals

- document lessons learned in Rust from a beginner's perspective.
- create a template and supporting scripts for writing similarly layed out books on any subject.

## Layout of Book repo
- The md directory contains the book
- A README.md at the root level which is synced with a copy in the md folder
- md/SUMMARY.md all pages must be recorded in this file - this is table of contents in the left bar
- src/*.rs is where all the example rust code is stored, the md files have references via `{{#include ../src/misc/anchors.rs:anchor1}}` for example.

## Authoring the Book

- Build/View loop: ```mdbook watch --open```
    - Opens the local book in a browser
    - Then you edit pages in md subdirectory
    - This will trigger a rebuild upon any change of the book
    - See the updates on browser with web page refresh.
- Just Build/View the book locally: ```mdbook build --open```
- *Note*: README.md in the root directory of the repo is a copy of md/README.md and you should edit the version in the md directory. `publish.sh` will copy the newer of either of these to the other, but the watch will not trigger a rebuild unless you edit the md directory version.
- Publish to github: `./publish.sh` shell script will generate and will publish changes to the book to github.

## Dependencies
### mdbook-plus
Clone, build, and install this repo `https://github.com/joemooney/mdbook-plus`
This is a mdbook preprocessor that I wrote. I use it for colored text and my quiz feature since I could not find an easier way to do these things.
If you get a WARN ```2021-12-04 14:45:30 [WARN] (mdbook::preprocess::cmd): The command wasn't found, is the "plus" preprocessor installed?``` when you run ``mdbook build --open`` it means you have not installed my mdbook helper mdbook-plus
To install clone the repo and then `cargo build` and `cargo install --path .`

## Install/Bootstrap

- Clone the GitHub repo for the book: [https://github.com/joemooney/rust-notes/](https://github.com/joemooney/rust-notes/)

Once opened, the book contains the instructions for
creating a new book, and editing, publishing etc.

The book contents (markdown) is contained in the md directory.
Any time you change contents in the md directory then the ```mdbook watch --open``` will detect the change and regenerate the book locally.

### Publishing
To publish your changes to github run the `publish.sh` script. Use the `-f` force option if it reports there are no changes but you really know that there are and want to publish anyhow. Publishing will require access to github. It is best to install the *gh* command line tool from github `https://github.com/cli/cli` and then `gh auth login` and create a SSH login for your account.
Once published then other people will be able to see your book in <your_name>.github.io/<your_repo> in a few minutes. 

### Checking your Installation
## Interactive Code Example Feature
When viewing the book, make sure this feature is working so that you can use the interactive editor to run Rust code while reading the book.

Here is an example of embedding a question and answer code block into the book which the reader can edit and run using *rust playground*:
If you see a '?Q' and '?A' then you have not installed *mdbook-plus* (see above)

?Q
What will happen when you run this code? {click on the arrow to reveal the answer}
```rust,editable
#fn main(){
let x:Option<u32> = None.unwrap();
#}
```

?A
It panics, because you cannot unwrap a None. 
It is best not to unwrap unless you know there is no possibility of failure.

?E

