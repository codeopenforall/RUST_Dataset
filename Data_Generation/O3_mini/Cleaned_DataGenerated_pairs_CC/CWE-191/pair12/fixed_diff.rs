    pub fn reduce(&mut self, sub: u32) -> Result<u32, &'static str> {
        if self.count < sub {
            return Err("Underflow error: subtraction would go below zero");
        }
        self.count -= sub;
        Ok(self.count)
    data.reduce(10)
                match lock.reduce(10) {
                    Ok(val) => println!("Subtraction result: {}", val),
                    Err(e) => println!("Error: {}", e),
