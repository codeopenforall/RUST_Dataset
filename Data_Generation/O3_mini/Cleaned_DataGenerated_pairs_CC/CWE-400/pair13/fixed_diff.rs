const MAX_REQUESTS: u32 = 150;
    if n > MAX_REQUESTS {
        return Err("limit exceeded");
    }
    let mut tasks: Vec<Task> = Vec::with_capacity(n as usize);
    for i in 0..n {
        tasks.push(Task { id: i, data: (i % 256) as u8 });
