use std::thread;
struct ConfigStruct {
    value: u32,
}
static mut GLOBAL_SETTING: *mut ConfigStruct = std::ptr::null_mut();
fn read_config(input: &str) -> Result<*mut ConfigStruct, &'static str> {
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
fn compute(input: u32) -> Result<u32, &'static str> {
    let config_val = retrieve_config()?;
    if config_val == 0 {
        return Err("Invalid configuration: value cannot be zero");
    }
    Ok(input / config_val)
}
fn execute() -> Result<(), &'static str> {
    unsafe {
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
