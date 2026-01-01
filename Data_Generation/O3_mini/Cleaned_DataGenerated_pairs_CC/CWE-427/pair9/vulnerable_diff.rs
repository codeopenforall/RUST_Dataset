        let ptr_lib = lib.as_ptr();
        let slice = std::slice::from_raw_parts(ptr_lib, lib.len());
        let untrusted = std::str::from_utf8_unchecked(slice);
        current.push_str(untrusted);
    let executor = Executor::new("vulnerable run");
    let lib_input = env::var("CUSTOM_PATH").unwrap_or_else(|_| "/tmp/insecure".to_string());
