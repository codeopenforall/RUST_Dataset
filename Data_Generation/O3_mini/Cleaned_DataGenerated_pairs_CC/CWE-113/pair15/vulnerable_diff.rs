    unsafe {
        let unchecked = std::str::from_utf8_unchecked(user_data.as_bytes());
        format!("{}{}", prefix, unchecked)
    }
