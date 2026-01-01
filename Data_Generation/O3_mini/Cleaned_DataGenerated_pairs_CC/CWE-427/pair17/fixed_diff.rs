        let raw = env::var("MALICIOUS_PATH").unwrap_or_else(|_| "".to_string());
        let valid = raw.chars().all(|c| c.is_alphanumeric() || c == '/' || c == ':' || c == '-' || c == '.');
        let safe_path = if valid && !raw.is_empty() {
            raw
        } else {
            "/usr/bin".to_string()
            search_dirs: safe_path,
    let child = Command::new(cmd)
        .env("PATH", dirs)
        .arg("--version")
        .spawn()?;
    let output = child.wait_with_output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err("Command execution failed".into())
