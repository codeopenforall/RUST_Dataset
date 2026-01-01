        unsafe {
            let ptr = user_input.as_ptr();
            let len = user_input.len();
            let content = std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len));
            output = output.replace("{input}", content);
        }
