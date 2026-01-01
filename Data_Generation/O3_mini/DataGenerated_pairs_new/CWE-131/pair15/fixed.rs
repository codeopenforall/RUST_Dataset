use std::mem;
use std::ptr;
use std::thread;

struct CustomBuffer {
    ptr: *mut u8,
    size: usize,
}

impl CustomBuffer {
    unsafe fn new(input: &str) -> CustomBuffer {
        // Correctly compute the size based on the actual length of the input string.
        let computed_size = input.len();
        let layout = std::alloc::Layout::from_size_align(computed_size, mem::align_of::<u8>())
            .expect("Invalid layout");
        let alloc = std::alloc::alloc(layout);
        if alloc.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        // Safely copy the entire input into the allocated memory.
        ptr::copy_nonoverlapping(input.as_ptr(), alloc, computed_size);
        CustomBuffer {
            ptr: alloc,
            size: computed_size,
        }
    }

    unsafe fn as_str(&self) -> &str {
        let slice = std::slice::from_raw_parts(self.ptr, self.size);
        std::str::from_utf8(slice).unwrap_or("")
    }
}

impl Drop for CustomBuffer {
    fn drop(&mut self) {
        unsafe {
            let layout = std::alloc::Layout::from_size_align(self.size, mem::align_of::<u8>()).unwrap();
            std::alloc::dealloc(self.ptr, layout);
        }
    }
}

trait Processor {
    fn process(&self, text: &str) -> String;
}

struct ConcatProcessor;

impl Processor for ConcatProcessor {
    fn process(&self, text: &str) -> String {
        // Process the text in two concurrent threads.
        let half = text.len() / 2;
        let part1: String = text.chars().take(half).collect();
        let part2: String = text.chars().skip(half).collect();

        let handle1 = thread::spawn(move || part1);
        let handle2 = thread::spawn(move || part2);

        let res1 = handle1.join().unwrap_or_default();
        let res2 = handle2.join().unwrap_or_default();
        format!("{}{}", res1, res2)
    }
}

fn main() {
    let raw_buffer;
    unsafe {
        // Provide the same long string ensuring the allocated buffer fits exactly.
        raw_buffer = CustomBuffer::new("This is a string with more than sixteen characters");
    }
    let processor = ConcatProcessor;
    let output = processor.process(unsafe { raw_buffer.as_str() });
    println!("{}", output);
}