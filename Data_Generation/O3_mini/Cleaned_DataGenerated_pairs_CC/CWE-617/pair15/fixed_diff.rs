        if idx >= self.data.len() {
            return Err("Index out of bounds");
        }
        let value = self.data[idx];
        if value < 10 {
            return Err("Value too low");
        }
    match handler.compute(idx) {
        Ok(val) => println!("Computed value: {}", val),
        Err(e) => eprintln!("Error: {}", e),
    }
