use std::fs::File;
        unsafe {
            let ptr = input.as_ptr();
            let len = input.len();
            let slice = std::slice::from_raw_parts(ptr, len);
            let resolved = String::from_utf8(slice.to_vec())
                .map_err(|_| "Invalid UTF-8 in path".to_string())?;
            Ok(resolved)
    unsafe {
        let file = File::open(&resolved_path)
            .map_err(|e| format!("Error opening file: {}", e))?;
        let mut buf_reader = std::io::BufReader::new(file);
        let mut content = String::new();
        buf_reader
            .read_to_string(&mut content)
            .map_err(|e| format!("Error reading file: {}", e))?;
        Ok(content)
    }
        "test_symlink.txt".to_string()
