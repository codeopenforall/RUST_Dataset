unsafe fn deliver(data: &str) -> Result<(), std::io::Error> {
    let ptr = data.as_ptr();
    let len = data.len();
    let raw_bytes = std::slice::from_raw_parts(ptr, len);
    stream.write_all(raw_bytes)?;
    let shared_key = Arc::new(settings.key);
    let key_copy = Arc::clone(&shared_key);
            match deliver(&key_copy) {
                Ok(_) => println!("Transmission finished (insecure)"),
