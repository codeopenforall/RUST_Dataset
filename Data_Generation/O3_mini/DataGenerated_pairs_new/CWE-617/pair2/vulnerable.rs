////////////////////// Vulnerable Code //////////////////////
// This Rust program uses unsafe pointer manipulation and
// multi‐threaded execution to process attacker–controlled input.
// A shared data structure with an internal vector is exposed via a trait.
// In the unsafe implementation, no checks are performed on the index
// before using raw pointer arithmetic and an assert! is invoked directly
// on the value read from memory. An attacker can supply a low index
// (e.g. 0) so that the value (50) fails the assertion (value > 100),
// causing the program to panic and thus enabling a denial‐of‐service.
use std::sync::Arc;
use std::thread;

trait Operation {
    // Unsafe operation that processes the element at "index".
    // Caller must ensure index is appropriate.
    unsafe fn process(&self, index: usize);
}

struct Container {
    elements: Vec<u32>,
}

impl Container {
    fn create() -> Self {
        // The data vector is initialized with fixed values.
        // Notice that elements[0] is 50 (i.e. below the threshold of 100).
        Self {
            elements: vec![50, 150, 200, 75, 125],
        }
    }
}

impl Operation for Container {
    unsafe fn process(&self, index: usize) {
        // Vulnerable point: using unchecked pointer arithmetic.
        let ptr = self.elements.as_ptr();
        // Line below can read an unintended element if index is attacker-controlled.
        let value = *ptr.add(index);
        // REACHABLE ASSERTION: if the value is not > 100,
        // this assertion will panic, enabling a DoS attack.
        assert!(value > 100, "Invariant failure: element is not > 100");
    }
}

// Spawns a thread to execute the processing operation.
fn run_task(index: usize, cnt: Arc<Container>) {
    let handle = thread::spawn(move || unsafe {
        cnt.process(index);
    });
    // If the thread panics (due to assert!), the unwrap() will propagate the panic.
    handle.join().unwrap();
}

fn main() {
    // Read attacker-influenced input (via command-line argument).
    let args: Vec<String> = std::env::args().collect();
    // If no argument is provided, default to 0 (an attacker-chosen value to trigger panic).
    let user_index: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };
    let shared = Arc::new(Container::create());
    run_task(user_index, shared);
}
//////////////////////////////////////////////////////////////