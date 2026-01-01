        let dangerous = unsafe {
            let raw = input.as_bytes();
            std::str::from_utf8_unchecked(raw)
        };
        format!("{}<div>{}</div>{}", self.header, dangerous, self.content)
