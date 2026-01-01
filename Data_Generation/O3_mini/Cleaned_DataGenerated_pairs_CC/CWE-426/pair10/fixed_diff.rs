    let cmd = "/bin/ls";
    let output = Command::new(cmd)
        .output()
        .map_err(|e| format!("Execution error: {}", e))?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
