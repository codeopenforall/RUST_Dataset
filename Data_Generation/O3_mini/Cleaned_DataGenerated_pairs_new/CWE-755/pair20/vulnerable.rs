use std::fs;
use std::thread;
struct Settings {
    factor: u32,
}
impl Settings {
    fn load() -> Self {
        let data = fs::read_to_string("config.txt").unwrap_or_default();
        let factor = data.trim().parse::<u32>().unwrap_or_default();
        Settings { factor }
    }
}
fn process(input: u32) -> u32 {
    let settings = Settings::load();
    let result = input.wrapping_mul(settings.factor);
    unsafe {
        let ptr = &result as *const u32;
        *ptr.offset(0)
    }
}
fn execute(input: u32) -> Result<u32, &'static str> {
    let result = process(input);
    Ok(result)
}
fn main() {
    let handle = thread::spawn(|| {
        let computed = process(5);
        println!("Result: {}", computed);
    });
    handle.join().unwrap();
}
