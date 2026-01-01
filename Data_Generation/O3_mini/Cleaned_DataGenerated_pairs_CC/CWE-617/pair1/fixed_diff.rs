    fn update(&mut self, multiplier: usize) -> Result<(), &'static str> {
        let new_value = self.value.wrapping_mul(multiplier);
        if new_value >= THRESHOLD {
            return Err("Update rejected: value would exceed safe threshold.");
        }
            *ptr = new_value;
        Ok(())
            let mut instance = service_clone.lock().unwrap();
            if let Err(e) = instance.update(factor) {
                eprintln!("Thread {}: {}", i, e);
            }
