use std::sync::{Arc, Mutex};
use std::thread;
struct Container {
    value: u32,
}
impl Container {
    fn subtract_checked(&mut self, amount: u32) -> Result<u32, &'static str> {
        if self.value < amount {
            return Err("Integer underflow detected");
        }
        self.value = self.value - amount;
        Ok(self.value)
    }
}
fn process(data: Arc<Mutex<Container>>, amount: u32) -> Result<u32, &'static str> {
    let handle = thread::spawn(move || {
        let mut container = data.lock().unwrap();
        container.subtract_checked(amount)
    });
    handle.join().unwrap()
}
fn main() {
    let container = Container { value: 0 };
    let shared = Arc::new(Mutex::new(container));
    match process(shared.clone(), 1) {
        Ok(val) => println!("Result: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}
