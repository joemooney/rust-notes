# 15 Puzzle Game


{pseudo}
{pseudo-text}
Define enums for Action, Direction, and Cell types
Define a Board type
Define an EMPTY constant
Define a struct for the P15 game

    - Constructor
    - fn is_valid(mut board: Board) -> bool {
    - find empty cell: return the location of the empty cell
    - get_moves return a hash map of direction: cell
    - play given a Direction
    - fn is_complete(&self) -> bool {
- Implement Display trait for P15
    - write fmt function
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

Implement user interface for function to ask user to
    Move a cell up/down/left/right Or Quit
    Use get_move to see what Cell can move in what direction
    Perform the move

{pseudo-code}

```rust,editable
extern crate rand;

use std::collections::HashMap;
use std::fmt;
 
use rand::Rng;
use rand::seq::SliceRandom;
 
#[derive(Copy, Clone, PartialEq, Debug)]
enum Cell {
    Card(usize),
    Empty,
}
 
#[derive(Eq, PartialEq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
 
enum Action {
    Move(Direction),
    Quit,
}
 
type Board = [Cell; 16];
const EMPTY: Board = [Cell::Empty; 16];
 
struct P15 {
    board: Board,
}
 
impl P15 {
    fn new() -> Self {
        let mut board = EMPTY;
        for (i, cell) in board.iter_mut().enumerate().skip(1) {
            *cell = Cell::Card(i);
        }
 
        let mut rng = rand::thread_rng();
 
        board.shuffle(&mut rng);
        if !Self::is_valid(board) {
            // random swap
            let i = rng.gen_range(0, 16);
            let mut j = rng.gen_range(0, 16);
            while j == i {
                j = rng.gen_range(0, 16);
            }
            board.swap(i, j);
        }
 
        Self { board }
    }
 
    fn is_valid(mut board: Board) -> bool {
        // TODO: optimize
        let mut permutations = 0;
 
        let pos = board.iter().position(|&cell| cell == Cell::Empty).unwrap();
 
        if pos != 15 {
            board.swap(pos, 15);
            permutations += 1;
        }
 
        for i in 1..16 {
            let pos = board
                .iter()
                .position(|&cell| match cell {
                    Cell::Card(value) if value == i => true,
                    _ => false,
                })
                .unwrap();
 
            if pos + 1 != i {
                board.swap(pos, i - 1);
                permutations += 1;
            }
        }
 
        permutations % 2 == 0
    }
 
    fn get_empty_position(&self) -> usize {
        self.board.iter().position(|&c| c == Cell::Empty).unwrap()
    }
 
    fn get_moves(&self) -> HashMap<Direction, Cell> {
        let mut moves = HashMap::new();
        let i = self.get_empty_position();
 
        if i > 3 {
            moves.insert(Direction::Up, self.board[i - 4]);
        }
        if i % 4 != 0 {
            moves.insert(Direction::Left, self.board[i - 1]);
        }
        if i < 12 {
            moves.insert(Direction::Down, self.board[i + 4]);
        }
        if i % 4 != 3 {
            moves.insert(Direction::Right, self.board[i + 1]);
        }
        moves
    }
 
    fn play(&mut self, direction: &Direction) {
        let i = self.get_empty_position();
        // This is safe because `ask_action` only returns legal moves
        match *direction {
            Direction::Up => self.board.swap(i, i - 4),
            Direction::Left => self.board.swap(i, i - 1),
            Direction::Right => self.board.swap(i, i + 1),
            Direction::Down => self.board.swap(i, i + 4),
        };
    }
 
    fn is_complete(&self) -> bool {
        self.board.iter().enumerate().all(|(i, &cell)| match cell {
            Cell::Card(value) => value == i + 1,
            Cell::Empty => i == 15,
        })
    }
}
 
impl fmt::Display for P15 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        r#try!(write!(f, "+----+----+----+----+\n"));
        for (i, &cell) in self.board.iter().enumerate() {
            match cell {
                Cell::Card(value) => r#try!(write!(f, "| {:2} ", value)),
                Cell::Empty => r#try!(write!(f, "|    ")),
            }
 
            if i % 4 == 3 {
                r#try!(write!(f, "|\n"));
                r#try!(write!(f, "+----+----+----+----+\n"));
            }
        }
        Ok(())
    }
}
 
fn main() {
    let mut p15 = P15::new();
 
    for turns in 1.. {
        println!("{}", p15);
        match ask_action(&p15.get_moves()) {
            Action::Move(direction) => {
                p15.play(&direction);
            }
            Action::Quit => {
                println!("Bye !");
                break;
            }
        }
 
        if p15.is_complete() {
            println!("Well done ! You won in {} turns", turns);
            break;
        }
    }
}
 
fn ask_action(moves: &HashMap<Direction, Cell>) -> Action {
    use std::io::{self, Write};
    use Action::*;
    use Direction::*;
 
    println!("Possible moves:");
 
    if let Some(&Cell::Card(value)) = moves.get(&Up) {
        println!("\tU) {}", value);
    }
    if let Some(&Cell::Card(value)) = moves.get(&Left) {
        println!("\tL) {}", value);
    }
    if let Some(&Cell::Card(value)) = moves.get(&Right) {
        println!("\tR) {}", value);
    }
    if let Some(&Cell::Card(value)) = moves.get(&Down) {
        println!("\tD) {}", value);
    }
    println!("\tQ) Quit");
    print!("Choose your move : ");
    io::stdout().flush().unwrap();
 
    let mut action = String::new();
    io::stdin().read_line(&mut action).expect("read error");
    match action.to_uppercase().trim() {
        "U" if moves.contains_key(&Up) => Move(Up),
        "L" if moves.contains_key(&Left) => Move(Left),
        "R" if moves.contains_key(&Right) => Move(Right),
        "D" if moves.contains_key(&Down) => Move(Down),
        "Q" => Quit,
        _ => {
            println!("Unknown action: {}", action);
            ask_action(moves)
        }
    }
}
```

{/pseudo}