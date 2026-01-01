        unsafe {
            let output = Command::new(&self.tool).arg("--version").output();
            match output {
                Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).into_owned()),
                Err(e) => Err(format!("Command failed: {}", e)),
            }
    let runner = Arc::new(Processor::new("untrusted_executable"));
    println!("Program execution complete (vulnerable).");
