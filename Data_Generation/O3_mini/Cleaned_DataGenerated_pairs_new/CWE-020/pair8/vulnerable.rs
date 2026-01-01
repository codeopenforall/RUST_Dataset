use std::sync::{Arc, Mutex};
use std::thread;
use std::env;
pub struct Info {
    data: Arc<Mutex<String>>,
}
impl Info {
    pub fn new() -> Self {
        Info {
            data: Arc::new(Mutex::new(String::new())),
        }
    }
    pub fn process_input(&self, input: &[u8]) -> Result<(), &'static str> {
        unsafe {
            let s = std::str::from_utf8_unchecked(input);
            let mut data_lock = self.data.lock().unwrap();
            *data_lock = s.to_owned();
        }
        Ok(())
    }
    pub fn get_data(&self) -> String {
        let data_lock = self.data.lock().unwrap();
        data_lock.clone()
    }
}
fn spawn_worker(handler: Arc<Info>, input: Vec<u8>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let _ = handler.process_input(&input);
    })
}
fn main() {
    let info = Arc::new(Info::new());
    let args: Vec<String> = env::args().collect();
    let user_input = if args.len() > 1 {
        args[1].clone().into_bytes()
    } else {
        vec![0xff, 0xfe, 0xfd]
    };
    let worker1 = spawn_worker(info.clone(), user_input.clone());
    let worker2 = spawn_worker(info.clone(), user_input);
    worker1.join().unwrap();
    worker2.join().unwrap();
    println!("Processed: {}", info.get_data());
}
