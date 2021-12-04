

/// A function that takes an Option<T> but
/// if just a T is provided then it converts
/// T into an Option<T>

fn some_func<T: Into<Option<u32>>>(num: T){
    let num: Option<u32> = num.into();
    match num {
        Some(val) => { println!("value is {}", val); },
        None => { println!("value is None"); },
    }
}

/* Goal:
#[is_option(num)]
fn orig_some_func(num: T){
    match num {
        Some(val) => { println!("value is {}", val); },
        None => { println!("value is None"); },
    }
}
*/

fn orig_some_func(num: Option<u32>){
    match num {
        Some(val) => { println!("value is {}", val); },
        None => { println!("value is None"); },
    }
}


fn main() {
    orig_some_func(1);
    some_func(1);
    some_func(Some(2));
    some_func(None);
}
