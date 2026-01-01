const MAX_CAPACITY: usize = 1024; 
    pub fn append_checked(&mut self, item: u8) -> Result<(), &'static str> {
        if self.data.len() >= MAX_CAPACITY {
            return Err("Resource limit reached");
        }
        Ok(())
        let mut locked = proc.lock().unwrap();
        locked.append_checked(byte)?;
    let proc = Arc::new(Mutex::new(Processor { data: Vec::with_capacity(MAX_CAPACITY) }));
        match simulate_input(input, &proc) {
            Ok(_) => {},
            Err(e) => eprintln!("Error: {}", e),
        }
