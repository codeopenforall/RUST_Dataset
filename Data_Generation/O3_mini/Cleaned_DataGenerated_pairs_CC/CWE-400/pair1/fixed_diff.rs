        const MAX_LOAD: usize = 100;
        if load > MAX_LOAD {
            return Err("Input load too high".to_string());
