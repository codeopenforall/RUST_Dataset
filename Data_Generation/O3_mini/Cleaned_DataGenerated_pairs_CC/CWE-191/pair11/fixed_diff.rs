        if pos < 5 {
            return Err("Input value too small, causes underflow");
        }
        let index = pos - 5; 
        if index >= self.arr.len() {
            return Err("Index out of bounds");
        }
        Ok(self.arr[index])
    let handles: Vec<_> = (5..8).map(|i| {
