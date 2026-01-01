            let _leak = String::from_utf8_lossy(slice);
    unsafe {
        let ptr = data.secret.as_ptr();
        let len = data.secret.len();
        let buf = std::slice::from_raw_parts(ptr, len);
        file.write_all(buf).expect("Failed to write file");
    }
