    unsafe fn decrease(&mut self, deduction: u32) {
        self.value = self.value.wrapping_sub(deduction);
            unsafe {
                guard.decrease(deduction);
            }
