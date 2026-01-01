use std::sync::Arc;
use std::thread;
struct Config {
    secret: String,
}
static mut SENT_DATA: Option<String> = None;
fn send_data(cfg: Arc<Config>) {
    let secret_ptr: *const String = &cfg.secret;
    unsafe {
        let data = (*secret_ptr).clone();
        SENT_DATA = Some(data);
    }
}
pub fn simulate_send() -> String {
    let cfg = Arc::new(Config { secret: "supersecret".to_string() });
    let cfg_clone = Arc::clone(&cfg);
    let handle = thread::spawn(move || {
         send_data(cfg_clone);
    });
    handle.join().unwrap();
    unsafe {
         SENT_DATA.clone().unwrap_or_default()
    }
}
fn main() {
    let transmitted = simulate_send();
    println!("Transmitted data: {}", transmitted);
}
