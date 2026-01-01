        if index < self.buffer.len() {
            self.buffer[index] = value;
            Ok(())
        } else {
            Err("Index out-of-bounds")
        let buffer = vec![0u8; size];
        dp.modify(5, 42).expect("In-bound modification must succeed");
    match dp.modify(11, 99) {
        Ok(_) => println!("Unexpectedly modified out-of-bound index"),
        Err(e) => println!("Failed modification as expected: {}", e),
    }
