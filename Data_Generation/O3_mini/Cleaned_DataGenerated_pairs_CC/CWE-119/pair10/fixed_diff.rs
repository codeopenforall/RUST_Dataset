    pub fn update_range(&mut self, start: usize, count: usize, value: u8) -> Result<(), &'static str> {
        if start.checked_add(count).filter(|&sum| sum <= self.len).is_none() {
            return Err("Write range exceeds buffer bounds");
        }
        for i in 0..count {
        Ok(())
    match buf.update_range(5, 5, 42) {
        Ok(()) => {
            println!("Buffer: {:?}", &buf.data[..buf.len]);
            println!("Guard byte: {}", buf.guard());
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
