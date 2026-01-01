use std::process::Command;
    fn new(input: String) -> Self {
        let allowed = vec!["/usr/bin", "/bin"];
        let sanitized = if allowed.contains(&input.as_str()) {
            input
        } else {
            "/usr/bin".to_string()
        };
        Executor { search_path: sanitized }
        env::set_var("PATH", &self.search_path);
        let output = Command::new("echo")
            .arg("safe_exec")
            .output()
            .expect("Failed to run command");
        output.status.code().unwrap_or(-1)
    let input_path = if args.len() > 1 {
    let exec = Arc::new(Executor::new(input_path));
    if let Some(&first) = results.lock().unwrap().get(0) {
        println!("Execution result: {}", first);
