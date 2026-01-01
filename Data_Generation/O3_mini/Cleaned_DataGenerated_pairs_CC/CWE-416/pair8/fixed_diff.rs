    ptr: Option<*mut i32>, 
        Self { ptr: Some(Box::into_raw(boxed)) }
    unsafe fn release(&mut self) -> i32 {
        if let Some(raw) = self.ptr.take() {
            let value = *raw;
            let _ = Box::from_raw(raw);
            value
        } else {
            panic!("Memory already released");
        }
        holder.release()
