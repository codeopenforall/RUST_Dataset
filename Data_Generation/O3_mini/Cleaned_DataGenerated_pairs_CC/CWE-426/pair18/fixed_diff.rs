use std::path::Path;
        let abs_path = if Path::new(bin).is_absolute() {
            bin.to_string()
        } else {
            match bin {
                "echo" => String::from("/bin/echo"),
                _ => bin.to_string(),
            }
        let c_str = CString::new(abs_path).expect("CString conversion failed");
        let mut cmd = Command::new(self.binary.to_str().unwrap());
        for arg in args {
            cmd.arg(arg);
        }
        match cmd.output() {
            Ok(output) => {
                let result = String::from_utf8_lossy(&output.stdout).to_string();
                Ok(result)
            Err(e) => Err(format!("Execution error: {}", e)),
