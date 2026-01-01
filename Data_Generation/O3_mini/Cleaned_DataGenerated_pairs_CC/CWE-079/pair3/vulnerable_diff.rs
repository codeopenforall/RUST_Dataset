        unsafe {
            let raw_input: &str = std::str::from_utf8_unchecked(input.as_bytes());
            format!("<html><body>Hello, {}!</body></html>", raw_input)
        }
