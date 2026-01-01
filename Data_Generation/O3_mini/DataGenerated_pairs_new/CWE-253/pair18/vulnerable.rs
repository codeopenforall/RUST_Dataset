use std::ptr;

struct Processor {
    value: i32,
}

impl Processor {
    fn update(&mut self, new_val: i32) {
        let ptr: *mut i32 = &mut self.value;
        // Call an unsafe function that simulates a sensitive update.
        let res = unsafe { change(ptr, new_val) };
        // Vulnerability: The return value is incorrectly interpreted.
        // Instead of only updating on success, the error branch (Err) is misinterpreted as success.
        if res.is_ok() {
            // When the unsafe function indicates success, nothing further is done.
            // (The target has already been updated via change.)
        } else {
            // Vulnerable branch: Even though the underlying function indicates failure,
            // the code erroneously forces the update.
            self.value = new_val;
        }
    }
}

unsafe fn change(ptr: *mut i32, new_val: i32) -> Result<(), i32> {
    // Simulate an unsafe operation that may fail.
    // If new_val equals 42, the operation fails (emulating an error condition).
    if new_val == 42 {
        Err(new_val)
    } else {
        *ptr = new_val;
        Ok(())
    }
}

fn main() {
    let mut p = Processor { value: 10 };
    // Using the triggering input which should cause a failure in the intended logic.
    p.update(42);
    println!("Value: {}", p.value);
}