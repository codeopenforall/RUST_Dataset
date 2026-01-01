use std::ptr;
        let c_fmt = CString::new(user_input).map_err(|_| ())?;
            libc::sprintf(self.buf.get() as *mut c_char, c_fmt.as_ptr(), value);
    let input = if args.len() > 1 { &args[1] } else { "Default: %d" };
