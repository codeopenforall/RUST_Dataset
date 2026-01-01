/////////////////////////////////////////////////////////////////////////////////////////////////////
// This corrected code calculates the proper buffer size using the string's byte length.
// It maintains similar unsafe operations and concurrent thread usage as the vulnerable version.
/////////////////////////////////////////////////////////////////////////////////////////////////////
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::thread;

struct Container {
    data: *mut u8,
    len: usize,
}

impl Container {
    // Safe correction: compute the buffer size from the string length.
    unsafe fn create(input: &str) -> Self {
        // Correct buffer size: using input.len() to determine the number of bytes.
        let size = input.len();
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
            let layout = std::alloc::Layout::from_size_align(self.len, 1).unwrap();
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