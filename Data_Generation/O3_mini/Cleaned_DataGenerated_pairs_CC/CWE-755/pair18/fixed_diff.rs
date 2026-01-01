        let content = fs::read_to_string(filename)?;
        let val = u8::from_str(content.trim()).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, e)
        })?;
         if let Err(e) = handler_clone.process("nonexistent.txt", 2) {
             eprintln!("Thread encountered error: {}", e);
    if let Err(e) = handler.process("nonexistent.txt", 4) {
         eprintln!("Main encountered error: {}", e);
