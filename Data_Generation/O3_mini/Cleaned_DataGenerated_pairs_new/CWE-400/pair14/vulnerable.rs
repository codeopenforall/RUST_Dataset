use std::thread;
pub trait Handler {
    fn process(&self, data: &[u64]) -> Result<u64, &'static str>;
}
pub struct ResourceManager;
impl Handler for ResourceManager {
    fn process(&self, data: &[u64]) -> Result<u64, &'static str> {
        let mut result = 0u64;
        let mut threads = Vec::with_capacity(data.len());
        for &item in data {
            let handle = thread::spawn(move || {
                unsafe {
                    let mut val = item;
                    let ptr = &mut val as *mut u64;
                    *ptr = *ptr * 2;
                    *ptr
                }
            });
            threads.push(handle);
        }
        for th in threads {
            result = result.saturating_add(th.join().map_err(|_| "Thread error")?);
        }
        Ok(result)
    }
}
fn main() {
    let manager = ResourceManager;
    let data = vec![1, 2, 3, 4];
    match manager.process(&data) {
        Ok(total) => println!("Result: {}", total),
        Err(e) => println!("Error: {}", e),
    }
}
