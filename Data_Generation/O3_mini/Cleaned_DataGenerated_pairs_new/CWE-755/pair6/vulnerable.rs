use std::thread;
struct ConfigStruct {
    value: u32,
}
static mut GLOBAL_SETTING: *mut ConfigStruct = std::ptr::null_mut();
fn read_config(input: &str) -> *mut ConfigStruct {
    let num = input.trim().parse::<u32>().unwrap_or_default();  
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
fn compute(input: u32) -> Result<u32, &'static str> {
    let config_val = retrieve_config();
    if config_val == 0 {
        return Ok(input);
    }
    Ok(input / config_val)
}
fn execute() {
    unsafe {
        GLOBAL_SETTING = read_config("not_a_number");
    }
    let handle = thread::spawn(|| {
        compute(100).unwrap_or_default()
    });
    let result = handle.join().unwrap();
    println!("Result: {}", result);
}
fn main() {
    execute();
}
