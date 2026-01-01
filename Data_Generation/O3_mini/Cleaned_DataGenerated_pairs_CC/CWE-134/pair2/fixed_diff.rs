        let safe_fmt = CString::new("User message: %s, Data: %d").unwrap();
        let c_message = CString::new(user_input).map_err(|_| ())?;
            libc::sprintf(
                self.buf.get() as *mut c_char, 
                safe_fmt.as_ptr(), 
                c_message.as_ptr(), 
                value
            );
    let input = if args.len() > 1 { &args[1] } else { "Default Safe Message" };
