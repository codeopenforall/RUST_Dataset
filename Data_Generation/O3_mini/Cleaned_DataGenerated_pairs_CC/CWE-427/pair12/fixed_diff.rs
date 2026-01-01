struct SafeExecutor;
impl SafeExecutor {
    fn trusted_path() -> String {
        "/usr/bin:/bin".to_string()
        let safe_path = Self::trusted_path();
        let output = Command::new("echo")
            .env("PATH", safe_path)
            .arg("hello")
            .output();
        match output {
            Ok(result) => {
                let out_str = String::from_utf8_lossy(&result.stdout).to_string();
                Ok(out_str)
            },
            Err(e) => Err(format!("Command execution failed: {:?}", e)),
    let modifier = thread::spawn(|| {
    match SafeExecutor::launch() {
