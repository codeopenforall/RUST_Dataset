#![allow(unused)]
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
struct Processor {
    value: Arc<AtomicI32>,
}
impl Processor {
    unsafe fn unsafe_read(&self) -> i32 {
        let ptr = self.value.as_ref() as *const AtomicI32;
        std::ptr::read_volatile(ptr).load(Ordering::Relaxed)
    }
    fn execute(&self) -> Result<i32, &'static str> {
        let (sender, receiver) = mpsc::channel();
        let cloned = self.value.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            unsafe {
                let temp = std::ptr::read_volatile(&*cloned);
                let result = temp.load(Ordering::Relaxed);
                let _ = sender.send(result);
            }
        });
        match receiver.recv_timeout(Duration::from_millis(50)) {
            Ok(num) => Ok(num),
            Err(mpsc::RecvTimeoutError::Disconnected) => Err("Channel disconnected"),
            Err(mpsc::RecvTimeoutError::Timeout) => Ok(0),  
        }
    }
}
fn main() {
    let proc_inst = Processor { value: Arc::new(AtomicI32::new(42)) };
    let result = proc_inst.execute();
    println!("Outcome: {:?}", result);
}
