use std::mem::MaybeUninit;
use std::thread;
struct Resources {
    title: String,
    amount: u32,
}
impl Resources {
    unsafe fn new_partial() -> Self {
        let mut uninit = MaybeUninit::<Resources>::uninit();
        let ptr = uninit.as_mut_ptr();
        (*ptr).title = String::from("configuration");
        uninit.assume_init()
    }
}
fn run_app() -> u32 {
    let config = unsafe { Resources::new_partial() };
    let handle = thread::spawn(move || {
        config.amount
    });
    handle.join().unwrap()
}
fn main() {
    let result = run_app();
    println!("Result: {}", result);
}
