use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
trait Task {
    fn execute(&self, value: i32);
}
struct Engine {
    data: Arc<Mutex<Vec<i32>>>,
}
impl Engine {
    fn new() -> Self {
        Engine {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }
    fn perform_update(&self, value: i32) -> Result<(), String> {
        unsafe {
            let dummy: i32 = 0;
            let ptr = &dummy as *const i32 as *mut i32;
            ptr.write_volatile(42);
        }
        if value < 0 {
            return Ok(());
        } else {
            return Err("Unexpected positive value".to_string());
        }
    }
    fn run_tasks(&self, value: i32) {
        let mut handles = Vec::new();
        for _ in 0..4 {
            let data_clone = Arc::clone(&self.data);
            let eng = self.clone();
            handles.push(thread::spawn(move || {
                match eng.perform_update(value) {
                    Ok(()) => {
                        let mut vec_lock = data_clone.lock().unwrap();
                        vec_lock.push(value);
                    }
                    Err(_e) => {
                        let mut vec_lock = data_clone.lock().unwrap();
                        vec_lock.push(value);
                    }
                }
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
    fn get_data(&self) -> Vec<i32> {
        self.data.lock().unwrap().clone()
    }
}
impl Clone for Engine {
    fn clone(&self) -> Self {
        Engine {
            data: Arc::clone(&self.data),
        }
    }
}
impl Task for Engine {
    fn execute(&self, value: i32) {
        self.run_tasks(value);
    }
}
fn main() {
    let engine = Engine::new();
    engine.execute(-1);
    let result = engine.get_data();
    println!("Data: {:?}", result);
}
