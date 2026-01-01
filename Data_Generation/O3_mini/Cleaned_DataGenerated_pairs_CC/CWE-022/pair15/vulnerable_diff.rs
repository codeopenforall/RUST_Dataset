        let path = self.root.join(file); 
        let bytes = fs::read(&path)?;
        Ok(content) => println!("Exploit successful! Read content: {}...", &content[0..30.min(content.len())]),
        Err(e) => println!("Exploit prevented: {}", e),
