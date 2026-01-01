        let ptr: *const i32;
        unsafe {
            let data_ref = self.data.lock().unwrap();
            ptr = &data_ref.secret as *const i32;
        }
        unsafe { *ptr }
    fn run(&self, _is_admin: bool) -> Result<i32, &'static str> {
            m_clone.run(false)
        let result = handle.join().unwrap();
        match result {
            Ok(val) => println!("Result is: {}", val),
            Err(err) => println!("Error: {}", err),
