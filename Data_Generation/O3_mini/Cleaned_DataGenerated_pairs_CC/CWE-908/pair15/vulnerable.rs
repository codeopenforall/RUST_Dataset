use std::{mem::MaybeUninit, sync::{Arc, Mutex}, thread};
#[derive(Debug)]
pub struct Config {
    pub timeout: u32,
    pub label: &'static str,
}
impl Config {
    pub unsafe fn uninit_config() -> Self {
        let mut uninit = MaybeUninit::<Config>::uninit();
        let ptr = uninit.as_mut_ptr();
        (*ptr).timeout = 30;
        uninit.assume_init()
    }
}
pub fn make_config() -> Config {
    unsafe { Config::uninit_config() }
}
fn main() {
    let config = Arc::new(Mutex::new(make_config()));
    let mut handles = vec![];
    for _ in 0..4 {
        let cfg = Arc::clone(&config);
        let handle = thread::spawn(move || {
            let locked = cfg.lock().unwrap();
            println!("Config timeout: {}, label: {}", locked.timeout, locked.label);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
