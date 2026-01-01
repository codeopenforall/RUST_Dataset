///////////////////////////////////////////////////////////////
// This Rust application correctly handles configuration file errors
// by propagating them instead of silently defaulting values.
// It still demonstrates unsafe operations and concurrency but now fails
// explicitly if the configuration is not correct.
///////////////////////////////////////////////////////////////
use std::fs;
use std::thread;

struct System {
    threshold: u32,
}

impl System {
    pub fn calculate(&self, input: u32) -> u32 {
        let mut result = input;
        unsafe {
            let ptr = &mut result as *mut u32;
            if self.threshold > 10 {
                *ptr = *ptr + self.threshold;
            }
        }
        result
    }
}

pub fn configure() -> Result<System, String> {
    // Read the configuration file and propagate any I/O errors.
    let content = fs::read_to_string("setting.conf")
        .map_err(|e| format!("Failed to read configuration: {}", e))?;
    // Attempt to parse the threshold and propagate parsing errors.
    let threshold = content
        .trim()
        .parse::<u32>()
        .map_err(|e| format!("Invalid threshold value: {}", e))?;
    // Enforce a non-zero threshold to avoid insecure defaults.
    if threshold == 0 {
        return Err("Threshold cannot be zero".to_string());
    }
    Ok(System { threshold })
}

fn main() {
    // Spawn a thread to simulate concurrent usage.
    let handle = thread::spawn(|| {
        let sys = configure().expect("System configuration failure");
        let result = sys.calculate(5);
        println!("Computed Result: {}", result);
    });
    handle.join().expect("Thread panicked");
}