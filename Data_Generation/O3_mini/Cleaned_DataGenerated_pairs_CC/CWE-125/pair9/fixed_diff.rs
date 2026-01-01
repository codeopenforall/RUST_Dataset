        if index < self.data.len() {
            unsafe {
                Ok(*self.data.as_ptr().add(index))
            }
        } else {
            Err("Index out of bounds")
