    content: String,
    fn new(content: String) -> Self {
        Executor { content }
    }
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        let forbidden = [';', '&', '|', '$', '`', '>', '<'];
        if self.content.chars().any(|c| forbidden.contains(&c)) {
            return Err("Input contains disallowed characters".into());
        }
        Ok(())
        self.validate()?;
        let ptr = self.content.as_ptr();
        let len = self.content.len();
        let safe_input = unsafe {
        let cmd_str = format!("echo safe && {}", safe_input);
        match exe.run() {
            Ok(result) => result,
            Err(e) => {
                eprintln!("execution error: {}", e);
                "error".to_string()
            }
        }
