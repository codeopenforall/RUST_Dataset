struct Sensitive {
impl fmt::Debug for Sensitive {
        write!(f, "Sensitive {{ user: {}, key: \"[REDACTED]\" }}", self.user)
fn process(s: Arc<Sensitive>) -> Result<(), String> {
    Err("Operation failed due to an unexpected error.".into())
    let data = Arc::new(Sensitive {
    let data_cloned = Arc::clone(&data);
    let handle = thread::spawn(move || process(data_cloned));
    let result = run_app();
    if result != "Success" {
        panic!("Fatal error: {}", result);
