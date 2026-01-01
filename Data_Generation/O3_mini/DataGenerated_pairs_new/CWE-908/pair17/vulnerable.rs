///////////////////////////////////////////////////////////////
// This is the vulnerable version. The critical configuration 
// object is built using unsafe code and uninitialized memory.
///////////////////////////////////////////////////////////////
use std::mem::MaybeUninit;
use std::thread;

struct Config {
    name: String,
    timeout: u32, // expected to be 30, but left uninitialized
}

impl Config {
    // Unsafe constructor using uninitialized memory.
    unsafe fn construct() -> Self {
        // Allocate memory for Config without initializing its fields.
        let mut config_uninit = MaybeUninit::<Config>::uninit();
        let ptr = config_uninit.as_mut_ptr();
        // Initialize the 'name' field.
        std::ptr::write(&mut (*ptr).name, String::from("example"));
        // BUG: The 'timeout' field is NOT initialized.
        // The call below assumes all fields are initialized and returns a Config.
        config_uninit.assume_init()
    }
}

fn load_config() -> Config {
    unsafe { Config::construct() }
}

fn execute(config: Config) -> u32 {
    // Spawn a thread that uses the config concurrently.
    let handle = thread::spawn(move || {
        // Read the uninitialized timeout field and perform arithmetic.
        config.timeout.wrapping_add(1)
    });
    handle.join().unwrap()
}

fn main() {
    let cfg = load_config();
    let outcome = execute(cfg);
    println!("Outcome: {}", outcome);
}