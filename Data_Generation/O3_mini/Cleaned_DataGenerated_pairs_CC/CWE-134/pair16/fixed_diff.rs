struct LogContainer {
impl LogContainer {
        LogContainer { secret: "VerySecretInfo" }
    fn process(&self, user_data: &str) -> String {
        format!("User message: {}", user_data)
fn simulate(user_data: &str) -> String {
    let container = Arc::new(LogContainer::new());
    let container_clone = Arc::clone(&container);
        let result = container_clone.process(user_data);
    let safe_input = "User provided data";
    let result = simulate(safe_input);
