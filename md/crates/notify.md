# notify

## [How do I recursively watch file changes in Rust?](https://stackoverflow.com/questions/55440289/how-do-i-recursively-watch-file-changes-in-rust)

This example watches a directory for changes and upon a change an event is received

```rust

use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    // Create a channel to receive the events.
    let (sender, receiver) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    // The Duration is how long after the event that you will
    // receive the notification. If this is too short then you
    // may end up taking an action before the event is complete.
    // For example, if a file is written to you may try something
    // before it is finished writing.
    let mut watcher = watcher(sender, Duration::from_secs(2)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch("/path/to/watch", RecursiveMode::Recursive).unwrap();

    loop {
        match receiver.recv() {
           Ok(event) => println!("{:?}", event),
           Err(e) => println!("watch error: {:?}", e),
        }
    }
}
```