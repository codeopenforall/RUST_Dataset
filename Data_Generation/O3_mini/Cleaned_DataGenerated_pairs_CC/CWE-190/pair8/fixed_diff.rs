    fn calculate(&self, input: u32) -> Result<u32, &'static str> {
        let prod: u64 = (input as u64).wrapping_mul(self.factor as u64);
        if prod > u32::MAX as u64 {
            Err("Overflow detected")
        } else {
            Ok(prod as u32)
        self.calculate(input)
            match proc_clone.check(input) {
                Ok(result) => println!("Result: {}", result),
                Err(e) => println!("Error: {}", e),
            }
