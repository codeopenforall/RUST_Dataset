use std::path::PathBuf;
        let file_path = self.base_dir.join(name); 
        let mut file = File::open(&file_path)
