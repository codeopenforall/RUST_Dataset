        buffer.resize(size, 0);
    fn update(&mut self, index: usize, value: u32) -> Result<(), &'static str> {
        if index < self.buffer.len() {
            self.buffer[index] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        let _ = handler.update(10, 42);
