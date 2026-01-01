use std::path::PathBuf;
        let path = self.base.join(input);
            let s = path.as_os_str().to_str().unwrap();
        fs::read_to_string(path)
