    fn decrease(&mut self, deduction: u32) -> Result<(), &'static str> {
        if deduction > self.value {
            Err("Integer underflow prevented: deduction exceeds current value")
        } else {
            self.value -= deduction;
            Ok(())
        }
            let _ = guard.decrease(deduction);
