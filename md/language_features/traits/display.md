```rust
use std::fmt; // Import `fmt`

#[derive(Debug)]
struct Foo {
    x: u32,
    y: u32,
}

impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
```
