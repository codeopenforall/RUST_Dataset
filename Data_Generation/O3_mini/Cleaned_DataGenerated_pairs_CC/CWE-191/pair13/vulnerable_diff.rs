    pub unsafe fn reduce(&mut self, amount: u32) {
        self.value = self.value - amount;
        unsafe {
            guard.reduce(amount);
        }
