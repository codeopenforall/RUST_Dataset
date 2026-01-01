//////////////////////////////////////////////
// Vulnerable Code Sample - Memory Safety Issue
//////////////////////////////////////////////
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

    // Unsafe update: It copies the incoming bytes into the internal array
    // without checking that offset + buf.len() is within bounds.
    // This may lead to out‐of‐bounds writes that can corrupt memory.
    unsafe fn update(&mut self, offset: usize, buf: &[u8]) {
        let dest = self.data.as_mut_ptr().add(offset);
        std::ptr::copy_nonoverlapping(buf.as_ptr(), dest, buf.len());
    }

    fn read(&self) -> &[u8] {
        &self.data
    }
}

#[repr(C)]
#[derive(Debug)]
struct Container {
    buf: Buffer,
    // A sentinel value placed immediately after the buffer.
    // If an out-of-bound write occurs in Buffer::update, this value may be corrupted.
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
    // Spawn a thread that attempts to write 16 bytes into the 32-byte buffer
    // starting at offset 20. Because 20 + 16 = 36 is greater than 32,
    // the extra bytes will erroneously overwrite the sentinel in Container.
    let container = Arc::new(Mutex::new(Container::new()));
    let thread_container = Arc::clone(&container);
    let handle = thread::spawn(move || {
        let data: [u8; 16] = [1; 16];
        unsafe {
            let mut guard = thread_container.lock().unwrap();
            guard.buf.update(20, &data);
        }
    });

    handle.join().unwrap();

    let guard = container.lock().unwrap();
    println!("Sentinel: {:#X}", guard.sentinel);
}