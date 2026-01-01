struct Runner {
    input_search: String,
impl Runner {
    fn sanitize(&self) -> String {
        let safe_whitelist = vec!["/usr/bin", "/bin", "/usr/local/bin"];
        let parts: Vec<&str> = self.input_search.split(':').collect();
        let validated: Vec<&str> = parts.into_iter()
            .filter(|d| safe_whitelist.contains(d))
            .collect();
        if validated.is_empty() {
            safe_whitelist.join(":")
        } else {
            validated.join(":")
        }
    }
    fn execute(&self) {
        let validated_search = self.sanitize();
        env::set_var("PATH", &validated_search);
        let raw_ptr = validated_search.as_ptr();
        let len = validated_search.len();
                    let path_str = std::str::from_utf8(slice).unwrap();
                    println!("Validated search path: {}", path_str);
            .arg("Executing secure command")
            .expect("command execution failed");
    let input_search = env::var("BAD_PATH")
    let runner = Runner { input_search };
    runner.execute();
