use std::fmt;

struct Int {
    i: usize,
}

impl Int {
    fn add(&self, y: &Int) -> Int {
        Self { i: self.i + y.i }
    }
    fn incr(&mut self) {
        self.i += 1;
    }
}

impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.i)    
    }
}


fn increment_by(x: Int) -> impl Fn(&mut Int) -> &Int {
   move |y| { x.add(y); y.incr(); y } 
}

fn run_fn<A, R>(x: fn(A)->R, a: A) -> R {
    x(a)
}

fn bool_to_int(x: bool) -> u32 {
    if x { 1 } else { 0 }
}

fn main() {
    let five = Int { i: 5 };
    let mut four = Int { i: 4 };
    let incr5 = increment_by(five);
    println!("4 + 5 = {}", incr5(&mut four));
    println!("4 + 5 = {}", incr5(&mut four));

    println!("{}->{}", true, run_fn(bool_to_int, true));
    println!("{}->{}", false, run_fn(bool_to_int, false));

}