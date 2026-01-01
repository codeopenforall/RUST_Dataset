        let buf = vec![0u8; size];
        if index < self.buf.len() {
            self.buf[index] = value;
            Ok(())
        } else {
            Err("index out of bounds")
    match handler.update(10, 255) {
        Ok(_) => println!("Main (fixed): update succeeded unexpectedly."),
        Err(e) => println!("Main (fixed): error occurred: {}", e),
