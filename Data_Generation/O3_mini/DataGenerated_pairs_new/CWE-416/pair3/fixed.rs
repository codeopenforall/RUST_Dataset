use std::sync::Arc;
use std::thread;

/// A simple structure holding an integer value.
struct Data {
    value: u32,
}

/// A container that safely manages the Data using reference counting.
struct Holder {
    data: Arc<Data>,
}

impl Holder {
    /// Allocates a new instance wrapped in an Arc.
    fn new(val: u32) -> Self {
        Self { data: Arc::new(Data { value: val }) }
    }

    /// Reads the value in a safe manner.
    fn read(&self) -> u32 {
        self.data.value
    }
}

/// Runs the core operation in a safe way.
/// It creates a Holder with an Arc so that the Data is kept alive while accessed by the thread.
pub fn run_op() -> u32 {
    let holder = Holder::new(100);
    let data_clone = holder.data.clone();
    // Spawn a thread which accesses the data via the Arc.
    let handle = thread::spawn(move || {
        data_clone.value
    });
    let result = handle.join().unwrap();
    result
}

fn main() {
    let res = run_op();
    println!("Result: {}", res);
}