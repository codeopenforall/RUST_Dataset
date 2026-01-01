    fn compute(&self, numerator: i32, denominator: i32) -> Result<i32, String> {
        if denominator == 0 {
            return Err(String::from("Division by zero error"));
        let result = unsafe { numerator / denominator };
        Ok(result)
        self.compute(numerator, denominator)
    match handle.join().unwrap() {
        Ok(value) => println!("Computed result: {}", value),
        Err(err) => println!("Encountered error: {}", err),
    }
