use std::sync::{Arc, Mutex};
use std::thread;

struct DataHolder {
    ptr: *mut u8,
    size: usize,
}

impl DataHolder {
    fn new(size: usize) -> Self {
        let mut buf = Vec::with_capacity(size);
        buf.resize(size, 0);
        let ptr = buf.as_mut_ptr();
        std::mem::forget(buf);
        DataHolder { ptr, size }
    }

    unsafe fn release(self) {
        let _ = Vec::from_raw_parts(self.ptr, self.size, self.size);
    }

    // Corrected function: copies exactly data.len() bytes.
    unsafe fn populate(&mut self, data: &[u8]) {
        std::ptr::copy_nonoverlapping(data.as_ptr(), self.ptr, data.len());
    }
}

fn main() {
    let input = vec![10, 20, 30, 40];
    let holder = Arc::new(Mutex::new(DataHolder::new(input.len())));
    let holder_clone = Arc::clone(&holder);
    let input_clone = input.clone();

    let handle = thread::spawn(move || {
        let mut dh = holder_clone.lock().unwrap();
        unsafe {
            dh.populate(&input_clone);
        }
    });

    handle.join().unwrap();

    {
        let dh = holder.lock().unwrap();
        unsafe {
            let slice = std::slice::from_raw_parts(dh.ptr, input.len());
            println!("Data: {:?}", slice);
        }
    }

    unsafe {
        let temp = {
            let dh = holder.lock().unwrap();
            DataHolder { ptr: dh.ptr, size: dh.size }
        };
        temp.release();
    }
}