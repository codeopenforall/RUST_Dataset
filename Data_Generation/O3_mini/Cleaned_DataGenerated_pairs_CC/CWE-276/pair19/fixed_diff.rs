use std::io::{self};
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o600)
            .open(&path)?;
        drop(file);
    let test_path = "/tmp/secure_resource.txt";
