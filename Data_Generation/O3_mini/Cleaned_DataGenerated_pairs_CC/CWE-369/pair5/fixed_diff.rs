    fn operation(&self) -> Result<i32, &'static str> {
        if self.denominator == 0 {
            return Err("Division by zero error");
        }
        unsafe {
            let numer_ptr: *const i32 = &self.numerator;
            let denom_ptr: *const i32 = &self.denominator;
            Ok(*numer_ptr / *denom_ptr)
        }
        shared_clone.operation()
        Ok(inner) => inner,
        Err(_) => Err("Thread error"),
