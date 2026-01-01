    fn process(&self, user_data: &str, value: i32) -> Option<String> {
        Some(format!("{}: {}", user_data, value))
