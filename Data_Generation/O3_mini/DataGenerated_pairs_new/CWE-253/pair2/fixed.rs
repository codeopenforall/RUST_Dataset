use std::thread;

struct Computation;

impl Computation {
    fn run(&self, x: i32) -> bool {
        // Using the same unsafe operation.
        let res = unsafe { Self::operation(x) };
        // CORRECTED CHECK: Properly detects an error and returns false.
        if res.is_err() {
            return false;
        }
        let val = res.unwrap();
        val >= 0
    }

    // The unsafe operation remains unchanged.
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
        // For an invalid (negative) input, the correct behavior should fail.
        cmp.run(-5)
    });
    let result = handle.join().unwrap();
    if result {
        println!("Operation succeeded");
    } else {
        println!("Operation failed");
    }
}