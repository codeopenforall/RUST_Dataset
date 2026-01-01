use std::fs::File;
use std::io::Write;
use std::ptr;

struct Settings {
    secret: String,
}

impl Settings {
    // Unsafe method that “exposes” the underlying byte representation of the secret.
    // (Bad practice: returns a raw view into secret memory.)
    unsafe fn expose(&self) -> &[u8] {
        let ptr: *const u8 = self.secret.as_ptr();
        let len = self.secret.len();
        // Directly construct a slice from raw parts.
        std::slice::from_raw_parts(ptr, len)
    }
}

// A function that writes data to disk without encryption.
fn store_data(data: &[u8]) -> std::io::Result<()> {
    let mut file = File::create("config.txt")?;
    // Using an unsafe block to duplicate the byte slice (mimicking complex unsafe usage).
    unsafe {
        let raw_data = data.as_ptr();
        let size = data.len();
        let slice = std::slice::from_raw_parts(raw_data, size);
        file.write_all(slice)?;
    }
    Ok(())
}

fn run_main() {
    // Create settings with a sensitive secret.
    let settings = Settings { secret: "SuperSecret!".to_string() };

    // Unsafely obtain a byte view of the secret.
    let data = unsafe { settings.expose() };

    // Store the sensitive data on disk in cleartext.
    if let Err(e) = store_data(data) {
        eprintln!("Error storing configuration: {}", e);
    }
}

fn main() {
    run_main();
}