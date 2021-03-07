# Quiz

## Option

_________________________________________________________

?Q Can you implement a trait on a type you did not define

?A {green}Yes!{/green} But you cannot implement an external trait (one that you did not define) on an external type (one that you did not define).
?E

_________________________________________________________

?Q Including code
?A

[one](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e9cc8920b712c93f0c09c1fe56f831b8)


[two](https://gist.github.com/e9cc8920b712c93f0c09c1fe56f831b8)


Here is a anchor1:
```rust,no_run,noplayground
{{#include ../src/misc/anchors.rs:anchor1}}
```

Here is a anchor2:
```rust,no_run,noplayground
{{#include ../src/misc/anchors.rs:anchor2}}
```

This is the anchor3.
```rust,no_run,noplayground
{{#include ../src/misc/anchors.rs:anchor3}}
```

<script src="https://gist.github.com/joemooney/815626d5cd658d191d10ae48d4803047.js" data-gist-hide-line-numbers="true"></script>

?E


?Q How to read a line from stdin?

?A

```rust,editable
use std::io;
use std::io::Write;
let mut s = String::new();
print!("Please enter something: ");
io::stdin().read_line(&mut s).expect("Failed to read line from stdin");
println!("You entered: {}", s);
```

?E
_________________________________________________________

?Q What happens here upon {code}unwrap{/code}?

```rust,editable
let x:Option<u32> = None.unwrap();
```

?A {red}The program panics!{/red} You cannot unwrap() a None, use `unwrap_or(...)` instead
?E

_________________________________________________________

?Q What is the difference between <code>unwrap_or</code> and `unwrap_or_else`?

?A `unwrap_or_else` is lazy, `unwrap_or` is eager.
So unwrap_or_else is generally for passing a closure which is evaluated only when needed and unwrap_or is for an existing literal value known at the time of execution. `unwrap_or_else` expects a `Fn<()>` so you cannot just provide a literal or a simple expression.
`unwrap_or` on the otherhand will not accept a closure.

```rust,editable
fn foo(k: u32) -> u32 {
    println!("eager ran");
    2 * k
}
assert_eq!(Some("car").unwrap_or("bike"), "car");
assert_eq!(None.unwrap_or("bike"), "bike");
assert_eq!(None.unwrap_or_else(||{ "bike" }), "bike");
let k = 10;
assert_eq!(Some(4).unwrap_or(foo(k)), 4);
assert_eq!(Some(4).unwrap_or_else(|| {println!("lazy did not run"); 2 * k}), 4);
assert_eq!(None.unwrap_or_else(|| {println!("lazy ran"); 2 * k}), 20);
println!("it ran");
```

?E

?Q What is the another way to think of `&mut x`?
?A Ryan Levick says `&mut x` is more correctly viewed as **exclusive** reference, **not** a mutable reference.
?E