            let pointer = secret.as_ptr();
            let leaked_secret = std::slice::from_raw_parts(pointer, secret.len());
            let leaked_str = std::str::from_utf8_unchecked(leaked_secret);
            Err(format!("Decryption failed on secret: {}", leaked_str))
