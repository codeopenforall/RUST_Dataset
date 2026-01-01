        "/usr/local/bin/helper_program"
        let output = Command::new(target)
            .output()
            .map_err(|e| e.to_string())?;
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
