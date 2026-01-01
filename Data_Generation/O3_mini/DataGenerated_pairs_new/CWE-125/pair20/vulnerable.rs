use std::sync::Arc;
use std::thread;

struct DataContainer {
    items: Vec<u32>,
}

impl DataContainer {
    fn new(count: usize) -> Self {
        DataContainer {
            items: (0..count as u32).collect(),
        }
    }

    // VULNERABLE: This method unsafely reads an element without proper bounds-checking.
    // If index is out-of-bound, undefined behavior will occur.
    fn read_item(&self, index: usize) -> Option<u32> {
        // No bounds check: using unsafe get_unchecked even for an interface returning Option.
        // If index >= self.items.len(), memory beyond the vector will be read.
        unsafe { Some(*self.items.get_unchecked(index)) }
    }
}

fn main() {
    let container = Arc::new(DataContainer::new(10));
    let shared = container.clone();

    let handle = thread::spawn(move || {
        // Triggering condition: using an index equal to the vector length.
        // This should be out-of-bound since valid indices are 0..9.
        let result = shared.read_item(10);
        // The output is nondeterministic due to undefined behavior.
        match result {
            Some(val) => println!("Thread read value: {}", val),
            None => println!("Thread encountered None"),
        }
    });

    let _ = handle.join();
}