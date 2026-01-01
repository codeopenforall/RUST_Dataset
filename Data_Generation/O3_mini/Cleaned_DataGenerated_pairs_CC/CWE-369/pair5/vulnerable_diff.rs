    unsafe fn operation(&self) -> i32 {
        let numer_ptr: *const i32 = &self.numerator;
        let denom_ptr: *const i32 = &self.denominator;
        *numer_ptr / *denom_ptr
        unsafe { shared_clone.operation() }
        Ok(res) => Ok(res),
        Err(_) => Err("Unhandled division by zero"),
