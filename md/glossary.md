# Glossary

### [Rustonomicom](https://doc.rust-lang.org/nomicon/index.html)

A book about writing unsafe code and the inner guts of Rust. |

### iterator adapters

Functions which take an Iterator and return another Iterator are often called 'iterator adapters', as they're a form of the 'adapter pattern'.

### iterator
Iterator is a trait with method `next` which returns `Some(Item)` as it iterates thru Items and returns `None` when there are no more items.

### method
Methods are functions attached to objects which have access to the data of the object and its other methods via the ```self``` keyword and defined in an ```impl``` block.

A method may either be a ```static``` or and ```instance``` method.

### static method
A static method does not have ```self``` as the first parameter.

### instance method
An instance method has ```self``` as the first parameter.

### external trait/external type
A trait or type that is not defined in the current module. You cannot implement an external trait on an external type but you can implement an external trait on a type you defined and someone
else can implement a trait defined in your module in one of their modules on their own type.

## cargo

Rust's package manager and build system. `cargo --version` `cargo build` `cargo run` `cargo install` `cargo check` `cargo new {name}`

## rustup

The command to upgrade your Rust compiler: `rustup update`

## rustc

The rust compiler, `rustc --version`

## Fully Qualified Method Calls

```
trait Foo {
    fn hi(&self);
}
trait Bar {
    fn hi(&self);
}
impl Foo for String {
    fn hi(&self) {
        println!("Calling Foo::hi for String");
    };
}
impl Bar for String {
    fn hi(&self) {
        println!("Calling Bar::hi for String");
    };
}
impl Foo for u32 {
    fn hi(&self) {
        println!("Calling Foo::hi for u32");
    };
}
```<str as ToString>::to_string("hi)``` is the fully qualified method call for qualified method call ```ToString::to_string("hi")``` and the 



