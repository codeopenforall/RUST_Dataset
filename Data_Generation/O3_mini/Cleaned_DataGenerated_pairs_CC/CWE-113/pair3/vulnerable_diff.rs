use std::str;
        unsafe {
            let raw_value = value.as_ptr();
            let len = value.len();
            let value_str = str::from_utf8_unchecked(std::slice::from_raw_parts(raw_value, len));
            self.headers.push(format!("{}: {}", key, value_str));
        }
