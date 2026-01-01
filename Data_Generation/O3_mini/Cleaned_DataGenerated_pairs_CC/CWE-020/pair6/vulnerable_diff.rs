use std::slice;
        unsafe {
            let ptr = data.as_ptr();
            let unslice = slice::from_raw_parts(ptr, expected_len);
            let result = str::from_utf8_unchecked(unslice);
            Ok(result.to_owned())
