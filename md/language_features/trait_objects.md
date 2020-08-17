# Trait Objects

If you want to pass a variable to a function which may be of different types at run time, then you can use a trait object.
Alternatively, you can use a generic function will generate a different function for each type that calls the function but a trait object will use the same single function with *dynamic dispatch* to make any calls to methods of the trait object in the function.

A *trait object* is a value which we can pass as a parameter to a function and which has as its type a trait as opposed to a concrete type. This means that for all of the trait methods the sizes of the parameters and the return values must be known at compile time. The *trait object* has an unknown type at compile time because the underlying concrete type is unknown. Lots of types can implement a trait. Some of those types may have a size that changes during execution. For example, if a struct has a String or a Vec, the size of the struct will change as the values are changed. Such a type cannot be placed on the stack and as such it cannot be passed or returned from a function.

A trait is either object safe or not. If a trait is not object safe, it cannot be used to make a trait object. So, being *object safe* means that we can create a *trait object* for that Trait.

A trait object must be a reference (or pointer) since it must be a fat pointer containing a pointer to the object data and a pointer to the vtable of the concrete implementation of the functions of the trait.

You have to take the Trait Object by reference or pointer. Whether you use a reference or pointer depends on whether you want to transfer ownership or not.

This will __not compile__ because a closure as an unknown size.
```rust
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
```

This will compile: Vec
```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```


```rust
// https://stevedonovan.github.io/rustifications/2018/09/08/common-rust-traits.html

use std::string::ToString;

// monomorphic - generic
fn to_string1<T: ToString> (item: &T) -> String {
    item.to_string()
}
// polymorphic - dynamic dispatch
fn to_string2(item: &dyn ToString) -> String {
    item.to_string()
}

println!("{}", to_string1(&42));
println!("{}", to_string2(&42));
println!("{}", to_string1(&"hello"));
println!("{}", to_string2(&"hello"));

```

