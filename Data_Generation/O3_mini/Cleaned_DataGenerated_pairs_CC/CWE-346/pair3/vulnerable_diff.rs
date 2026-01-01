        unsafe {
            let trusted = CStr::from_ptr(TRUSTED.as_ptr() as *const i8)
                .to_str()
                .unwrap();
            let header = std::str::from_utf8_unchecked(req_clone.header.as_bytes());
            if header != trusted {
                println!("Info: Origin mismatch detected, but processing anyway.");
            }
    handle.join().unwrap();
    Ok(())
    println!("Request handled.");
