        if index >= self.len {
            panic!("Index {} out of bounds for length {}", index, self.len);
        }
    let result = std::panic::catch_unwind(|| {
        data.write(4, 42);
    });
    match result {
        Ok(_) => Ok(data.sum()),
        Err(_) => Err("Detected out-of-bounds write via panic"),
    }
