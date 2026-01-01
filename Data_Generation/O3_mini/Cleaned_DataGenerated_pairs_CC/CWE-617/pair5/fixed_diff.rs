    fn update(&self, index: usize, value: u64) -> Result<(), &'static str> {
        if index >= self.data.len() {
            return Err("Index out of range");
        }
        Ok(())
    if let Err(err) = locker.update(5, 42) {
        eprintln!("Failed to update: {}", err);
    } else {
        println!("Execution completed.");
    }
