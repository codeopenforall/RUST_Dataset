    fn subtract_checked(&mut self, amount: u32) -> Result<u32, &'static str> {
        if self.value < amount {
            return Err("Integer underflow detected");
        self.value = self.value - amount;
        Ok(self.value)
        container.subtract_checked(amount)
    match process(shared.clone(), 1) {
