# [100 Doors](http://rosettacode.org/wiki/100_doors#Rust)

This my first foray into writing a simple algorithm in Rust.

## Lessons Learned

1. Don't use `vec!` for a fixed size list (array)
    ```let mut doors = vec![false; 100];```
    ```let mut doors = [false; 100];```
1. Translate a bool into a string - nice!:
    a. println!("{}", if d {"open"} else {"closed"});
1. `for (i, &is_open) in door_open.iter().enumerate()`
    a. Maybe this is faster/more idiomatic than my approach - but less readable? Based on comments perhaps this approach is faster due to printing being a bottleneck. I don't understand.
1. Rust is (at least) two languages bundled into one:
    a. A procedural and a declarative (functional) language
    b. The procedural language is easier to understand for this problem.
    c. The procedural language requires you become familiar with a limited set of verbs: let, for, if, while etc.
    d. The declarative language requires you become familiar with many more verbs: filter, iterate, enumerate, map, last, any, find, skip, nth, take, take_while, etc.
    e. The magic of Rust is that you can get similar (sometimes better) performance in Declarative versus Procedural.
    f. To be a good Rust programmer, you should become familiar with both styles of programming. In the longer term, it is probably better to prefer the Declarative style.
    g. In many cases, parallelization can be trivially achieved in the Declarative style.


## My Soution

First roseatta code problem attempt in Rust (not looking at solution)...

```rust
    let mut doors = vec![false; 100];
    for i in 0..100 {
        let mut d = i;
        while d < 100 {
            doors[d] = !doors[d];
            d += i + 1;
        }
    }
    for d in 0..100 {
        println!(" {}/{}", d + 1, doors[d]);
    }
```

## Official Solution

### Procedural

```rust
    let mut door_open = [false; 100];
    for pass in 1..101 {
        let mut door = pass;
        while door <= 100 {
            door_open[door - 1] = !door_open[door - 1];
            door += pass;
        }
    }
    for (i, &is_open) in door_open.iter().enumerate() {
        println!("Door {} is {}.", i + 1, if is_open {"open"} else {"closed"});
    }
```

- 100 bools initialized to false
- for 100 passes 
  - walk until past 100 doors 
    - start at door# = pass#
    - toggle state of door
    - skip forward pass# number of doors

### Declarative

```rust
    let doors = vec![false; 100]
       .iter_mut()
       .enumerate()
       .map(|(door, door_state)|
         (1..100).into_iter().filter(|pass| door % pass == 0)
         .map(|_| { *door_state = !*door_state; *door_state })
         .last()
         .unwrap()
        )
        .collect::<Vec<_>>();

    println!("{:?}", doors);
```

- 100 bools initialized to false
- mutable iterator
- enumerate to give (index, value) (i.e door, door_state)
  - iterate 100 times for each (door, door_state)
    - skip unless this pass includes this door state
      - (door number / pass#) divides evenly
    - toggle the door state (boolean)
    - this produces a boolean list for this door
      - (100 long for door #1; 1 long for door #100)
  - take the last entry in this list
    -  this is the final state for the door
  - because lists can be empty and there may not be a last element, so last() needs to return an Option\<T\> not a T.
  - unwrap() the Option\<T\> into a T
    - note: you are guaranteed to have at least one element becuase you visited every door, so the unwrap cannot panic.
- collect the results for all doors
