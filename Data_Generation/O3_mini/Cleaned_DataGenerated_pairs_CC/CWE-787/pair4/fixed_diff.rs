        let data = vec![0; size];
    fn store_safe(&mut self, index: usize, value: u32) -> Result<(), &'static str> {
        if index < self.data.len() {
            self.data[index] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
        if let Err(e) = self.buffer.store_safe(0, 100) {
            eprintln!("Error storing value: {}", e);
