        unsafe {
            let ptr = input.as_ptr();
            let len = input.len();
            let slice = std::slice::from_raw_parts(ptr, len);
            let collected = String::from_utf8_lossy(slice);
            header.push_str("Content-Length: ");
            header.push_str(&collected.len().to_string());
            header.push_str("\r\n\r\n");
            header.push_str(&collected);
        }
