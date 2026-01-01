        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o600)  
            .open(path)?;
            file.write_all(b"Secure log entry\n").unwrap();
