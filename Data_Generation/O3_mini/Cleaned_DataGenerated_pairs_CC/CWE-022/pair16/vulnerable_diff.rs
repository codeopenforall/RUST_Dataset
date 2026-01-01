            base: PathBuf::from(base),
        let full_path = self.base.join(input);
        unsafe {
            let mut files: Vec<String> = Vec::new();
            let ptr = &mut files as *mut Vec<String>;
            let entries = fs::read_dir(&full_path).map_err(|e| e.to_string())?;
            for entry in entries {
                let entry = entry.map_err(|e| e.to_string())?;
                let file_name = entry.file_name().into_string().unwrap_or_default();
                (*ptr).push(file_name);
            }
            Ok(files)
