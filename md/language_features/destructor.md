# Destructors

```rust,editable
struct PrintOnDrop(&'static str);

impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

let mut overwritten = PrintOnDrop("drops when overwritten");
letoverwritten = PrintOnDrop("drops when scope ends");
```