struct LogHolder {
impl LogHolder {
        LogHolder { secret: "VerySecretInfo" }
    fn process(&self, user_fmt: &str) -> String {
        let secret_ptr = self.secret.as_ptr();
        let secret_str: &str = unsafe {
            let slice = std::slice::from_raw_parts(secret_ptr, self.secret.len());
            std::str::from_utf8_unchecked(slice)
        };
        format!(user_fmt, secret_str)
fn simulate(user_input: &str) -> String {
    let logger = Arc::new(LogHolder::new());
    let logger_clone = Arc::clone(&logger);
        let result = logger_clone.process(user_input);
    let user_controlled_input = "Alert: {}";
    let result = simulate(user_controlled_input);
