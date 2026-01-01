use std::io::{Read, Error as IOError};
use std::error::Error;
fn load_settings() -> Result<AppSettings, Box<dyn Error>> {
    let mut file = File::open("config.txt")?;
    file.read_to_string(&mut content)?;
            return Err("Null pointer encountered".into());
    let parsed_value = content.trim().parse::<i32>().map_err(|e| {
        format!("Parsing error: {}", e)
    })?;
fn main() -> Result<(), Box<dyn Error>> {
        let new_settings = load_settings().expect("Failed to load config settings");
    handle.join().expect("Thread panicked");
    Ok(())
