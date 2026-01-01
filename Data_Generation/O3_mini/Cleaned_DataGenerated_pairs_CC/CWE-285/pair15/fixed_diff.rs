        let data = self.data.lock().unwrap();
        data.secret
    fn run(&self, is_admin: bool) -> Result<i32, &'static str> {
        if !is_admin {
            return Err("Unauthorized access");
        }
    let authorized = m.run(true);
    match authorized {
        Ok(val) => println!("Authorized result: {}", val),
        Err(err) => println!("Error: {}", err),
    }
            m_clone.run(true)
        match handle.join().unwrap() {
            Ok(val) => println!("Thread result: {}", val),
            Err(err) => println!("Thread error: {}", err),
