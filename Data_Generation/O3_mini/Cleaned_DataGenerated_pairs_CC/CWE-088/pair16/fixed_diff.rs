        let output = Command::new("echo")
            .arg(input)
            .output()
            .map_err(|e| format!("Execution failed: {}", e))?;
        let result = String::from_utf8_lossy(&output.stdout).into_owned();
        Ok(result)
    let payload = "test; echo injected";
    match runner.run(payload) {
