        {
            let vec_guard = self.data.lock().unwrap();
            if input.len() > vec_guard.len() {
                return Err(Error::new(ErrorKind::Other, "Buffer overflow"));
            }
        unsafe { self.add_data(input) }
        assert!(res.is_err(), "Thread: Invalid input did not produce an error as expected.");
    println!("Processing complete safely.");
