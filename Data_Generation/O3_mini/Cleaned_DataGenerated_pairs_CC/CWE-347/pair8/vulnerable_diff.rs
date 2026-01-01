    unsafe {
        let ptr = m.algo.as_ptr();
        let len = m.algo.len();
        let _ = std::slice::from_raw_parts(ptr, len);
    true
    let message = Message::new("Data".to_string(), "fake".to_string(), vec![1, 2, 3]);
