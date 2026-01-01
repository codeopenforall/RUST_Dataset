/* 
   The corrected version addresses the use‐after‐free vulnerability by employing a reference‐counted pointer.
   It replaces the raw pointer with a shared ownership pointer (Rc). The Rc type ensures the data remains
   allocated until all references are dropped, thereby preventing dangling pointer access.
*/

use std::rc::Rc;

#[derive(Debug)]
struct Data {
    value: i32,
}

trait Action {
    fn execute(&self) -> i32;
}

struct Handler {
    data: Rc<Data>,
}

impl Action for Handler {
    fn execute(&self) -> i32 {
        // Safely access the data through shared ownership.
        self.data.value
    }
}

pub fn compute() -> i32 {
    // Allocate data wrapped in a reference-counted pointer.
    let data_rc = Rc::new(Data { value: 42 });
    let handler = Handler { data: Rc::clone(&data_rc) };

    // Both data_rc and handler.data share the same allocation.
    // The data is freed only when the last Rc goes out of scope, ensuring safety.
    let result = handler.execute();
    result
}

fn main() {
    let res = compute();
    println!("Computed result: {}", res);
}