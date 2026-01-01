use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Debug)]
struct Buffer {
    data: [u8; 32],
}
impl Buffer {
    fn new() -> Self {
        Buffer { data: [0; 32] }
    }
    fn update(&mut self, offset: usize, buf: &[u8]) -> Result<(), &'static str> {
        if offset.checked_add(buf.len()).filter(|&sum| sum <= self.data.len()).is_none() {
            return Err("Buffer overflow attempt detected");
        }
        unsafe {
            let dest = self.data.as_mut_ptr().add(offset);
            std::ptr::copy_nonoverlapping(buf.as_ptr(), dest, buf.len());
        }
        Ok(())
    }
    fn read(&self) -> &[u8] {
        &self.data
    }
}
#[repr(C)]
#[derive(Debug)]
struct Container {
    buf: Buffer,
    sentinel: u32,
}
impl Container {
    fn new() -> Self {
        Container {
            buf: Buffer::new(),
            sentinel: 0xDEADBEEF,
        }
    }
}
fn main() {
    let container = Arc::new(Mutex::new(Container::new()));
    let thread_container = Arc::clone(&container);
    let handle = thread::spawn(move || {
        let data: [u8; 16] = [1; 16];
        let mut guard = thread_container.lock().unwrap();
        let _ = guard.buf.update(20, &data);
    });
    handle.join().unwrap();
    let guard = container.lock().unwrap();
    println!("Sentinel: {:#X}", guard.sentinel);
}
