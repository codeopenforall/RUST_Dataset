        let mut guard = self.data.lock().unwrap();
        let current = *guard;
        match current.checked_sub(amount) {
            Some(new_val) => *guard = new_val,
            None => panic!("Underflow detected: cannot subtract {} from {}", amount, current),
    let _ = run();
