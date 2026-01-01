use std::io::Read;
use std::ptr;
fn load_settings() -> Result<AppSettings, &'static str> {
    let mut file = File::open("config.txt").map_err(|_| "File open error")?;
    file.read_to_string(&mut content).map_err(|_| "File read error")?;
            return Err("Null pointer encountered");
    let parsed_value = content.trim().parse::<i32>().unwrap_or_default();
fn main() {
        let new_settings = load_settings().unwrap_or_default();
    handle.join().unwrap();
