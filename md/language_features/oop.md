# Object Orientation

## Inheritance

Favor composition over inheritance.

```rust,editable
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    phone: u64,
}

#[derive(Debug)]
struct Employee {
    person: Person,
    phone: u64,
    id: u32,
}

fn main() {
    let p = Person { name: "John".to_string(), age:33, phone:3334445555 };
    let e = Employee {person:p, phone:1112223333, id:12345};
    println!("{:?}", p);
    println!("{:?}", e);
    println!("{:?}, home phone:{}", e, e.person.phone);
}
```