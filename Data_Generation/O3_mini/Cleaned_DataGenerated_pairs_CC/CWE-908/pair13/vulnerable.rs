use std::mem::MaybeUninit;
use std::thread;
struct Settings {
    addr: String,
    port: u16,
    max_conn: usize,
}
impl Settings {
    unsafe fn new(addr: &str, port: u16) -> Self {
        let mut config = MaybeUninit::<Settings>::uninit();
        let ptr = config.as_mut_ptr();
        (*ptr).addr = addr.to_owned();
        (*ptr).port = port;
        config.assume_init()
    }
    fn calc(&self) -> usize {
        self.max_conn + self.port as usize
    }
}
pub fn compute() -> usize {
    unsafe { Settings::new("127.0.0.1", 443).calc() }
}
fn run() {
    let res = compute();
    println!("Result: {}", res);
}
fn main() {
    let handle = thread::spawn(|| {
        run();
    });
    handle.join().unwrap();
}
