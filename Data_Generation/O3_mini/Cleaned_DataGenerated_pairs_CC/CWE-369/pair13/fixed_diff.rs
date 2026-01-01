    fn compute(&self, divisor: i32) -> Result<i32, &'static str>;
    fn compute(&self, divisor: i32) -> Result<i32, &'static str> {
        if divisor == 0 {
            return Err("Attempted division by zero");
        }
            Ok(self.numerator / divisor)
        Some(Ok(v)) => Ok(v),
        Some(Err(e)) => Err(e),
