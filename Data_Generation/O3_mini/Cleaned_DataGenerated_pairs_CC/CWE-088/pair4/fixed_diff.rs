fn safe_run() -> String {
    let clean: String = untrusted
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect();
    let output = Command::new("echo")
        .arg(clean)
        .output()
        .expect("failed to execute process");
            let res = safe_run();
