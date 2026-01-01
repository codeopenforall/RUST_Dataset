    fn compute(&self) -> u32 {
        let data = self.data.lock().unwrap();
            100u32.checked_div(0).unwrap_or_default()
        } else {
            let sum: u32 = data.iter().sum();
            let count = data.len() as u32;
            sum.checked_div(count).unwrap_or_default()
        Ok(self.compute())
    let processor = DataProcessor { data: Arc::new(Mutex::new(vec![])) };
    let processor_clone = DataProcessor { data: Arc::clone(&processor.data) };
    let handle = thread::spawn(move || {
            let data = processor_clone.data.lock().unwrap();
            let ptr = data.as_ptr();
                let _ = *ptr.add(i);
    });
    handle.join().unwrap();
    println!("Output: {:?}", processor.average());
