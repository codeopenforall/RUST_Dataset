    let unsafe_str = unsafe { std::str::from_utf8_unchecked(input.as_bytes()) };
    format!("<html><body><h1>Welcome {}</h1></body></html>", unsafe_str)
