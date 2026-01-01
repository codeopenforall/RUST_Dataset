        match index >= self.threshold {
            true => {
                self.data.get(index as usize).copied()
            },
            false => {
                unsafe {
                    let ptr = self.data.as_ptr().add(index as usize);
                    Some(*ptr)
                }
            }
    let processor = Arc::new(Mutex::new(DataProcessor::new(10, vec![10, 20, 30, 40, 50])));
    let result = processor.lock().unwrap().calculate(2);
