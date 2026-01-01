        if idx >= self.buffer.len() {
            return Err("Index out of bounds");
        self.buffer[idx] = val;
        match manager.update(5, 42) {
            Ok(_) => println!("Unexpected update. Buffer state may be corrupted."),
            Err(e) => println!("Error: {}", e),
        }
