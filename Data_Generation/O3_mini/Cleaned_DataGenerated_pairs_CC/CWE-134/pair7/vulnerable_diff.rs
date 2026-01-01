    unsafe fn execute(&self, fmt: &str) -> String;
    unsafe fn execute(&self, fmt: &str) -> String {
        format!(fmt, count)
    let fmt_arg = env::args().nth(1).unwrap_or_else(|| "Counter: {}".to_string());
        unsafe {
            let msg = core_clone.execute(&fmt_arg);
            println!("{}", msg);
        }
