    let capacity = (n / 2) as usize;
    let mut tasks: Vec<Task> = Vec::with_capacity(capacity);
    unsafe {
        for i in 0..n {
            let ptr = tasks.as_mut_ptr().add(i as usize);
            ptr.write(Task { id: i, data: (i % 256) as u8 });
            tasks.set_len((i + 1) as usize);
        }
