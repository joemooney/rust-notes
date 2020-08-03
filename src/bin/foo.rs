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
    let s = String::new();
    let u = 32;
    s.hi();
    Boo::hi(&u);
    "hhh".hi(); // --> error  not a method of str
    "hhh".to_string().hi();
}
