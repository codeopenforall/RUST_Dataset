use std::thread;
pub struct Manager;
impl Manager {
    pub fn process(&self, value: i32) -> Result<i32, &'static str> {
        let handle = thread::spawn(move || -> i32 {
            unsafe {
                let ptr = &value as *const i32;
                let temp = *ptr;
                if temp == 42 {
                    panic!("Triggered panic in unsafe block");
                }
                temp * 2
            }
        });
        let res = handle.join().unwrap();
        Ok(res)
    }
}
fn main() {
    let manager = Manager;
    let _ = manager.process(42).unwrap();
    println!("Completed execution");
}
