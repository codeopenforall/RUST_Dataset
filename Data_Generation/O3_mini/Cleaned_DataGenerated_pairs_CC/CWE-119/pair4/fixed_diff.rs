        if start.checked_add(len).filter(|&sum| sum <= self.data.len()).is_none() {
            return Err("Destination buffer index out of bounds");
        }
        if len > src.len() {
            return Err("Source slice does not have enough elements");
        }
    match buf.update(&source, 8, 5) {
        Ok(_) => println!("Buffer state: {:?}", buf.get()),
        Err(e) => println!("Error: {}", e),
    }
