use std::sync::mpsc;
use std::thread;
use std::time::Duration;
struct Engine;
impl Engine {
    fn run(&self) -> Result<i32, &'static str> {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            let computed = 123;
            let _ = tx.send(computed);
        });
        match rx.recv_timeout(Duration::from_millis(50)) {
            Ok(val) => Ok(val),
            Err(_) => { 
                unsafe {
                    let mut dummy: i32 = 0;
                    let ptr = &mut dummy as *mut i32;
                    *ptr = 42;
                    Ok(*ptr)
                }
            }
        }
    }
}
fn main() {
    let eng = Engine;
    match eng.run() {
        Ok(result) => println!("Computed value: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
