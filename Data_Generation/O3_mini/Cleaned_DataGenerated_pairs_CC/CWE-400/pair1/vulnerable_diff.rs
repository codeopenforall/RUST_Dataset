        unsafe {
            if load > 100 {
                panic!("Resource exhaustion triggered: input load {} exceeds limit", load);
            }
