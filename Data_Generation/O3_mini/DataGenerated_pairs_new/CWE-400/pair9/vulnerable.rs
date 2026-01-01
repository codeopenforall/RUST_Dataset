use std::thread;

/// A type that processes a list of numbers.
struct Processor;

impl Processor {
    /// Processes the supplied data in a new thread.
    /// 
    /// WARNING: This implementation uses unsafe pointer arithmetic and an
    /// artificial boundary check that limits processing to 100 elements,
    /// regardless of the actual input length. This is a flawed approach that
    /// may lead to incorrect results when large inputs are provided,
    /// effectively causing a denial-of-service (DoS) if an attacker deliberately
    /// supplies oversized data.
    pub fn run(&self, data: Vec<u32>) -> u32 {
        // Spawn a worker thread to perform the sum
        let handle = thread::spawn(move || {
            unsafe {
                let ptr = data.as_ptr();
                let len = data.len();
                // Vulnerability: if the input length exceeds 100, only the first 100
                // elements are processed. This fixed boundary is an arbitrary limitation
                // that could be exploited to cause uncontrolled resource consumption.
                let limit = if len > 100 { 100 } else { len };
                let mut total = 0;
                for i in 0..limit {
                    total += *ptr.add(i);
                }
                total
            }
        });
        handle.join().unwrap()
    }
}

/// Exposed API function for external callers (e.g. test oracle).
/// This function simply delegates to the Processor's run() method.
pub fn run_fn(data: Vec<u32>) -> u32 {
    let proc = Processor;
    proc.run(data)
}

fn main() {
    let proc = Processor;
    // Example input that may come from untrusted sources.
    let input = vec![1u32; 150]; // 150 ones; the correct sum should be 150.
    // Due to the vulnerability, only the first 100 items are processed.
    let res = proc.run(input);
    println!("Computed sum: {}", res);
}