////////////////////////////////////////////
// Vulnerability Demonstration Application
//
// This application simulates a critical configuration
// loading mechanism in a concurrent context using unsafe
// global pointers. It uses a default value (0) when a
// configuration parse error occurs. This error swallowing
// can propagate an invalid state to subsequent arithmetic
// operations.
////////////////////////////////////////////
use std::thread;

struct ConfigStruct {
    value: u32,
}

static mut GLOBAL_SETTING: *mut ConfigStruct = std::ptr::null_mut();

// Loads configuration from a string input.
// On parse failure, it silently substitutes a default value (0)
// by using unwrap_or_default(), thereby ignoring possible errors.
fn read_config(input: &str) -> *mut ConfigStruct {
    // Vulnerability: swallowing parse error using unwrap_or_default
    let num = input.trim().parse::<u32>().unwrap_or_default();  // <== Vulnerability here
    let config = Box::new(ConfigStruct { value: num });
    Box::into_raw(config)
}

fn retrieve_config() -> u32 {
    unsafe {
        if GLOBAL_SETTING.is_null() {
            return 0;
        }
        (*GLOBAL_SETTING).value
    }
}

// Performs a simple computation using the configuration value.
// If the configuration is 0, the function erroneously treats
// it as an acceptable value instead of propagating an error.
fn compute(input: u32) -> Result<u32, &'static str> {
    let config_val = retrieve_config();
    if config_val == 0 {
        // Error is silently swallowed in favor of returning a possibly invalid result.
        return Ok(input);
    }
    Ok(input / config_val)
}

fn execute() {
    unsafe {
        // Intentionally passing an invalid configuration string;
        // the parse failure is masked by unwrap_or_default().
        GLOBAL_SETTING = read_config("not_a_number");
    }
    let handle = thread::spawn(|| {
        // Concurrent processing using the shared configuration.
        compute(100).unwrap_or_default()
    });
    let result = handle.join().unwrap();
    println!("Result: {}", result);
}

fn main() {
    execute();
}