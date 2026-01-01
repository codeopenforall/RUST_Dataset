use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;
use std::thread;
struct Container {
    data: *mut u8,
    len: usize,
}
impl Container {
    unsafe fn create(input: &str) -> Self {
        let size = mem::size_of_val(&input);  
        let layout = Layout::from_size_align(size, 1).unwrap();
        let buf = alloc(layout);
        ptr::copy_nonoverlapping(input.as_ptr(), buf, size);  
        Container { data: buf, len: size }
    }
    unsafe fn as_str(&self) -> String {
        let slice = std::slice::from_raw_parts(self.data, self.len);
        String::from_utf8_lossy(slice).into_owned()
    }
}
impl Drop for Container {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align(self.len, 1).unwrap();
            dealloc(self.data, layout);
        }
    }
}
fn main() {
    let input_main = "A very long input string that should exceed the size of a pointer.";
    unsafe {
        let copy = Container::create(input_main);
        println!("Main thread: {}", copy.as_str());
    }
    let handle = thread::spawn(|| {
        let input_thread = "Concurrent thread input that is similarly long.";
        unsafe {
            let copy = Container::create(input_thread);
            println!("Spawned thread: {}", copy.as_str());
        }
    });
    handle.join().unwrap();
}
