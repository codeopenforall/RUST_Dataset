        if index < self.threshold {
            return None;
        self.data.get(index as usize).copied()
    let processor = Arc::new(Mutex::new(DataProcessor::new(10, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110])));
    let result = processor.lock().unwrap().calculate(10);
