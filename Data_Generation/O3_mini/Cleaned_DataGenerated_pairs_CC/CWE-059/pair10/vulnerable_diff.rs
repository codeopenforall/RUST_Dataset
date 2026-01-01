        let path = Path::new(filename);
        let raw_path = path.as_os_str().as_bytes().as_ptr();
            let len = path.as_os_str().as_bytes().len();
        let final_path_clone = final_path.clone();
            if let Ok(data) = fs::read_to_string(&final_path_clone) {
