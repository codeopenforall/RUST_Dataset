        unsafe {
            let ptr: *const i32 = &b;
            let divisor = *ptr;
            Ok(a / divisor)
