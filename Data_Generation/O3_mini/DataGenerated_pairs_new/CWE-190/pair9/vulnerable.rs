//////////////////////////////////////////////////////////////
// A complex application that uses raw pointer arithmetic
// in a concurrent context may inadvertently trigger an
// integer wraparound vulnerability (CWE-190) via unchecked
// arithmetic. This version uses an unsafe intrinsic to add
// numbers without checking for overflow.
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

    fn add(&mut self, amount: u32) {
        // UNSAFE: using an unchecked intrinsic to add values.
        // If the sum exceeds u32::MAX, the result will wrap around.
        unsafe {
            // The unchecked_add intrinsic performs addition without any overflow checks.
            self.balance = std::intrinsics::unchecked_add(self.balance, amount);
        }
    }

    fn current(&self) -> u32 {
        self.balance
    }
}

fn run_calculation(init: u32, add: u32, threads: usize) -> u32 {
    let accumulator = Arc::new(Mutex::new(Accumulator::new(init)));
    let mut handles = vec![];

    for _ in 0..threads {
        let acc_clone = Arc::clone(&accumulator);
        handles.push(thread::spawn(move || {
            let mut guard = acc_clone.lock().unwrap();
            guard.add(add);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let guard = accumulator.lock().unwrap();
    guard.current()
}

pub fn try_compute() -> Result<u32, &'static str> {
    // Parameters chosen to cause an overflow: starting near u32::MAX.
    // For example, if init = u32::MAX - 10 and add = 15:
    // The mathematically correct result would overflow.
    let result = run_calculation(u32::MAX - 10, 15, 1);
    // This vulnerable variant does not report the overflow.
    Ok(result)
}

fn main() {
    let result = try_compute().unwrap();
    println!("Final result: {}", result);
}