# Dropping/Destructors

```rust,editable
struct PrintOnDrop(&'static str, u32);

impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

fn main() {
    let y = PrintOnDrop("dropping y will happening at exit of main", 1);
    println!("<<<<<<<<<before block>>>>>>>>>>>>");
    {
       let mut x = PrintOnDrop("x=2 shadowed on next line, but not dropped", 2);
       let z = PrintOnDrop("z=2 manually dropped", 2);
       println!("z={}", z.1);
       drop(z); // we can manually drop
       let x = PrintOnDrop("x=3 drops when scope ends", 3);
       println!("x={}", x.1)
       // x=3 dropped
       // x=2 dropped  (reverse order of creation)
    }
    println!("<<<<<<<<<<<after block>>>>>>>>>>>>");
    // y=1 dropped
}
```
