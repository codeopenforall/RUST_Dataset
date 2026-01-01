////////////////////// Corrected Code //////////////////////
// This Rust program corrects the vulnerability by performing proper input
// validation and boundary checking before executing the core operation.
// It uses a safe branch to reject attacker-influenced indices that would result
// in an invariant violation. The operation now returns a Result instead of triggering
// a panic, and the threads execute the safe logic.
use std::sync::Arc;
use std::thread;

trait Operation {
    // Safe operation that processes the element at "index" and returns a Result.
    fn process(&self, index: usize) -> Result<(), &'static str>;
}

struct Container {
    elements: Vec<u32>,
}

impl Container {
    fn create() -> Self {
        // Data vector with fixed values.
        Self {
            elements: vec![50, 150, 200, 75, 125],
        }
    }
}

impl Operation for Container {
    fn process(&self, index: usize) -> Result<(), &'static str> {
        // Validate that the index is within bounds.
        if index >= self.elements.len() {
            return Err("Index out-of-range");
        }
        let value = self.elements[index];
        // Instead of asserting, we check the invariant and return an error gracefully.
        if value <= 100 {
            return Err("Invariant violation: element is not > 100");
        }
        Ok(())
    }
}

// Spawns a thread to execute the safe processing operation.
fn run_task(index: usize, cnt: Arc<Container>) -> Result<(), &'static str> {
    let cnt_safe = Arc::clone(&cnt);
    let handle = thread::spawn(move || {
        cnt_safe.process(index)
    });
    handle.join().unwrap()
}

fn main() {
    // Read input, but enforce safe defaults.
    let args: Vec<String> = std::env::args().collect();
    // Use a default index of 1 if no argument is provided.
    let mut user_index: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(1)
    } else {
        1
    };
    let container = Arc::new(Container::create());
    // Validate input: if the chosen index leads to an invariant violation,
    // override it with a safe index.
    if user_index >= container.elements.len() || container.elements[user_index] <= 100 {
        // Log the incident and choose a known-good index.
        eprintln!("Provided index is unsafe. Falling back to index = 2.");
        user_index = 2;
    }
    match run_task(user_index, container) {
        Ok(_) => println!("Processing completed successfully."),
        Err(err) => println!("Processing failed: {}", err),
    }
}
//////////////////////////////////////////////////////////////