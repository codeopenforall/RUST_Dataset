        let mut lock = self.data.lock().map_err(|_| "Lock poisoned")?;
            let ptr = lock.as_mut_ptr();
    fn concurrent_update(&self, index: usize, value: i32) -> thread::JoinHandle<Result<(), &'static str>> {
        let data_clone = self.data.clone();
        thread::spawn(move || {
            let mut lock = data_clone.lock().map_err(|_| "Lock poisoned")?;
            if index >= lock.len() {
                return Err("Index out of bounds");
            }
                let ptr = lock.as_mut_ptr();
            Ok(())
        })
    if let Err(e) = engine.update(9, 42) {
        eprintln!("update error: {}", e);
        return;
    }
    let handle = engine.concurrent_update(8, 55);
    match handle.join() {
        Ok(Ok(())) => {},
        Ok(Err(e)) => {
            eprintln!("concurrent update error: {}", e);
            return;
        },
        Err(_) => {
            eprintln!("Thread panicked");
            return;
        }
    }
    assert!(engine.update(15, 99).is_err(), "Expected error for out-of-bound update");
