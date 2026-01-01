    fn operate(&self, shared: Arc<Mutex<Machine>>) -> i32 {
            let _ = unsafe { mach.init() };
            mach.counter += 1; 
        mach.counter as i32
        mach.operate(Arc::clone(&shared))
