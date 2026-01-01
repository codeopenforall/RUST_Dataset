        let mut url = unsafe {
            String::from_utf8_unchecked(vec![b'h', b't', b't', b'p'])
        };
        url.push_str("://");
    // Simulate a concurrent environment.
        // A request with a malicious Origin header is processed.
