/* Vulnerability Example: This program uses an unsafe block to perform unchecked
   wrapping arithmetic on a u8 accumulator. When adding 10 to an initial value of
   250, the result wraps around (250 + 10 = 4), which is not the intended behavior.
   This mimics a CWE-190 Integer Overflow vulnerability. The code uses a struct,
   methods and a main entry point similar to real-world Rust applications. */
use std::sync::{Arc, Mutex};
use std::thread;

struct Accumulator {
    value: u8,
}

impl Accumulator {
    fn new(init: u8) -> Self {
        Self { value: init }
    }

    // The unsafe arithmetic here does not check for overflow.
    fn update(&mut self, add: u8) {
        unsafe {
            // Vulnerable: unchecked wrapping addition in an unsafe block.
            // If the addition overflows, the result wraps around.
            self.value = self.value.wrapping_add(add);
        }
    }

    fn get(&self) -> u8 {
        self.value
    }
}

// This function performs the calculation and returns the resulting accumulator value.
fn run_calculation() -> u8 {
    let mut acc = Accumulator::new(250);
    // This addition is intended to trigger an overflow on a u8.
    acc.update(10);
    acc.get()
}

// The main method spawns a few threads to mimic real-world concurrency,
// but for simplicity, we call the helper once and print its result.
fn main() {
    // In a concurrent scenario, shared data might be updated unsafely.
    let shared = Arc::new(Mutex::new(run_calculation()));
    let mut threads = vec![];
    for _ in 0..2 {
        let s = Arc::clone(&shared);
        threads.push(thread::spawn(move || {
            let val = *s.lock().unwrap();
            // Just a dummy concurrent read.
            println!("Thread sees value: {}", val);
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    println!("Final result: {}", run_calculation());
}