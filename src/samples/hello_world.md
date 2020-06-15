# Hello World

```rust,editable
fn main() {
    print!("Hello World!");
}
```

```rust,editable
fn main() {
    let number = 5;
    print!("{}", number);
}
```

```rust,editable
//TODO: how dow import in an mdbook block of code? use itertools::Itertools;
fn main() {
    let numbers = vec![1,2,3,4,5,6,7,8,9];
    // To use itertools
    // numbers.foreach(|i| print!("{}", i));
    for i in numbers {
        print!("{}", i);
    }
}
```