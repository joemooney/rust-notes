fn foo(x: Option<i32>) {
    match x {
        Some(n) if n >  0 => println!("Some(Non-negative)"),
        Some(n) if n <  0 => println!("Some(Negative)"),
        Some(_)           => unreachable!(), // compile error if commented out
        None              => println!("None")
    }
}
fn main() {
   foo(Some(1));
   foo(Some(-1));
   foo(None);
   foo(Some(0));
}