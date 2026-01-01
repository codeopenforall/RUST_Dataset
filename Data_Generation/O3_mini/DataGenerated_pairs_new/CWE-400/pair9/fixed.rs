use std::thread;

/// A type responsible for processing numerical data.
struct Worker;

impl Worker {
    /// Processes the entire supplied data safely by leveraging Rust’s safe abstractions.
    /// 
    /// This implementation does not impose an arbitrary limit on the number of elements,
    /// thereby preventing unintended denial-of-service due to uncontrolled resource consumption.
    pub fn execute(&self, data: Vec<u32>) -> u32 {
        // Spawn a worker thread to safely compute the sum.
        let handle = thread::spawn(move || {
            // Safe iteration over the vec ensures all elements are processed.
            data.iter().sum()
        });
        handle.join().unwrap()
    }
}

/// Exposed API function for external callers (e.g. test oracle).
/// This function simply delegates to the Worker’s execute() method.
pub fn run_fn(data: Vec<u32>) -> u32 {
    let worker = Worker;
    worker.execute(data)
}

fn main() {
    let worker = Worker;
    // Example input that may come from untrusted sources.
    let input = vec![1u32; 150]; // 150 ones; the expected sum is 150.
    let res = worker.execute(input);
    println!("Computed sum: {}", res);
}