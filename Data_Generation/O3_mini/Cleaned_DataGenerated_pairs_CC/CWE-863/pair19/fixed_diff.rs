const MAX_COUNT: usize = 50;
        if count > MAX_COUNT {
            return Err("Input exceeds permitted limit");
        }
        let mut buffer = vec![0u32; MAX_COUNT];
                buffer[i] = 42;
