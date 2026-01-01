        DataBuffer { data: vec![0; size] }
        if index < self.data.len() {
            self.data[index] = value;
        } else {
            self.data.resize(index + 1, 0);
            self.data[index] = value;
    println!("Buffer updated safely. Value at index 2: {}", buffer.data[2]);
