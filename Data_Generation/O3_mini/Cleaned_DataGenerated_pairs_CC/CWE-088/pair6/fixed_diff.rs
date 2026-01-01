    fn execute(&self, input: String) -> io::Result<String> {
        let output = Command::new("echo")
            .arg(input)
