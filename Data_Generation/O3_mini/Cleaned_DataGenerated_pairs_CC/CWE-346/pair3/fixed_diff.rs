        let trusted = CStr::from_bytes_with_nul(TRUSTED)
            .unwrap()
            .to_str()
            .unwrap();
        let header = req_clone.header.as_str();
        if header != trusted {
            return Err("Unauthorized origin");
        Ok(())
    let result = handle.join().unwrap();
    result
    println!("Request handled securely.");
