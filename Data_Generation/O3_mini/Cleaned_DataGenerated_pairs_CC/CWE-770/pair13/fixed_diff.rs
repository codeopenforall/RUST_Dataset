const THREAD_LIMIT: usize = 1000; 
            let mut buf = Vec::with_capacity(1);
            buf.set_len(1);
            buf[0] = (task_id % 256) as u8;
    if task_count > THREAD_LIMIT {
        return Err("Requested task count exceeds allowed limit");
    }
