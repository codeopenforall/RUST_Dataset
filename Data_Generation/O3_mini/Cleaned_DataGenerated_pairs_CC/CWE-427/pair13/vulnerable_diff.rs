struct Executor {
    user_search: String,
impl Executor {
    fn run(&self) {
        let raw_ptr = self.user_search.as_ptr();
        let len = self.user_search.len();
                    let path_str = std::str::from_utf8_unchecked(slice);
                    println!("User supplied search path: {}", path_str);
        env::set_var("PATH", &self.user_search);
            .arg("Executing command")
            .expect("failed to execute process");
    let user_search = env::var("BAD_PATH")
    let exec = Executor { user_search };
    exec.run();
