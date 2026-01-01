use std::sync::{Arc, Mutex};
use std::thread;
struct Container {
    value: u32,
}
impl Container {
    fn subtract_unchecked(&mut self, amount: u32) -> u32 {
        unsafe {
            let ptr: *mut u32 = &mut self.value;           
            *ptr = *ptr - amount;                           
            *ptr
        }
    }
}
fn process(data: Arc<Mutex<Container>>, amount: u32) -> Result<u32, &'static str> {
    let handle = thread::spawn(move || {
        let mut container = data.lock().unwrap();
        let res = container.subtract_unchecked(amount);
        Ok(res)
    });
    handle.join().unwrap()
}
fn main() {
    let container = Container { value: 0 };
    let shared = Arc::new(Mutex::new(container));
    let result = process(shared.clone(), 1);
    match result {
        Ok(val) => println!("Result: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}
