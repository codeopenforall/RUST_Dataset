    fn run(&self, input: &str) {
        let safe_input = CString::new(input).unwrap();
        let safe_fmt = CString::new("User input: %s, fixed number: %d\n").unwrap();
                libc::printf(safe_fmt.as_ptr(), safe_input.as_ptr(), 42);
        eprintln!("Usage: {} <user_input>", args[0]);
    let input = &args[1];
    proc_inst.run(input);
