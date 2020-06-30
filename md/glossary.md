# Glossary

[Rustonomicom](https://doc.rust-lang.org/nomicon/index.html)
    A book about writing unsafe code and the inner guts of Rust. |

Functions which take an Iterator and return another Iterator are often called 'iterator adapters', as they're a form of the 'adapter pattern'.

Iterator is a trait with method `next` which returns `Some(Item)` as it iterates thru Items and returns `None` when there are no more items.

