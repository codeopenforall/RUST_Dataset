        if idx < self.data.len() {
            Ok(self.data[idx])
        } else {
            Err("index out of bounds")
