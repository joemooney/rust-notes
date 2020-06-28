use std::fmt;
/*
   https://www.youtube.com/watch?v=pgFWz0jgqMU&t=1026s

   Iterators traverse allow you data structures, consume remote resources with pagination
   Iterator is a Trait so you implement that trait on your struct.

*/

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

// NOTE: #2 We require a matching lifetime for each reference lifetime
struct Iter<'a> {
    // NOTE: #1 Since we want client to retain overship, pass by reference
    //          Each reference requires a lifetime
    vec: &'a [u32],
    index: usize,
    // Note: f is a closure, Fn is a Trait
    // Note: Closures have unkown size
    //       The size of type dyn Fn(u32) -> bool cannot be known
    //       at compilation time
    // Note: You need to Box any member that has unknown size
    // Note: Trait objects without an explicit `dyn` are deprecated
    //       so we add dyn before Fn
    f: Box<dyn Fn(u32) -> bool>,
}

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

impl<'a> Iter<'a> {
    // NOTE: #4 We pass in the lifetime reference here
    fn new1<F>(vec: &'a [u32], f: F) -> Self
    where
        F: Fn(u32) -> bool + 'static,
    {
        Self {
            vec,
            index: 0,
            f: Box::new(f),
        } // NOTE: don't need vec: vec is name is the same
    }
    fn new2(vec: &'a [u32], f: Box<dyn Fn(u32) -> bool>) -> Self {
        Self {
            vec,
            index: 0,
            f: Box::new(f),
        } // NOTE: don't need vec: vec is name is the same
    }
}

// NOTE: #3 Since struct Iter has a lifetime we need to match it
impl<'a> Iter<'a> {
    // NOTE: #4 We pass in the lifetime reference here
}

// NOTE: #5 Since struct Iter has a lifetime we need to match it
impl<'a> Iterator for Iter<'a> {
    type Item = u32;

    // NOTE: Here we could write -> Option<u32>
    fn next(&mut self) -> Option<Self::Item> {
        while self.index + 1 < self.vec.len() {
            let prev = self.vec[self.index];
            self.index += 1;
            if (*self.f)(prev) {
                return Some(self.vec[self.index]);
            }
        }
        None
    }
}

fn condition(num: u32) -> bool {
    num % 2 == 1
}

fn main() {
    println!("Hello, world!");
    let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];

    let foo = Foo::new1(|x| x * 2 == 4);
    println!("foo:{}", foo);
    let foo = Foo::new1(|x| x * 2 == 5);
    println!("foo:{}", foo);

    // Syntax Note:
    // Box<condition> that would be a type, not an arg
    //  error: comparison operators cannot be chained
    // Box::new(condition) is an arg
    let iter = Iter::new2(&vec, Box::new(|x| x % 3 == 0));
    // let iter = Iter::new(&vec, Box::new(condition));
    println!("{:?}", vec);
    // println!("{:?}", iter.collect());
    for i in iter {
        print!("{}", i);
    }
    println!("");

    let iter = Iter::new1(&vec, |x| x % 3 == 0);
    for i in iter {
        print!("{}", i);
    }
    println!("");

    let iter = Iter::new1(&vec, condition);
    for i in iter {
        print!("{}", i);
    }
    println!("");
}
