        let bytes = bin.as_bytes();
        let c_str = unsafe {
            CString::from_vec_unchecked(bytes.to_vec())
        unsafe {
            let mut cmd = Command::new(self.binary.to_str().unwrap());
            for arg in args {
                cmd.arg(arg);
            match cmd.output() {
                Ok(output) => {
                    let result = String::from_utf8_lossy(&output.stdout).to_string();
                    Ok(result)
                }
                Err(e) => Err(format!("Execution error: {}", e)),
            }
