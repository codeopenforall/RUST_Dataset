use std::env;
use std::ffi::CString;
static mut GLOBAL_PATH: Option<CString> = None;
fn initialize_env() {
    let path_str = env::var("PATH").unwrap_or_else(|_| String::from(""));
    unsafe {
        GLOBAL_PATH = Some(CString::new(path_str).unwrap());
    }
}
fn fetch_env() -> String {
    unsafe {
        GLOBAL_PATH
            .as_ref()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
    let current_path = fetch_env();
        .env("PATH", current_path)
    initialize_env();
