        let content = fs::read_to_string(path)?;
        self.config = content;
        if let Err(e) = eng.load_conf("nonexistent.conf") {
            eprintln!("Error occurred: {}", e);
        }
