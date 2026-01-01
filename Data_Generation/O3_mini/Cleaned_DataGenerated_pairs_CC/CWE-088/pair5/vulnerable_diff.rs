    data: String,
    fn new(data: String) -> Self {
        Executor { data }
        let ptr = self.data.as_ptr();
        let len = self.data.len();
        let dangerous_input = unsafe {
        let cmd_str = format!("echo safe && {}", dangerous_input);
        exe.run().unwrap_or_else(|e| {
            eprintln!("execution error: {}", e);
            "error".to_string()
        })
