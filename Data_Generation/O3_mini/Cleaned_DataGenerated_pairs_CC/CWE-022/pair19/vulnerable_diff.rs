        let target_path = self.base_dir.join(filename); 
        fs::read_to_string(&target_path).map_err(|e| format!("Error: {}", e))
