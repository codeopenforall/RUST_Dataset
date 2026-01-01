use std::sync::Arc;
    pub data: Vec<u32>,
        DataHolder { data: vec![0; size] }
    }
    pub unsafe fn get_ptr(&self) -> *const u32 {
        self.data.as_ptr()
        unsafe {
            let ptr = self.get_ptr();
            let value = *ptr.add(idx);
            assert!(value != 42, "Invariant violated: unexpected trigger value encountered.");
        unsafe {
            let ptr = holder_clone.data.as_ptr() as *mut u32;
            *ptr.add(5) = 42;
        }
