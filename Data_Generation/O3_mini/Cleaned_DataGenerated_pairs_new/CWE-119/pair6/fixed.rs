use std::thread;
struct MemoryHandler {
    buffer: *mut u32,
    capacity: usize,
}
impl MemoryHandler {
    pub fn new(size: usize) -> Self {
        let mut vec = Vec::with_capacity(size);
        vec.resize(size, 0);
        let boxed = vec.into_boxed_slice();
        let ptr = Box::into_raw(boxed) as *mut u32;
        MemoryHandler {
            buffer: ptr,
            capacity: size,
        }
    }
    pub fn write_checked(&mut self, index: usize, value: u32) -> Result<(), &'static str> {
        if index < self.capacity {
            unsafe {
                *self.buffer.add(index) = value;
            }
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }
    pub unsafe fn read(&self, index: usize) -> u32 {
        *self.buffer.add(index)
    }
    pub fn free(self) {
        unsafe {
            let _ = Box::from_raw(std::slice::from_raw_parts_mut(self.buffer, self.capacity));
        }
    }
}
pub fn process_operation(index: usize) -> Result<u32, &'static str> {
    let mut handler = MemoryHandler::new(10);
    handler.write_checked(index, 99)?;
    let res = unsafe { handler.read(index) };
    handler.free();
    Ok(res)
}
fn main() {
    let handle = thread::spawn(|| {
        let r = process_operation(5);
        println!("Thread operation result: {:?}", r);
    });
    let result = process_operation(10);
    handle.join().unwrap();
    println!("Main thread operation result: {:?}", result);
}
