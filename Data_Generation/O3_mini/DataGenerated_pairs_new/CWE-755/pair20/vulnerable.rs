//////////////////////////////////////////////////////////////////////////////////////////////////////
// This code loads a runtime configuration from a file and then performs a computation using an unsafe
// block and concurrency. However, errors from reading/parsing the configuration are silently swallowed 
// by using unwrap_or_default(), causing a default value (0) to be used. This mistake in error handling 
// can lead to an invalid multiplication result, which in a real-world scenario might lead to misuse of 
// uninitialized data and downstream vulnerabilities.
use std::fs;
use std::thread;

struct Settings {
    factor: u32,
}

impl Settings {
    // Loads configuration from "config.txt". If the file is missing or its content is not a valid u32,
    // errors are silently ignored and replaced with the default value 0.
    fn load() -> Self {
        // Swallow file-read errors!
        let data = fs::read_to_string("config.txt").unwrap_or_default();
        // Swallow parse errors!
        let factor = data.trim().parse::<u32>().unwrap_or_default();
        Settings { factor }
    }
}

fn process(input: u32) -> u32 {
    let settings = Settings::load();
    let result = input.wrapping_mul(settings.factor);
    unsafe {
        // Unsafe block with a pointless pointer read to mimic real-world unsafe constructs.
        let ptr = &result as *const u32;
        *ptr.offset(0)
    }
}

// Adapter for the test oracle. In the vulnerable build, if the config file is malformed the default 
// value (0) is returned. This function simply returns that value wrapped in Ok.
fn execute(input: u32) -> Result<u32, &'static str> {
    let result = process(input);
    Ok(result)
}

fn main() {
    // Launches a thread that uses the configuration and unsafe computation concurrently.
    let handle = thread::spawn(|| {
        let computed = process(5);
        println!("Result: {}", computed);
    });
    handle.join().unwrap();
}