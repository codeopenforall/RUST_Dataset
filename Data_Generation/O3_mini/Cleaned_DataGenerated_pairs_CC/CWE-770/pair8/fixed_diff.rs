const TASK_LIMIT: usize = 100;
        if count > TASK_LIMIT {
            return Err("Number of requested tasks exceeds the allowed limit");
        }
        for _ in 0..count {
    let _ = handler.execute_tasks(50);
