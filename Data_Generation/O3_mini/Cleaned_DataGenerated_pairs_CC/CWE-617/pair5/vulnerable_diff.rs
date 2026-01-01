    fn update(&self, index: usize, value: u64) {
            assert!(index < self.data.len(), "Index out of range: {}", index);
    let locker_clone = locker;
    let handle = thread::spawn(move || {
        locker_clone.update(15, 42);
    });
    let _ = handle.join();
    println!("Execution completed.");
