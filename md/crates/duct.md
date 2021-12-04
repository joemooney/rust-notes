# duct

```rust,editable
# fn main() {
   let output_obj = cmd!("sh", "-c", "echo foo >&2").stderr_capture().run().unwrap();
   println!("stdout<<{}>>", &output_obj.stdout[..]);
   println!("stderr<<{}>>", &output_obj.stderr[..]);
   assert_eq!(&b"foo\n"[..], &output_obj.stderr[..]);
}
```