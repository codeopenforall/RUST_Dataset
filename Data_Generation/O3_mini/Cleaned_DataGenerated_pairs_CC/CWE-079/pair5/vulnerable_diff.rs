        unsafe {
            let raw_ptr = input.as_ptr();
            let len = input.len();
            let slice = std::slice::from_raw_parts(raw_ptr, len);
            let unescaped = String::from_utf8_lossy(slice);
            page.push_str(&format!("<div>{}</div>", unescaped));
        }
