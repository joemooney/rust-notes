# Trait Objects

See Also: [Traits](./traits.md)

A function that has an argument of `&dyn T` is a trait object.
Only [object-safe traits](./traits.md) are candidates - this eliminates Traits that have any functions with generic arguments or return types.

## Why use trait objects? 
- Trait objects allow you have collections of objects that implement the same trait but are of different types.
- Smaller binaries (polymorphic dynamic dispatch functions instead of monomorphic static dispatch generic functions)
- Apparently, the additional runtime overhead is small.
- Increasing your use of Traits is good practice.

## Why not use trait objects? 
- Trait objects result in slower binaries.
- Compiler cannot perform inlining optimizations, so if your function is small and frequently called this may not be preferable.

```rust
// https://stevedonovan.github.io/rustifications/2018/09/08/common-rust-traits.html

use std::string::ToString;

// Note the signatures are different but the implementations are the same

// Using an Object Reference (not Trait Object), monomorphic - generic function
fn to_string1<T: ToString> (item: &T) -> String {
    item.to_string()
}
// Using a Trait Object, polymorphic - dynamic dispatch
fn to_string2(item: &dyn ToString) -> String {
    item.to_string()
}

// From the calling side there is no difference, but the code generated is different
println!("{}", to_string1(&42));  // uses a one version of function to_string1 expanded for &u32
println!("{}", to_string1(&"hello"));  // uses a different version of function to_string1 expanded for &str
println!("{}", to_string2(&42));  // uses the only version of function to_string2 passing as a trait object &u32
println!("{}", to_string2(&"hello"));  // uses the only version of function to_string2 passing as a trait object &str

```

Another example of using a Vec of trait objects looks pretty slick.

```rust
// https://dev.to/magnusstrale/rust-trait-objects-in-a-vector-non-trivial-4co5

use std::f32::consts::PI;

#[derive(PartialEq)]
struct Circle {
    radius: u32,
}

struct Square {
    side: u32,
}

trait Shape: Any {
    fn box_eq(&self, other: &dyn Any) -> bool;
    fn as_any(&self) -> &dyn Any;    
    fn area(&self) -> u32;
}

impl Shape for Square {
    // boilerplate, same for all impl of Shape
    fn as_any(&self) -> &dyn Any {
        self
    }
    // boilerplate, same for all impl of Shape
    fn box_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }
    fn area(&self) -> u32 {
        self.size * self.size
    }
}

impl Shape for Circle {
    // boilerplate, same for all impl of Shape
    fn as_any(&self) -> &dyn Any {
        self
    }
    // boilerplate, same for all impl of Shape
    fn box_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }
    fn area(&self) -> u32 {
        self.radius.pow(2) * PI
    }
}

impl PartialEq for Box<dyn Shape> {
    fn eq(&self, other: &Box<dyn Shape>) -> bool {
        self.box_eq(other.as_any())
    }
}

fn do_stuff(objects: Vec<Box<dyn Shape>>) {
    let obj1 = &objects[0];
    let obj2 = &objects[1];
    if obj1 == obj2 { println!("Equal"); }
}

```

Here we see a Vec of a Trait type which can contain different types of concrete objects which can be differentiated.


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

## Excercise

Write a function taking a Trait Object for Trait Foo.
Foo trait has a function foo with just `&self` as argument with no return - it would simply print "I am a Cat" where Cat is a struct that implements the Trait or "I am a Dog" for a Dog.

