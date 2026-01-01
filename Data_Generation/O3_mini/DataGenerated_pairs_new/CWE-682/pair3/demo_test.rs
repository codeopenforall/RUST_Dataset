#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    // Oracle function: verifies that the computation produces the correct result.
    // For inputs a = 100, b = 3, c = 10, the expected result is 30.
    // In the vulnerable version, the function returns 0, causing the test to fail.
    // The fixed version correctly returns 30, passing the test.
    #[test]
    fn check_calculation() {
        // Setup shared state similar to main functions.
        let shared = Arc::new(Mutex::new(10u32));
        let shared_clone = Arc::clone(&shared);

        let handle = thread::spawn(move || {
            let mut value = shared_clone.lock().unwrap();
            unsafe {
                let ptr: *mut u32 = &mut *value;
                *ptr += 5;
            }
        });

        handle.join().unwrap();

        let comp = Computor { multiplier: *shared.lock().unwrap() };

        // Using the test input that exposes the vulnerability when b < c.
        let computed = unsafe { comp.calculate(100, 3, 10) };
        assert_eq!(computed, 30, "Calculation did not yield the expected result 30");
    }
}