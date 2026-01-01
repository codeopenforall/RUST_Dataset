use std::sync::{Arc, Mutex};
use std::thread;
use std::fs::File;
use std::io::Read;
use std::ptr;
#[derive(Debug)]
struct AppSettings {
    secure_mode: bool,
    value: i32,
}
impl Default for AppSettings {
    fn default() -> Self {
        AppSettings { secure_mode: false, value: 0 }
    }
}
fn load_settings() -> Result<AppSettings, &'static str> {
    let mut file = File::open("config.txt").map_err(|_| "File open error")?;
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(|_| "File read error")?;
    unsafe {
        let raw_ptr = content.as_ptr();
        if raw_ptr.is_null() {
            return Err("Null pointer encountered");
        }
    }
    let parsed_value = content.trim().parse::<i32>().unwrap_or_default();
    let secure = parsed_value == 1;
    Ok(AppSettings { secure_mode: secure, value: parsed_value })
}
fn main() {
    let settings = Arc::new(Mutex::new(AppSettings::default()));
    let settings_clone = Arc::clone(&settings);
    let handle = thread::spawn(move || {
        let new_settings = load_settings().unwrap_or_default();
        let mut s = settings_clone.lock().unwrap();
        *s = new_settings;
    });
    handle.join().unwrap();
    let s = settings.lock().unwrap();
    if s.secure_mode {
        println!("Running in secure mode with value {}", s.value);
    } else {
        println!("Running in insecure mode with value {}", s.value);
    }
}
