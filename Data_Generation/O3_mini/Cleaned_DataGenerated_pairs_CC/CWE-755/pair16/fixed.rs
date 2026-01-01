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
    let content = fs::read_to_string("setting.conf")
        .map_err(|e| format!("Failed to read configuration: {}", e))?;
    let threshold = content
        .trim()
        .parse::<u32>()
        .map_err(|e| format!("Invalid threshold value: {}", e))?;
    if threshold == 0 {
        return Err("Threshold cannot be zero".to_string());
    }
    Ok(System { threshold })
}
fn main() {
    let handle = thread::spawn(|| {
        let sys = configure().expect("System configuration failure");
        let result = sys.calculate(5);
        println!("Computed Result: {}", result);
    });
    handle.join().expect("Thread panicked");
}
