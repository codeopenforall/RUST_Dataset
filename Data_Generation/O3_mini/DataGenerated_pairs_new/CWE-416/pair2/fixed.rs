//////////////////////////////
// Corrected Code Example
//////////////////////////////
use std::boxed::Box;

struct Info {
    value: i32,
}

impl Info {
    // The safe version of the crunch method retains proper ownership.
    // Memory is accessed through safe references, ensuring that it is not freed prematurely.
    pub fn crunch() -> i32 {
        // Allocate a boxed instance of Info.
        let original = Box::new(Info { value: 42 });
        // Safely borrow the value without relinquishing ownership.
        let ret = original.value;
        ret
    }
}

fn main() {
    // Call the safe crunch function.
    let res = Info::crunch();
    println!("Result: {}", res);
}