        let shell_command = format!("echo {}", self.input);  
        let output = unsafe {
            Command::new("sh")
                .arg("-c")
                .arg(shell_command)
                .expect("failed to execute process")
        };
        let ptr = Arc::as_ptr(&self.state) as *mut Mutex<i32>;
        unsafe {
            let _guard = (*ptr).lock().unwrap();
