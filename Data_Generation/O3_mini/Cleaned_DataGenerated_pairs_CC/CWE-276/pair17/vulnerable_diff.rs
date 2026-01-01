        let file = unsafe {
            OpenOptions::new()
                .write(true)
                .create(true)
                .mode(0o777)  
                .open(path)?
        };
            file.write_all(b"Insecure log entry\n").unwrap();
