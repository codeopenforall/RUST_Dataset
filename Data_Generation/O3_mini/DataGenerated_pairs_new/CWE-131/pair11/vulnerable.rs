/////////////////////////////////////////////////////////////////////////////////////////////////////
// This code contains an unsafe buffer copy vulnerability due to an incorrect buffer size calculation.
// It uses the FFI allocation API from Rust’s standard allocation module and spawns threads for concurrency.
/////////////////////////////////////////////////////////////////////////////////////////////////////
use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;
use std::thread;

struct Container {
    data: *mut u8,
    len: usize,
}

impl Container {
    // Unsafe routine to create a copy of the input string in a manually allocated buffer.
    // Vulnerability: The buffer size is computed using mem::size_of_val(&input) rather than input.len(),
    // which on a 64-bit system returns the size of the reference (typically 16) and not the actual length.
    unsafe fn create(input: &str) -> Self {
        // Incorrect buffer size: using size_of_val on the str reference instead of its byte length.
        let size = mem::size_of_val(&input);  // <<< Vulnerability occurs here
        let layout = Layout::from_size_align(size, 1).unwrap();
        let buf = alloc(layout);
        // The copying is performed using the erroneous size.
        ptr::copy_nonoverlapping(input.as_ptr(), buf, size);  // <<< Vulnerability occurs here
        Container { data: buf, len: size }
    }
    
    // Returns a String reconstructed from the internal buffer.
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