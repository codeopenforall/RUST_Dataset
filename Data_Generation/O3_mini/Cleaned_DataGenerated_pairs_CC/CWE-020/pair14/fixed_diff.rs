use std::error::Error;
    fn process(&self, input: &str) -> Result<String, Box<dyn Error>> {
            return Err("Input too short to contain length field".into());
        let len: usize = len_str.parse()?;
        if input.len() < 4 + len {
            return Err("Declared length exceeds input size".into());
        }
        let result = std::str::from_utf8(data_slice.as_bytes())?;
        Ok(result.to_string())
        eprintln!("Usage: {} <input>", args[0]);
            match shared_clone.process(&input_clone) {
                Ok(result) => println!("Output: {}", result),
                Err(e) => eprintln!("Error: {}", e),
            }
