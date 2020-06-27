[# 100 Prisoners](http://rosettacode.org/wiki/100_prisoners#Rust)

{pseudo}

- 100 prisoners are individually numbered 1 to 100
- A room having a cupboard of 100 opaque drawers numbered 1 to 100, that cannot be seen from outside.
- Cards numbered 1 to 100 are placed randomly, one to a drawer, and the drawers all closed; at the start.
- Prisoners start outside the room

    - They can decide some strategy before any enter the room.
    - Prisoners enter the room one by one, can open a drawer, inspect the card number in the drawer, then close the drawer.
    - A prisoner can open no more than 50 drawers.
    - A prisoner tries to find his own number.
    - A prisoner finding his own number is then held apart from the others.

- If all 100 prisoners find their own numbers then they will all be pardoned. If any don't then all sentences stand. 

{pseudo-text}

1. create boxes vec containing numbers 1 thru 100
    - These numbers are the prison numbers
    - hint: this is a collected range
1. Prisoner guess: given the boxes and a prisoner number
    1. create guesses vec containing numbers 1 thru 100
    2. shuffle guesses
    3. for first 50 guesses
        1. return if any box corresponding to the guess contains a number matching prisoner number
1. Perform trial:
    1. shuffle the boxes
    1. for all prisoners numbers 0 thru 100
        - Prisoner guess
3. Peform 1000 trials
    1. filter successes
       1. Perform trial
    2. count successes

{pseudo-code}

```rust
let boxes = (1u8..101u8).collect();
fn prisoner_guess(boxes: mut &[u8], prisoner_number) -> bool {
    let guesses = {
        boxes.shuffle()
    }
}
```

## Official Solution
```rust
extern crate rand;
 
use rand::prelude::*;
 
// Do a full run of checking boxes in a random order for a single prisoner
fn check_random_boxes(prisoner: u8, boxes: &[u8]) -> bool {
    let checks = {
        let mut b: Vec<u8> = (1u8..=100u8).collect();
        b.shuffle(&mut rand::thread_rng());
        b
    };
    checks.into_iter().take(50).any(|check| boxes[check as usize - 1] == prisoner)
}
 
// Do a full run of checking boxes in the optimized order for a single prisoner
fn check_ordered_boxes(prisoner: u8, boxes: &[u8]) -> bool {
    let mut next_check = prisoner;
    (0..50).any(|_| {
        next_check = boxes[next_check as usize - 1];
        next_check == prisoner
    })
}
 
fn main() {
    let mut boxes: Vec<u8> = (1u8..=100u8).collect();
 
    let trials = 100000;
 
    let ordered_successes = (0..trials).filter(|_| {
        boxes.shuffle(&mut rand::thread_rng());
        (1u8..=100u8).all(|prisoner| check_ordered_boxes(prisoner, &boxes))
    }).count();
 
    let random_successes = (0..trials).filter(|_| {
        boxes.shuffle(&mut rand::thread_rng());
        (1u8..=100u8).all(|prisoner| check_random_boxes(prisoner, &boxes))
    }).count();
 
    println!("{} / {} ({:.02}%) successes in ordered", ordered_successes, trials, ordered_successes as f64 * 100.0 / trials as f64);
    println!("{} / {} ({:.02}%) successes in random", random_successes, trials, random_successes as f64 * 100.0 / trials as f64);
 
}
```
{/pseudo}


