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

    // CORRECTED: This method safely checks the bounds before accessing the vector.
    fn read_item(&self, index: usize) -> Option<u32> {
        self.items.get(index).copied()
    }
}

fn main() {
    let container = Arc::new(DataContainer::new(10));
    let shared = container.clone();

    let handle = thread::spawn(move || {
        // Using an index equal to the vector length.
        // The safe get method will return None for an out-of-bound index.
        let result = shared.read_item(10);
        match result {
            Some(val) => println!("Thread read value: {}", val),
            None => println!("Out-of-bound access prevented"),
        }
    });

    let _ = handle.join();
}