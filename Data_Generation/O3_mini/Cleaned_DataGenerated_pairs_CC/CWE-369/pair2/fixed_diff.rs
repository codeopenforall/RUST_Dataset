    fn execute(&self) -> Result<i32, &'static str> {
        if self.denominator == 0 {
            return Err("Error: Division by zero encountered");
        }
            Ok(num / self.denominator)
    let result = handler.join().expect("Thread panicked unexpectedly");
    match result {
        Ok(val) => println!("Result: {}", val),
        Err(err) => println!("{}", err)
    }
