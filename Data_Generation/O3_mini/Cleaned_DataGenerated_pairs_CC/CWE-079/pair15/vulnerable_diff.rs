use std::str;
        let mut output = self.tpl.clone();
        unsafe {
            let bytes = input.as_bytes();
            let untrusted = str::from_utf8_unchecked(bytes);
            output = output.replace("{user_data}", untrusted);
        }
        output
