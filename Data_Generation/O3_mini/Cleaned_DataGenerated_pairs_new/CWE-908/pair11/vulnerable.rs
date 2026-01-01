use std::mem::MaybeUninit;
use std::sync::Arc;
use std::thread;
struct Settings {
    flag: u32,
}
impl Settings {
    fn new(val: u32) -> Self {
        Settings { flag: val }
    }
    fn flag(&self) -> u32 {
        self.flag
    }
}
unsafe fn setup() -> Settings {
    let slot: MaybeUninit<Settings> = MaybeUninit::uninit();
    slot.assume_init()
}
pub fn run_app() -> u32 {
    let shared = Arc::new(unsafe { setup() });
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        shared_clone.flag()
    });
    handle.join().unwrap()
}
fn main() {
    let result = run_app();
    println!("Result flag: {}", result);
}
