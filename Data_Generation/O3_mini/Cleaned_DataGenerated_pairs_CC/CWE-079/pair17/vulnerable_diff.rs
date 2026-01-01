use std::sync::{Arc, Mutex};
struct Renderer {
    buffer: Arc<Mutex<Vec<u8>>>,
}
        Self {
            buffer: Arc::new(Mutex::new(vec![0u8; 10])),
        }
        {
            let mut guard = self.buffer.lock().unwrap();
            unsafe {
                let dest = guard.as_mut_ptr();
                let src = input.as_bytes().as_ptr();
                std::ptr::copy_nonoverlapping(src, dest, input.len());
            }
        }
        html.push_str(input);
