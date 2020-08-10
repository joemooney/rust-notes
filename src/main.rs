mod function_pointer;
mod iterators; // import iterators/mod.rs
mod methods; // import methods/mod.rs
mod traits; // import methods/mod.rs

// use crate::foo::bar;    // relative path
// use foo::bar;           // absolute path

use function_pointer::demo_function_pointer;
use iterators::iter_example1::demo_iter_example1; // relative path // relative path
use methods::methods_example1::demo_methods_example1; // relative path // relative path
use traits::example1::eg1; // relative path // relative path
                           // use traits; // relative path // relative path

fn main() {
    demo_iter_example1();
    demo_function_pointer();
    demo_methods_example1();
    eg1();
}
