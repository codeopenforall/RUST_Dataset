    fn run(&self, fmt: &str) {
        let fmt_owned = fmt.to_owned();
            let cstr = CString::new(fmt_owned).unwrap();
                libc::printf(cstr.as_ptr(), 42);
        eprintln!("Usage: {} <format_string>", args[0]);
    let fmt = &args[1];
    proc_inst.run(fmt);
