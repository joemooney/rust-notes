mod function_pointer;
mod iterators; // import iterators/mod.rs

// use crate::foo::bar;    // relative path
// use foo::bar;           // absolute path

use function_pointer::demo_function_pointer;
use iterators::iter_example1::demo_iter_example1; // relative path // relative path

fn main() {
    demo_iter_example1();
    demo_function_pointer();
}
