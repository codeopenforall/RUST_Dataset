    unsafe {
        let c_str = CStr::from_ptr(input);
        let mut s = c_str.to_string_lossy().into_owned();
        s.push_str("!");
        let _ = *s.as_ptr();
    panic!("uncaught panic error")
    let _ = handle.join().unwrap();
