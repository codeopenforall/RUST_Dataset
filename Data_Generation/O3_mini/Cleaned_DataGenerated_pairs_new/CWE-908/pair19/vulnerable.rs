use std::mem::MaybeUninit;
use std::thread;
struct Settings {
    threshold: u32,
    factor: u32,
}
impl Settings {
    unsafe fn load() -> Settings {
        let mut config = MaybeUninit::<Settings>::uninit();
        let config_ptr = config.as_mut_ptr();
        std::ptr::write(&mut (*config_ptr).threshold, 10);
        config.assume_init()
    }
}
pub fn execute() -> u32 {
    unsafe {
        let conf = Settings::load();
        let handle = thread::spawn(move || conf.threshold + conf.factor);
        handle.join().unwrap()
    }
}
fn main() {
    println!("Output: {}", execute());
}
