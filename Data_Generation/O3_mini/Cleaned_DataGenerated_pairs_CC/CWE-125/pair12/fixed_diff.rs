        if index < self.data.len() {
            Ok(self.data[index])
        } else {
            Err("Index out-of-range")
