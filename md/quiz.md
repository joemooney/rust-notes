# Quiz

## Option

_________________________________________________________

?Q Can you implement a trait on a type you did not define

?A {green}Yes! All hail Traits!{/green}
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
So unwrap_or_else is for pasing a closure and unwrap_or is for a exiting value.

```rust,editable
assert_eq!(Some("car").unwrap_or("bike"), "car");
assert_eq!(None.unwrap_or("bike"), "bike");
let k = 10;
assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
assert_eq!(None.unwrap_or_else(|| 2 * k), 20);
```

?E
