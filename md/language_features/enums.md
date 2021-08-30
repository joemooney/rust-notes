# Enums

?Q Can an Enum have methods?

?A Yes

```rust,editable
enum Pill {
   Blue,
   Red,
}
impl Pill {
   fn take(&self) {
   	match self {
         Pill::Blue => println!("Welcome to the Matrix!"),
         Pill::Red => println!("You are awake, welcome to your nightmare!"),
	}
   	
   }
}
fn main() {
   let c = Pill::Red;
   c.take();
}
```

?E
