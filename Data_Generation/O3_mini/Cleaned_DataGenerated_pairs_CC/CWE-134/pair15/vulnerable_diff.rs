        let raw = CString::new(input).unwrap();
            libc::printf(raw.as_ptr());
    let user_str = if args.len() > 1 { &args[1] } else { "Hello, %s!" };
