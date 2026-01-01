        let output = Command::new("echo")
                .arg(&self.input)
                .expect("failed to execute process");
        {
            let mut guard = self.state.lock().unwrap();
            *guard += 1;
