//////////////////////////////////////////////////////////////
// A corrected version that protects against integer wraparound
// (CWE-190) by using checked arithmetic. Instead of unsafe unchecked
// addition, we call checked_add and fail gracefully when overflow occurs.
// The concurrency control remains unchanged.
//////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Accumulator {
    balance: u32,
}

impl Accumulator {
    fn new(init: u32) -> Self {
        Accumulator { balance: init }
    }

    fn add(&mut self, amount: u32) -> Result<(), &'static str> {
        // Use checked addition to detect overflow.
        self.balance = self.balance.checked_add(amount).ok_or("Overflow occurred")?;
        Ok(())
    }

    fn current(&self) -> u32 {
        self.balance
    }
}

fn run_calculation(init: u32, add: u32, threads: usize) -> Result<u32, &'static str> {
    let accumulator = Arc::new(Mutex::new(Accumulator::new(init)));
    let mut handles = vec![];

    for _ in 0..threads {
        let acc_clone = Arc::clone(&accumulator);
        // Each thread returns a Result so that errors can propagate.
        handles.push(thread::spawn(move || -> Result<(), &'static str> {
            let mut guard = acc_clone.lock().unwrap();
            guard.add(add)
        }));
    }

    // Check that all threads complete without error.
    for handle in handles {
        let res = handle.join().map_err(|_| "Thread panicked")?;
        if let Err(e) = res {
            return Err(e);
        }
    }

    let guard = accumulator.lock().unwrap();
    Ok(guard.current())
}

pub fn try_compute() -> Result<u32, &'static str> {
    // With the same parameters used in the vulnerable version,
    // the checked addition will detect the overflow condition.
    run_calculation(u32::MAX - 10, 15, 1)
}

fn main() {
    match try_compute() {
        Ok(val) => println!("Final result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}