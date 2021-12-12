trait Named {
  fn name(&self) -> &str;
}

struct Cat {
}
impl Named for Cat {
  fn name(&self) -> &str {
    "Cat"
  }
}

struct Dog {
}
impl Named for Dog {
  fn name(&self) -> &str {
    "Dog"
  }
}

fn foo(named: &dyn Named) {
   println!("{}", named.name());
}

fn main() {
   let pluto = Dog{};
   let garfield = Cat{};
   foo(&pluto);
   foo(&garfield);
}
