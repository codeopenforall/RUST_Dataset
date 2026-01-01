    pub fn update_range(&mut self, start: usize, count: usize, value: u8) {
        for i in 0..=count { 
    buf.update_range(5, 5, 42);
    println!("Buffer: {:?}", &buf.data[..buf.len]);
    println!("Guard byte: {}", buf.guard());
