use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Record {
    value: i32,
}

impl Record {
    fn new(val: i32) -> Self {
        Record { value: val }
    }
}

// In this corrected version, compute() avoids converting the Box into a raw pointer.
// It safely extracts the record's value while the Box is still valid, ensuring correct memory usage.
fn compute() -> i32 {
    // Allocate memory safely with Box.
    let boxed = Box::new(Record::new(42));
    // Extract the value from the allocated Record.
    let value = boxed.value;

    // Concurrent thread is still spawned to simulate application workload,
    // but it does not interfere with our data since we have already copied it.
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(10));
        let _temp = Box::new(Record::new(100));
    });
    handle.join().unwrap();

    // Return the safely captured value.
    value
}

fn main() {
    let result = compute();
    println!("Result: {}", result);
}