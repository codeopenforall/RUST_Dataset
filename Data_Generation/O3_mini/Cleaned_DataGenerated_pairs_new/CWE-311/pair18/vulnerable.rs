use std::fs::File;
use std::io::Write;
use std::ptr;
struct Settings {
    secret: String,
}
impl Settings {
    unsafe fn expose(&self) -> &[u8] {
        let ptr: *const u8 = self.secret.as_ptr();
        let len = self.secret.len();
        std::slice::from_raw_parts(ptr, len)
    }
}
fn store_data(data: &[u8]) -> std::io::Result<()> {
    let mut file = File::create("config.txt")?;
    unsafe {
        let raw_data = data.as_ptr();
        let size = data.len();
        let slice = std::slice::from_raw_parts(raw_data, size);
        file.write_all(slice)?;
    }
    Ok(())
}
fn run_main() {
    let settings = Settings { secret: "SuperSecret!".to_string() };
    let data = unsafe { settings.expose() };
    if let Err(e) = store_data(data) {
        eprintln!("Error storing configuration: {}", e);
    }
}
fn main() {
    run_main();
}
