const MAX_TASKS: usize = 1000; 
        if count > MAX_TASKS {
            return Err(format!(
                "Task count ({}) exceeds the maximum allowed ({})",
                count, MAX_TASKS
            ));
        }
    if let Err(err) = handler.run(500) {
    println!("Successfully completed processing with proper throttling.");
