# Miscellaneous

```rustup update``` to update your version of Rust.
```cargo check``` to see if your crates are up-to-date.

### Read a line of input
```
  use std::io;
  let mut ans = String::new();
  print!("Enter answer: ");
  io::stdin().read_line(&mut ans)
  .expect("Failed to read input");
```

```cargo doc --open``` opens the documentation for all your dependencies.

*Haskell Curry* was an American mathematician and logician. 

### Handling error cases with a ```match``` expression

```
let ans: u32 = match ans.trim().parse() { 
    Ok(num) => num,
    Err(_) => 0,
}
```

### 