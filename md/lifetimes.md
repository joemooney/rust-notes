# Lifetimes

As a beginner one approach is to avoid writing your program with any explicit lifetimes.
The Rust language is difficult enough without learning these at the beginning.
If you do so then you will learn a lot about why lifetimes are needed in the first place because you will be forced to write your code in  a way that avoids them which is an interesting exercise.
It is easy to wade into lifetimes from the outset and become bogged down.

Read this:
https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md

## Create a `'static` ref at Run-Time


```
use rand;

// generate random 'static str refs at run-time
fn rand_str_generator() -> &'static str {
    let rand_string = rand::random::<u64>().to_string();
    Box::leak(rand_string.into_boxed_str())
}
```

