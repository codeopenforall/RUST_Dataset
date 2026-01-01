    fn update(&mut self, new_value: usize) {
        unsafe {
            let ptr = &mut self.counter as *mut usize;
            *ptr = new_value;
        assert!(self.counter <= self.limit, "Value exceeds maximum allowed");
    worker.update(input);
    Ok(())
        worker.update(150);
    println!("Completed processing in vulnerable version.");
