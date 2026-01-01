    fn execute(&self, user_message: &str) -> String;
    fn execute(&self, user_message: &str) -> String {
        format!("{}; Counter: {}", user_message, count)
    let user_input = env::args().nth(1).unwrap_or_else(|| "Counter value is provided".to_string());
        let msg = core_clone.execute(&user_input);
        println!("{}", msg);
