        let lock = self.data.lock().map_err(|_| "Lock poisoned")?;
            let ptr = lock.as_ptr() as *mut i32;
    fn concurrent_update(&self, index: usize, value: i32) {
        let core_clone = self.data.clone();
        let _ = thread::spawn(move || {
            let lock = core_clone.lock().unwrap();
                let ptr = lock.as_ptr() as *mut i32;
        });
    engine.concurrent_update(15, 42); 
    let _ = engine.update(20, 99); 
    thread::sleep(std::time::Duration::from_millis(50));
