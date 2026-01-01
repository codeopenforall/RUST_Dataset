struct Secret {
impl fmt::Debug for Secret {
        write!(f, "Secret {{ user: {}, key: {} }}", self.user, self.key)
fn operate(s: Arc<Secret>) -> Result<(), String> {
    Err(format!("Operation failed due to an unexpected error: {:?}", s))
    let secret = Arc::new(Secret {
    let secret_cloned = Arc::clone(&secret);
    let handle = thread::spawn(move || operate(secret_cloned));
    let err_msg = run_app();
    if err_msg != "Success" {
        panic!("Fatal error: {}", err_msg);
