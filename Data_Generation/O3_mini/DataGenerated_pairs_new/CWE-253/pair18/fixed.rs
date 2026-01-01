use std::ptr;

struct Processor {
    value: i32,
}

impl Processor {
    fn update(&mut self, new_val: i32) {
        let ptr: *mut i32 = &mut self.value;
        // Call the unsafe function to perform the operation.
        let res = unsafe { change(ptr, new_val) };
        // Correctly check the return value.
        // On success (Ok), the unsafe function has already performed the update.
        // On error (Err), the update is left unapplied.
        if res.is_err() {
            // Here, we explicitly log the error instead of updating the value.
            eprintln!("Update failed for input {}", new_val);
        }
    }
}

unsafe fn change(ptr: *mut i32, new_val: i32) -> Result<(), i32> {
    // Simulate the same unsafe operation.
    if new_val == 42 {
        Err(new_val)
    } else {
        *ptr = new_val;
        Ok(())
    }
}

fn main() {
    let mut p = Processor { value: 10 };
    // Using the triggering input which should cause the update to be rejected.
    p.update(42);
    println!("Value: {}", p.value);
}