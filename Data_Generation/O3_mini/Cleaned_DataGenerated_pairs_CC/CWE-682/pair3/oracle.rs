#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;
    #[test]
    fn check_calculation() {
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
        let computed = unsafe { comp.calculate(100, 3, 10) };
        assert_eq!(computed, 30, "Calculation did not yield the expected result 30");
    }
}
