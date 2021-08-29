# Random Rust Notes

A place to store notes about programming in Rust. 
Stored using a gitbook-like clone called *mdbook*.
This repo serves the dual purpose of documenting my experience
and documenting how I went about documenting.

As a byproduct, this repo will have commands and instructions for creating a new empty book.

The contents of the "book" are stored on GitHub.
It is a "book" because it uses the mdbook crate.
Beyond mdbook there some bells and whistles such as a preprocessor that makes it easy to generate an interactive quiz.

- View the book in github here:  [https://joemooney.github.io/rust-notes/](https://joemooney.github.io/rust-notes/)
- View the book locally: [file:///home/jpm/rust/rust-notes/book/index.html](file:///home/jpm/rust/rust-notes/book/index.html)
- View locally while making updates: ```mdbook watch --open```

## Goals

- document lessons learned in Rust.
- create a simple wrapper crate for mdbook to create new books.

## Build

- Build/View the book locally: ```mdbook build --open```
- Rebuild/View loop: ```mdbook watch --open```
- Publish: ./publish.sh:  this shell script will generate and will publish changes to the book to github.

## Install/Bootstrap

- GitHub source code for book: [https://github.com/joemooney/rust-notes/](https://github.com/joemooney/rust-notes/)

Once opened, the book contains the instructions for
creating a new book, and editing, publishing etc.

## Interactive Code Examples
Here is an example of embedding code into the book which the reader can then edit and run the code:

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

