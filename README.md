# My Rust Notes

A place for me to store notes about programming in Rust. 
I am storing these notes using gitbook-like clone *mdbook*.
This repo serves the dual purpose of documenting my experience
and documenting how I went about documenting.

As a byproduct, this repo will have commands and instructions for
creating a new empty book.

The contents of the "book" are stored on GitHub.
It is a "book" because it uses the mdbook crate.
Beyond mdbook I have added in some bells and whistles such as a preprocessor that makes it easy to generate an interactive quiz.

View the book here:  [https://joemooney.github.io/rust-notes/](https://joemooney.github.io/rust-notes/)

## Goals

- document words of wisdom I have learned about Rust.
- byprodut: create a simple wrapper crate for mdbook to create new books.

## Build

- Build/View the book locally: ```mdbook build --open```
- Rebuild/View loop: ```mdbook watch --open```
- Publish: ./publish.sh:  this shell script will generate and will publish changes to the book to github.

## Install/Bootstrap

- GitHub source code for book: [https://github.com/joemooney/rust-notes/](https://github.com/joemooney/rust-notes/)
-- View the book locally: [file:///home/jpm/rust/rust-notes/book/index.html](file:///home/jpm/rust/rust-notes/book/index.html)

Once you have opened the book the rest of the instructions for
setting up a new book, editing, publishing etc. are within the book itself.

```rust,editable
let x:Option<u32> = None.unwrap();
```