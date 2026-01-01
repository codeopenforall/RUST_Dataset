use std::fs::{canonicalize, File};
        let safe_path = canonicalize(path)?;
        let meta = std::fs::metadata(&safe_path)?;
        let mut file = File::open(&safe_path)?;
