# Rust Beginner's Guide

| Author | Date |
| ------ | ---- |
| Joe Mooney | {timestamp} |

A beginner's guide to programming in Rust written by a beginner.  
A beginner's guide to writing a book like this using *mdbook*.

## Instructions on Writing your own Book or Edit this Book

### Installation Dependencies

- `cargo install mdbook`
- If you get an error about cannot find crti.o: `sudo apt install libc-dev`
- `npm i -g live-server` in combination with `mdbook watch --open` this will reload the page anytime there are updates.

There is also another crate `mdbook-plus` I wrote that is used when building the book which has the extra features to simplify editing the book with custom markdown capabilities.

## Viewing the Book while Editing or the Published Version

- View the book in github here:  [https://joemooney.github.io/rust-notes/](https://joemooney.github.io/rust-notes/)
- View the book locally: [file:///home/jpm/rust/rust-notes/book/index.html](file:///home/jpm/rust/rust-notes/book/index.html)
- View locally while making updates: ```./watch.sh```
This will open the book in the browser and listen for changes to the book in the filesystem and regenerate the book.

## Goals

- document lessons learned in Rust from a beginner's perspective.
- create a template and supporting scripts for writing similarly layed-out books on any subject.

## Notes on Markdown

- Code in single backticks looks like `enclosed in single backticks`
- Code in triple backticks looks like ```enclosed in triple backticks```
- When you want a **newline** then end the line in two spaces
- \`\`\` block of code \`\`\`  produces

```bash
block of code
```

- A literal \` is produced with a using \\\`

## Layout of Book repo

- The md directory contains the book
- A README.md at the root level which is synced with a copy in the md folder.  
⚠️ Please edit the md/README.md file and not the one in the top level in order for the changes to be automatically detected. Otherwise, the change will only be synced during publish.
- md/SUMMARY.md all pages must be recorded in this file - this is table of contents in the left bar
- src/*.rs is where all the example rust code is stored, the md files can include code blocks from the same code files via text like
{{{{#include ../src/misc/anchors.rs:anchor4} }}}
will generate ```{{#include ../src/misc/anchors.rs:anchor4}}``` where the code displayed came from

## Authoring the Book

- Build/View loop: `watch.sh` which calls ```mdbook watch --open```
  - Opens the local book in a browser
  - Then you edit pages in md subdirectory
  - This will trigger a rebuild upon any change of the book
  - See the updates on browser with web page refresh.
  - `watch.sh` uses `live-server` which will do a hot reload of the browser.
- Just Build/View the book locally: ```mdbook build```
- *Note*: README.md in the root directory of the repo is a copy of md/README.md and you should edit the version in the `md` directory. `publish.sh` will copy the newer of either of these to the other, but the watch will not trigger a rebuild unless you edit the `md` directory version.
- Publish to github: `./publish.sh` shell script will generate and will publish changes to the book to github.

## Dependencies

### mdbook-plus

Clone, build, and install this repo `https://github.com/joemooney/mdbook-plus`
This is a mdbook preprocessor that I wrote. I use it for colored text and my quiz feature since I could not find an easier way to do these things.
If you get a WARN ```2021-12-04 14:45:30 [WARN] (mdbook::preprocess::cmd): The command wasn't found, is the "plus" preprocessor installed?``` when you run ``mdbook build --open`` it means you have not installed my mdbook helper mdbook-plus
To install clone the repo and then `cargo build` and `cargo install --path .`

## Install/Bootstrap (Cloning this Book and Editing)

- Clone the GitHub repo for this book: [https://github.com/joemooney/rust-notes/](https://github.com/joemooney/rust-notes/)

Once opened, the book contains the instructions for
creating a new book, and editing, publishing etc.

The book contents (markdown) is contained in the md directory.
Any time you change contents in the md directory then the ```./watch.sh``` will detect the change and regenerate the book locally and hot reload the browser.

### Publishing

To publish changes to github run the `publish.sh` script. Use the `-f` force option if it reports there are no changes but you really know that there are and want to publish anyhow. Publishing will require access to github. It is best to install the *gh* command line tool from github `https://github.com/cli/cli` and then `gh auth login` and create a SSH login for your account.
Once published then other people will be able to see your book in <your_name>.github.io/<your_repo> in a few minutes.

### Checking your Installation

- Check the following Q&A feature works.
- Check the following interactive code runs (it uses Rust Playgound)

#### Q&A and Interactive Code Example Feature

When viewing the book, make sure this feature is working so that you can use the interactive editor to run Rust code while reading the book.
Not only can you run code, but you can edit and make changes (if the block is marked editable like the following Rust code block)

Here is an example of embedding a question and answer code block into the book which the reader can edit and run using *rust playground*:
If you see a '?Q' and '?A' then you have not installed *mdbook-plus* (see above)

?Q (<-- click on the arrow to reveal the answer) What will happen when you run this code?

```rust,editable
#fn main(){
let _x:Option<u32> = None.unwrap();
#}
```

?A
You program panics, because you cannot unwrap a None.
It is best not to use `unwrap` unless you know there is no possibility of failure.

?E

## Editing the Book

Most of the work spent on the book is writing the scripts and tools
for generating the layout of book as opposed to the contents.

This book is generated using a gitbook-like clone called *mdbook*. The book is stored in github and so the book serves the additional purpose of documenting how to create a similar book.

I have spent time on adding logic to make editing the book and producing
content like question and answer blocks easier to write in pseudo-markdown.

Included are commands and instructions for creating a new empty book and how to go about publishing the book on github.

In addition to `mdbook` there some add-ons to mdbook stored in this repo. There is a preprocessor that makes it easy to generate an interactive quiz.

TODO: find out how to trigger hot page reload in the browser when I am editing.

## Create a New Book

Clone this repo and empty out the `md` directory leaving just the `README.md` and `SUMMARY.md`. Also you can remove the `src` directory.
TODO: write a helper script to do this.
