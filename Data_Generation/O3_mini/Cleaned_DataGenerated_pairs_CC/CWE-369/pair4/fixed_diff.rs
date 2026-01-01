    pub unsafe fn compute(&self, a: i32, b: i32) -> Result<i32, &'static str> {
        if b == 0 {
            return Err("Division by zero");
        }
        let divisor = *b_ptr;
        Ok(numerator / divisor)
    match calc.compute(a, b) {
        Ok(val) => val,
        Err(err) => {
            println!("Error encountered: {}", err);
            0  
        }
    }
                let res = match calc_clone.compute(pair.0, pair.1) {
                    Ok(val) => val,
                    Err(err) => {
                        println!("Error encountered: {}", err);
                        0
                    }
                };
