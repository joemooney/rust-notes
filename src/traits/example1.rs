trait Run {
    fn age(&self) -> u32;
    fn run(&self) {
        println!("I'm running at {}", self.age());
    }
}
trait Race: Run {
    fn race(&self) {
        println!("I'm racing at {}", self.age() - 5);
    }
}
trait Sprint: Run + Race {
    fn sprint(&self);
}
struct Foo {
    age: u32,
}
impl Race for Foo {}
impl Run for Foo {
    fn age(&self) -> u32 {
        self.age + 10
    }
}
impl Sprint for Foo {
    fn sprint(&self) {
        println!("I'm sprinting at {}", self.age);
    }
}

pub fn eg1() {
    let s = Foo { age: 42 };
    s.run();
    s.race();
    s.sprint();
}
