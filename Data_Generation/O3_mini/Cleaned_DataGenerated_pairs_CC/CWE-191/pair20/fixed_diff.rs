    fn update(&mut self, amt: u32) -> Result<(), &'static str> {
        if self.balance < amt {
            return Err("Insufficient balance: subtraction would underflow");
        self.balance -= amt;
        Ok(())
        let _ = acc.update(20);
        acc.update(5).expect("Subtraction within bounds");
