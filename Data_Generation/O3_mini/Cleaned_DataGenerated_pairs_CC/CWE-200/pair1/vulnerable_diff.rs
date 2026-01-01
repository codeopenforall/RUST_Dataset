        let slice = std::slice::from_raw_parts(secret_ptr, secret_len);
        let extracted = String::from_utf8_lossy(slice);
            return Err(format!("Processing failed, sensitive details: {:?}", extracted));
