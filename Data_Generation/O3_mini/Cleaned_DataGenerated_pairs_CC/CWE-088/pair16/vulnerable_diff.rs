        unsafe {
            let command = format!("echo {}", input);
            let output = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output()
                .map_err(|e| format!("Execution failed: {}", e))?;
            let result = String::from_utf8_lossy(&output.stdout).into_owned();
            Ok(result)
        }
    let injection = "test; echo injected";
    match runner.run(injection) {
