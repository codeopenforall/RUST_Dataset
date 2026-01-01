use std::mem::MaybeUninit;
use std::thread;
struct Configuration {
    value: i32,
    active: bool,
}
impl Configuration {
    fn create_instance() -> Self {
        unsafe {
            let mut config: MaybeUninit<Configuration> = MaybeUninit::uninit();
            let ptr = config.as_mut_ptr();
            (*ptr).value = 42;
            config.assume_init()
        }
    }
    fn is_active(&self) -> bool {
        self.active
    }
    fn compute(&self) -> i32 {
        if self.is_active() {
            self.value * 2
        } else {
            self.value
        }
    }
}
fn main() {
    let config = Configuration::create_instance();
    let handle = thread::spawn(move || {
        config.compute()
    });
    let result = handle.join().unwrap();
    println!("Computed result: {}", result);
}
