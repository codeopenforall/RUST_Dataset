    command: String,
        Loader { command: String::from("vuln_cmd_exe") }
            return Err(String::from("Insecure execution: relative path used"));
        let cmd_clone = self.command.clone();
        let ptr: *const u8 = unsafe { cmd_clone.as_ptr().offset(0) };
        unsafe {
            let bytes = std::slice::from_raw_parts(ptr, cmd_clone.len());
            let loaded_cmd = String::from_utf8_lossy(bytes).into_owned();
            let output = Command::new(loaded_cmd)
                .arg("--version")
                .output();
            match output {
                Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).into_owned()),
                Err(e) => Err(format!("Process launch failed: {}", e)),
            }
