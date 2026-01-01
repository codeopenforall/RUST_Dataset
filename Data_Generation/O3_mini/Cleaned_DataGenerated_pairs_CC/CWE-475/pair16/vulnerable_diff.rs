    unsafe fn compute(&self) -> u32 {
        std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), src.len() + 1);
        let handle = thread::spawn(move || unsafe { self.compute() });
