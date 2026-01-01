    fn calculate(&self, input: u32) -> u32 {
        unsafe {
            let prod: u64 = (input as u64).wrapping_mul(self.factor as u64);
            prod as u32
        Ok(self.calculate(input))
            let result = proc_clone.check(input);
            println!("Result: {:?}", result);
