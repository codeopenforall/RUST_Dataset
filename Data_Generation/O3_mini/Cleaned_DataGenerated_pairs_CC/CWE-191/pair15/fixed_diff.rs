    fn adjust(&mut self, amount: u32) -> Result<(), String> {
        if amount > self.balance {
            return Err(String::from("Underflow detected: deduction exceeds balance"));
        self.balance -= amount;
        Ok(())
        if let Err(err) = acc.adjust(20) {
            eprintln!("Error: {}", err);
        }
