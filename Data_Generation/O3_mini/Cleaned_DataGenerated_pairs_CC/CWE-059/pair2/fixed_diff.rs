use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::env::current_exe;
        let file = File::open(path)?;
        let _metadata = file.metadata()?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
