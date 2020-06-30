use std::fmt;

/*
A reference has a known size - it is a pointer.
Functions must return a known size of memory.
Functions must return a concrete type - unlike other languages.
Traits are not concrete - their size is unknown.
You cannot return Traits.
Place Traits in a Box.
A Box is a reference to heap memory.
Rust prefers code to be explicit if memory is heap or stack.
dyn indicates that memory is in the heap.
*/

struct Foo {
    f: Box<dyn Fn(u32) -> bool>,
}

impl fmt::Display for Foo {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "foo.f(2) = {}", (self.f)(2))
    }
}

// type FnType = dyn Fn(u32) -> bool;

impl Foo {
    fn new1<F>(f: F) -> Self
    where
        F: Fn(u32) -> bool + 'static,
    {
        Self { f: Box::new(f) }
    }
    /*
       You cannot do this because FnType is unsized since it is
       a Trait Fn and that is unsized. But if you make it a
       generic function then since generic types are sized by
       default you can pass something of the type as an arg.
    */
    // fn new2(f: FnType) -> Self {
    //     Self { f: Box::new(f) }
    // }
    /*
       You cannot do this because FnType is a type alias and
       the where clause required a Trait - it is a "trait bound".
    */
    // fn new2<F>(f: F) -> Self
    // where
    //     F: FnType + 'static,
    // {
    //     Self { f: Box::new(f) }
    // }
}

fn condition(num: u32) -> bool {
    num % 2 == 1
}

pub fn demo_function_pointer() {
    let foo = Foo::new1(|x| x * 2 == 4);
    println!("foo:{}", foo);
    let foo = Foo::new1(|x| x * 2 == 5);
    println!("foo:{}", foo);
}
