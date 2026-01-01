const MAX_TASKS: usize = 1000; 
        if tasks > MAX_TASKS {
            return Err("Number of tasks exceeds allowed limit".to_string());
        }
