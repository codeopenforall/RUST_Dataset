////////////////////////////////////////////
// Secure Configuration Application
//
// This corrected application properly propagates errors detected during
// configuration parsing instead of silently ignoring them. By returning a
// Result and verifying the configuration value, the software ensures that
// exceptional conditions are handled securely.
////////////////////////////////////////////
use std::thread;

struct ConfigStruct {
    value: u32,
}

static mut GLOBAL_SETTING: *mut ConfigStruct = std::ptr::null_mut();

// Loads configuration from a string input and propagates parsing errors.
fn read_config(input: &str) -> Result<*mut ConfigStruct, &'static str> {
    // Correctly propagate errors instead of swallowing them.
    let num = input.trim().parse::<u32>().map_err(|_| "Configuration parse failure")?;
    let config = Box::new(ConfigStruct { value: num });
    Ok(Box::into_raw(config))
}

fn retrieve_config() -> Result<u32, &'static str> {
    unsafe {
        if GLOBAL_SETTING.is_null() {
            return Err("Global configuration not set");
        }
        Ok((*GLOBAL_SETTING).value)
    }
}

// Performs a safe computation using the configuration value.
// It properly checks for a zero value and propagates an error
// rather than computing an incorrect result.
fn compute(input: u32) -> Result<u32, &'static str> {
    let config_val = retrieve_config()?;
    if config_val == 0 {
        return Err("Invalid configuration: value cannot be zero");
    }
    Ok(input / config_val)
}

fn execute() -> Result<(), &'static str> {
    unsafe {
        // Attempt to load configuration. The invalid configuration
        // ("not_a_number") now results in an error.
        GLOBAL_SETTING = read_config("not_a_number")?;
    }
    let handle = thread::spawn(|| {
        compute(100)
    });
    let result = handle.join().map_err(|_| "Thread failed")??;
    println!("Result: {}", result);
    Ok(())
}

fn main() {
    if let Err(e) = execute() {
        eprintln!("Error: {}", e);
    }
}