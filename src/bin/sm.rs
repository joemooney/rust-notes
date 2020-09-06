use std::fmt;
use std::io::{self, Write};
use std::string::ToString;

// This is an instance of an event
struct Event {
    name: String,
}

// This is a type of event which a state machine may expect
struct EventType {
    name: String,
}

// This is a type of event which a state machine may expect
struct StateMachine {
    name: String,
    states: HashMap<String, State>
}

impl StateMachine {
    fn new(name: String) -> StateMachine {
        StateMachine {
            name
        }
    }
}

// Similarly, implement `Display` for `Point2D`
impl fmt::Display for StateMachine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StateMachine: {}\nNumber of States: {}", self.name, self.states.len())
    }
}

fn main() {
    let s = StateMachine::new("s1");
    println!("{}", &s);
    println!("{:?}", &s);
}
