////////////////////////////////////////////////////////////////
// Corrected example eliminating the double release vulnerability.
struct Item {
    // Holds a raw pointer to an integer allocated on the heap.
    data: *mut i32,
}

impl Item {
    // Allocates a new Item with the integer value 100.
    fn new() -> Self {
        let boxed = Box::new(100);
        Item { data: Box::into_raw(boxed) }
    }
    // Returns the integer value stored in the allocated memory.
    fn value(&self) -> i32 {
        unsafe { *self.data }
    }
    // Constructs an Item from an integer value by allocating a new heap object.
    fn from_value(val: i32) -> Self {
        let boxed = Box::new(val);
        Item { data: Box::into_raw(boxed) }
    }
}

impl Clone for Item {
    // Corrected clone: Creates a deep copy by allocating new memory and
    // copying the value. This ensures each Item has its own allocation.
    fn clone(&self) -> Self {
        let val = self.value();
        Item::from_value(val)
    }
}

impl Drop for Item {
    // On drop, the heap memory is safely freed.
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.data));
        }
    }
}

// Function that runs the application logic.
fn run_app() {
    let first = Item::new();
    let second = first.clone();
    // The two Items have independent copies of the allocated memory,
    // and dropping them will free each allocation exactly once.
    println!("Values: {} and {}", first.value(), second.value());
}

fn main() {
    run_app();
}