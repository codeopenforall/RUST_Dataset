//////////////////////
// Vulnerable Example
//////////////////////
use std::mem::MaybeUninit;
use std::thread;

struct Settings {
    addr: String,
    port: u16,
    max_conn: usize,
}

impl Settings {
    // Unsafe constructor using uninitialized memory.
    // Note: Only addr and port are initialized; max_conn is left uninitialized.
    unsafe fn new(addr: &str, port: u16) -> Self {
        let mut config = MaybeUninit::<Settings>::uninit();
        let ptr = config.as_mut_ptr();
        // Initialize addr and port fields.
        (*ptr).addr = addr.to_owned();
        (*ptr).port = port;
        // BUG: max_conn is not initialized here.
        config.assume_init()
    }
    
    // Computes a result based on the fields.
    fn calc(&self) -> usize {
        self.max_conn + self.port as usize
    }
}

// Public API to compute a result.
// For the vulnerable version, it uses the unsafe constructor.
pub fn compute() -> usize {
    unsafe { Settings::new("127.0.0.1", 443).calc() }
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