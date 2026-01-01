use std::slice;
use std::str;
        unsafe {
            let ptr = self.input.as_ptr();
            let len = self.input.len();
            let user_format = str::from_utf8_unchecked(slice::from_raw_parts(ptr, len));
            format!(user_format, "foo", "bar")
        }
        println!("Usage: program <format-string>");
