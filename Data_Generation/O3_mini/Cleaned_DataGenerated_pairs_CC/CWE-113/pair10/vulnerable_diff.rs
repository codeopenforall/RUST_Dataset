use std::ptr;
        unsafe {
            let raw_key = key.as_ptr();
            let raw_value = value.as_ptr();
            let key_len = key.len();
            let value_len = value.len();
            let mut combined = String::with_capacity(key_len + 2 + value_len);
            combined.push_str(key);
            combined.push_str(": ");
            let slice = std::slice::from_raw_parts(raw_value, value_len);
            combined.push_str(std::str::from_utf8_unchecked(slice));
            resp.header = combined;
        }
            unsafe {
                let ptr = resp_clone.as_ptr();
                let len = resp_clone.len();
                let _slice = std::slice::from_raw_parts(ptr, len);
            }
