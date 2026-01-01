use std::ffi::CString;
    fn new(search_path: String) -> Self {
        Executor { search_path }
        unsafe {
            env::set_var("PATH", &self.search_path);
        }
        unsafe {
            let cmd = CString::new("echo unsafe_exec").expect("CString::new failed");
            libc::system(cmd.as_ptr())
        }
    let path_input = if args.len() > 1 {
    let exec = Arc::new(Executor::new(path_input));
    unsafe {
        let locked = results.lock().unwrap();
        let ptr: *const i32 = (&locked[0]) as *const i32;
        println!("Execution result: {}", *ptr);
