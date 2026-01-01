    pub fn lessen(&mut self, amount: u32) {
        self.value = self.value.checked_sub(amount).unwrap_or(0);
        guard.lessen(amount);
