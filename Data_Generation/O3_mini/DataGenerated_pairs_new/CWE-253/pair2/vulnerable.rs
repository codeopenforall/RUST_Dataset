use std::thread;

struct Computation;

impl Computation {
    fn run(&self, x: i32) -> bool {
        // Using an unsafe operation that returns a Result.
        let res = unsafe { Self::operation(x) };
        // INCORRECT CHECK: Incorrectly treats an error as a successful outcome.
        if res.is_err() {
            return true; // Vulnerability: On error, it returns true instead of indicating failure.
        }
        let val = res.unwrap();
        val >= 0
    }

    // An unsafe operation simulating a low-level computation.
    unsafe fn operation(x: i32) -> Result<i32, i32> {
        if x < 0 {
            Err(-1)
        } else {
            Ok(x * 2)
        }
    }
}

fn main() {
    let cmp = Computation;
    let handle = thread::spawn(move || {
        // For an invalid (negative) input, the correct behavior should be failure.
        cmp.run(-5)
    });
    let result = handle.join().unwrap();
    if result {
        println!("Operation succeeded");
    } else {
        println!("Operation failed");
    }
}