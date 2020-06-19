# Quiz

## Option

_________________________________________________________

{question}What happens here upon {code}unwrap{/code}?

```rust,editable
let x:Option<u32> = None.unwrap();
```

{answer}A: The program panics! You cannot unwrap() a None, use `unwrap_or(...)` instead{/question}

_________________________________________________________

{question}What is the difference between ```unwrap_or``` and ```unwrap_or_else```?

A: ```unwrap_or_else``` is lazy, ```unwrap_or``` is eager.

```rust,editable
assert_eq!(Some("car").unwrap_or("bike"), "car");
assert_eq!(None.unwrap_or("bike"), "bike");
let k = 10;
assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
assert_eq!(None.unwrap_or_else(|| 2 * k), 20);
```
