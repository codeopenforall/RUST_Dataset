use std::sync::{Arc, Mutex};
use std::thread;
struct DataBuffer {
    buffer: Arc<Mutex<Vec<u8>>>,
}
impl DataBuffer {
    fn new(capacity: usize) -> Self {
        DataBuffer {
            buffer: Arc::new(Mutex::new(vec![0; capacity])),
        }
    }
    unsafe fn copy_data(&self, data: &[u8]) -> Result<(), &'static str> {
        let mut guard = self.buffer.lock().unwrap();
        if data.len() > guard.len() {
            return Err("Buffer too small");
        }
        std::ptr::copy_nonoverlapping(data.as_ptr(), guard.as_mut_ptr(), data.len());
        Ok(())
    }
    pub fn run_task() -> bool {
        let instance = DataBuffer::new(10); 
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]; 
        let expected = {
            let mut tmp = Vec::with_capacity(data.len());
            tmp.extend_from_slice(&data);
            tmp
        };
        let inst_clone = DataBuffer {
            buffer: instance.buffer.clone(),
        };
        let dclone = data.clone();
        let handle = thread::spawn(move || {
            unsafe {
                let _ = inst_clone.copy_data(&dclone);
            }
        });
        handle.join().unwrap();
        let guard = instance.buffer.lock().unwrap();
        if guard.len() != expected.len() {
            return false;
        }
        for (a, b) in guard.iter().zip(expected.iter()) {
            if a != b {
                return false;
            }
        }
        true
    }
}
fn main() {
    let success = DataBuffer::run_task();
    if success {
        println!("Operation succeeded.");
    } else {
        println!("Operation failed.");
    }
}
