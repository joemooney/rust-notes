# Quiz

## Option

_________________________________________________________

?Q Can you implement a trait on a type you did not define

?A {green}Yes!{/green} But you cannot implement an external trait (one that you did not define) on an external type (one that you did not define).
?E

_________________________________________________________

[one](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e9cc8920b712c93f0c09c1fe56f831b8)


[two](https://gist.github.com/e9cc8920b712c93f0c09c1fe56f831b8)

[three](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&code=%23!%5Ballow(unused)%5D%0Afn%20main()%20%7B%0Ause%20std%3A%3Aio%3A%3A%7Bself%2C%20Write%7D%3B%0A%0Aprint!(%22this%20%22)%3B%0Aprint!(%22will%20%22)%3B%0Aprint!(%22be%20%22)%3B%0Aprint!(%22on%20%22)%3B%0Aprint!(%22the%20%22)%3B%0Aprint!(%22same%20%22)%3B%0Aprint!(%22line%20%22)%3B%0A%0Aio%3A%3Astdout().flush().unwrap()%3B%0A%0Aprint!(%22this%20string%20has%20a%20newline%2C%20why%20not%20choose%20println!%20instead%3F%5Cn%22)%3B%0A%0Aio%3A%3Astdout().flush().unwrap()%3B%0A%7D)

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
So unwrap_or_else is generally for passing a closure which is evaluated only when needed and unwrap_or is for an existing literal value known at the time of execution.

```rust,editable
assert_eq!(Some("car").unwrap_or("bike"), "car");
assert_eq!(None.unwrap_or("bike"), "bike");
assert_eq!(None.unwrap_or_else("bike"), "bike");
let k = 10;
assert_eq!(Some(4).unwrap_or(|| {println!("eager ran"); 2 * k}), 4);
assert_eq!(Some(4).unwrap_or_else(|| {println!("lazy did not run"); 2 * k}), 4);
assert_eq!(None.unwrap_or_else(|| {println!("lazy ran"); 2 * k}), 20);
println!("it ran");
```

?E
