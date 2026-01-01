    fn update(&self, s: &str) -> Result<(), String> {
        let new_val = s.trim().parse::<u32>()
            .map_err(|e| format!("Failed to parse configuration: {}", e))?;
        let mut guard = self.config.lock().map_err(|_| "Lock poisoned")?;
        *guard = Config { value: new_val };
        Ok(())
        if let Err(e) = thread_mgr.update("not_a_number") {
            eprintln!("Error updating configuration: {}", e);
        }
    handle.join().expect("Thread panicked");
