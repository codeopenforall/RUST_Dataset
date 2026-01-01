#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;
    #[test]
    fn sum_oracle() {
        let shared = Arc::new(Mutex::new(Data::new(10)));
        let mut handles = vec![];
        for _ in 0..4 {
            let clone = shared.clone();
            handles.push(thread::spawn(move || {
                let guard = clone.lock().unwrap();
                guard.sum()
            }));
        }
        let mut total = 0u64;
        for handle in handles {
            total += handle.join().unwrap();
        }
        assert_eq!(total, 180, "The computed total does not match the expected result");
    }
}
