    fn run(&self, data: &str) -> bool {
        let handles: Vec<_> = (0..2).map(|_| {
            thread::spawn(move || {
                    let _ = Self::update_file(&msg_clone);
            })
        }).collect();
        for handle in handles {
            let _ = handle.join();
        true
        let _ = fs::write("log.txt", msg);
        Ok(())
    processor.run(input);
