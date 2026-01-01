        if idx < self.buffer.len() {
            self.buffer[idx] = value;
            Ok(())
        } else {
            Err("Index out of bounds".into())
                mgr.apply(9, 42 + i as u8).expect("Update failed");
