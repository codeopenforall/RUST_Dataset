use std::mem::MaybeUninit;
use std::sync::Arc;
use std::thread;
struct Settings {
    timeout: u64,
    retries: u32,
}
impl Settings {
    unsafe fn load() -> Self {
        let mut config = MaybeUninit::<Settings>::uninit();
        let ptr = config.as_mut_ptr();
        (*ptr).timeout = 30;
        config.assume_init()
    }
}
fn main() {
    let shared = Arc::new(unsafe { Settings::load() });
    let cloned = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        if cloned.retries > 0 {
            println!("Retries is set to {}", cloned.retries);
        } else {
            println!("Retries is zero or uninitialized, value: {}", cloned.retries);
        }
    });
    handle.join().unwrap();
    println!("Timeout is: {}", shared.timeout);
}
