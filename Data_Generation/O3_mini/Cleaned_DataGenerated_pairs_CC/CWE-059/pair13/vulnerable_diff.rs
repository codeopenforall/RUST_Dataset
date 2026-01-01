use std::fs::{metadata, File};
        let meta = metadata(path)?;
        let mut file = File::open(path)?;
