# Glossary

### [Rustonomicom](https://doc.rust-lang.org/nomicon/index.html)

A book about writing unsafe code and the inner guts of Rust. |

### iterator adapters

Functions which take an Iterator and return another Iterator are often called 'iterator adapters', as they're a form of the 'adapter pattern'.

### iterator
Iterator is a trait with method `next` which returns `Some(Item)` as it iterates thru Items and returns `None` when there are no more items.


### external trait/external type
A trait or type that is not defined in the current module. You cannot implement an external trait on an external type but you can implement an external trait on a type you defined and someone
else can implement a trait defined in your module in one of their modules on their own type.

