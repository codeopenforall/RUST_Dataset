use std::fs::File;
        FileHandler {
            base_dir: PathBuf::from(base),
        }
        let target_path = self.base_dir.join(input);
        let mut file = unsafe {
            File::open(&target_path)?
        };
