    fn adjust(&mut self, amount: u32) {
        unsafe {
            let ptr: *mut u32 = &mut self.balance;
            *ptr = *ptr - amount;
        acc.adjust(20);
