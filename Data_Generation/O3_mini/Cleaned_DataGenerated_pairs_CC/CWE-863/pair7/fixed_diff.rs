const MAX_TASKS: usize = 100;
    fn process(&mut self) {
        for byte in self.data.iter_mut() {
            *byte = 0;
    if task_count > MAX_TASKS {
        panic!("Requested {} tasks, but maximum allowed is {}", task_count, MAX_TASKS);
    }
                data: Vec::with_capacity(1024 * 1024),
            worker.data.resize(1024 * 1024, 0);
            worker.process();
        h.join().unwrap();
