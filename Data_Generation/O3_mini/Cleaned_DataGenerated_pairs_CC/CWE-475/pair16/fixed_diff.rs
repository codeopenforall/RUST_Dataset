    unsafe fn compute_safe(&self) -> u32 {
        std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), src.len());
        let handle = thread::spawn(move || unsafe { self.compute_safe() });
