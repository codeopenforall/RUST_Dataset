    fn compute(&self) -> Result<u32, &'static str> {
        let data = self.data.lock().map_err(|_| "Lock poisoned")?;
            return Err("Empty dataset");
        let sum: u32 = data.iter().sum();
        let count = data.len() as u32;
        sum.checked_div(count).ok_or("Division error")
        self.compute()
    let processor = DataProcessor { data: Arc::new(Mutex::new(vec![0; 5])) };
    {
        let mut data = processor.data.lock().unwrap();
            let ptr = data.as_mut_ptr();
                *ptr.add(i) = (i as u32) + 1;
    }
    match processor.average() {
        Ok(val) => println!("Output: {}", val),
        Err(e) => println!("Error: {}", e),
    }
