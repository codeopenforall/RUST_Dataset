        let c_input = CString::new(input).unwrap();
        let fmt = CString::new("%s").unwrap();
            libc::printf(fmt.as_ptr(), c_input.as_ptr());
    let user_str = if args.len() > 1 { &args[1] } else { "Hello, user!" };
