use std::io::{self, Write};

struct PrintOnDrop(&'static str);

impl PrintOnDrop {
    fn hi(&self) {
        println!("hi from {}", self.0);
    }
}
impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("{} is dropped!", self.0);
    }
}

trait Foo {
    fn hi(&self);
}
trait Boo {
    fn hi(&self);
}
impl Foo for str {
    fn hi(&self) {
        println!("Foo Hi from str");
    }
}
/*
impl Foo for String {
    fn hi(&self) {
        println!("Foo Hi from String");
    }
}
*/
impl Boo for u32 {
    fn hi(&self) {
        println!("Boo Hi from u32");
    }
}
impl Foo for u32 {
    fn hi(&self) {
        println!("Foo Hi from u32");
    }
}
fn main() {
    let s = PrintOnDrop("s1");
    s.hi();
    let s = PrintOnDrop("s2");
    s.hi();

    let mut s = String::new();
    let u = 32;
    print!("Enter a number: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut s)
        .expect("Could not read from stdin");
    let x: u32 = match s.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    println!("You answered {}", x);
    s.hi();
    Boo::hi(&u);
    "hhh".hi(); // --> error  not a method of str
    "hhh".to_string().hi();
}
