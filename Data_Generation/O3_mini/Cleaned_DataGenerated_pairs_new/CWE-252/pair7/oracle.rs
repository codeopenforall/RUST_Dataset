#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    #[test]
    fn integrity_check() {
        let engine = Core::new(10);
        assert!(engine.update(5, 77).is_ok(), "Valid update failed");
        let _ = engine.concurrent_update(15, 88);
        thread::sleep(Duration::from_millis(100));
        let data = engine.data.lock().unwrap();
        assert_eq!(data[5], 77, "Valid update was unexpectedly altered");
        if data.iter().any(|&x| x == 88) {
            panic!("Memory corruption detected: out-of-bound update modified valid memory");
        }
    }
}
