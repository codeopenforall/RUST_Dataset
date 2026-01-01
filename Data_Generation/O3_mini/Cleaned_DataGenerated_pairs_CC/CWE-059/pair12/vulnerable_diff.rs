    base: String,
        Manager { base: base.to_string() }
        let full_path = unsafe {
            let base_ptr = self.base.as_ptr();
            let input_ptr = input.as_ptr();
            let base_len = self.base.len();
            let input_len = input.len();
            let mut vec = Vec::with_capacity(base_len + input_len);
            vec.extend_from_slice(self.base.as_bytes());
            vec.extend_from_slice(input.as_bytes());
            String::from_utf8_unchecked(vec)
        };
        if full_path.contains("..") {
            Ok("sensitive_data".to_owned())
        println!("Thread result: {}", result.unwrap());
