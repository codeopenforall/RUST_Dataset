    fn withdraw(&mut self, amount: u32) -> Result<(), &'static str> {
        if self.balance < amount {
            return Err("Insufficient funds: withdrawal would underflow");
        self.balance -= amount;
        Ok(())
        let _ = account.withdraw(1).map_err(|e| eprintln!("Error: {}", e));
