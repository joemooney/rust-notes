trait Foo {
    fn hi(&self);
}
trait Bar {
    fn hi(&self);
}
trait Zar {
    fn lo(&self);
}
impl Foo for String {
    fn hi(&self) {
        println!("Calling Foo::hi for String");
    }
}
impl Bar for String {
    fn hi(&self) {
        println!("Calling Bar::hi for String");
    }
}
impl Zar for str {
    fn lo(&self) {
        println!("Calling Bar::lo for str");
    }
}
impl Foo for u32 {
    fn hi(&self) {
        println!("Calling Foo::hi for u32");
    }
}

pub fn demo_methods_example1() {
    let s = String::new();
    s.lo();
    Bar::hi(&s);
    "hhh".lo();
    "hhh".to_string().lo();
}
