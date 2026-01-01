#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;
    #[test]
    fn test_engine() {
        let engine = Arc::new(Mutex::new(Engine::new(0)));
        let engine_cloned = Arc::clone(&engine);
        let handle = thread::spawn(move || {
            let a: u32 = 30000;
            let b: u32 = 150000;
            let mut eng = engine_cloned.lock().unwrap();
            eng.process(a, b);
        });
        handle.join().unwrap();
        let final_value = engine.lock().unwrap().value;
        assert_eq!(final_value, 0, "Engine computed an incorrect value due to integer overflow");
    }
}
