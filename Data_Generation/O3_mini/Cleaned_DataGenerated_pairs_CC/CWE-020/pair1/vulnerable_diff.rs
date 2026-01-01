        let data_ptr = unsafe { input.as_ptr().add(4) };
        let message_slice = unsafe { std::slice::from_raw_parts(data_ptr, claimed) };
        let result_str = unsafe { std::str::from_utf8_unchecked(message_slice) };
        Ok(result_str.to_string())
