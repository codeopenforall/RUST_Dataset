use std::path::PathBuf;
    command: PathBuf,
        Loader { command: PathBuf::from("/usr/bin/fixed_cmd_exe") }
            return Ok(String::from("Secure execution"));
        if !self.command.is_absolute() {
            return Err(String::from("Execution failed: non-absolute path"));
        }
        let output = Command::new(&self.command)
            .arg("--version")
            .output();
        match output {
            Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).into_owned()),
            Err(e) => Err(format!("Process launch failed: {}", e)),
