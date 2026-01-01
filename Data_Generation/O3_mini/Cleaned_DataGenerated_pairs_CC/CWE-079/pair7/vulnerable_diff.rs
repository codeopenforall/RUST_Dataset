        unsafe {
            let bytes = input.as_bytes();
            let untrusted = std::str::from_utf8_unchecked(bytes);
            self.template.replace("{}", untrusted)
