        let result = unsafe { self.operation(input) };
        if result.is_err() {
            return Ok(());  
        }
        result
