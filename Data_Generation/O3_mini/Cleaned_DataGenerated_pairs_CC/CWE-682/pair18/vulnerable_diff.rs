        unsafe {
            let ptr: *const i32 = &self.multiplier;
            let factor = *ptr / self.divisor;  
            self.base * factor
        }
