use std::fmt;

/*
A reference has a known size - it is a pointer.
Functions must return a known size of memory.
Functions must return a concrete type - unlike other languages.
Traits are not concrete - their size is unknown.
You cannot return Traits.
You can place Traits in a Box.
A Box is a reference to heap memory.
In Rust the code is explicit if memory is heap or stack.
`dyn` indicates that memory is in the heap.
*/

struct MyObjWithFuncPointer {
    func: Box<dyn Fn(u32) -> bool>,
}

impl fmt::Display for MyObjWithFuncPointer {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "foo.f(2) = {}", (self.func)(2))
    }
}

// type FnType = dyn Fn(u32) -> bool;

impl MyObjWithFuncPointer {
    fn new<F>(func: F) -> Self
    where
        F: Fn(u32) -> bool + 'static,
    {
        Self { func: Box::new(func) }
    }
    fn check(&self, val: u32) -> bool {
        (self.func)(val)
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

fn is_odd(num: u32) -> bool {
    num % 2 == 1
}

fn main() {
    let is_even = |x: u32| x % 2 == 0;
    let odder = MyObjWithFuncPointer::new(is_odd);
    println!("{} {} is odd", 2, odder.check(2));
    println!("{} {} is odd", 3, odder.check(3));
    println!("{} {} is even", 2, is_even(2));
}