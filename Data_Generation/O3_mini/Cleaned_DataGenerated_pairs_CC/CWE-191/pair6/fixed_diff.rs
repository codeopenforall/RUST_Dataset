    fn subtract(&mut self, sub: u64) -> Result<(), &'static str> {
        if self.value < sub {
            return Err("Underflow detected");
        }
        self.value -= sub;
        Ok(())
    comp.subtract(sub)?;
