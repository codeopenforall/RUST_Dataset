//////////////////////
// Corrected Example
//////////////////////
use std::thread;

struct Settings {
    addr: String,
    port: u16,
    max_conn: usize,
}

impl Settings {
    // Safe constructor that fully initializes all fields.
    fn new(addr: &str, port: u16, max_conn: usize) -> Self {
        Settings {
            addr: addr.to_owned(),
            port,
            max_conn,
        }
    }
    
    // Computes a result based on the fields.
    fn calc(&self) -> usize {
        self.max_conn + self.port as usize
    }
}

// Public API to compute a result using properly initialized configuration.
pub fn compute() -> usize {
    // Here, we explicitly provide a valid value for max_conn.
    Settings::new("127.0.0.1", 443, 100).calc()
}

fn run() {
    // Execute the computation and print the result.
    let res = compute();
    println!("Result: {}", res);
}

fn main() {
    // Spawn a thread to simulate concurrent usage.
    let handle = thread::spawn(|| {
        run();
    });
    handle.join().unwrap();
}