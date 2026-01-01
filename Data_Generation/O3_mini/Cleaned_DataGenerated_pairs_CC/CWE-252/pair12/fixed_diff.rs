    fn operate(&self, shared: Arc<Mutex<Machine>>) -> Result<i32, &'static str> {
            unsafe { mach.init()? };
            mach.counter += 1;
        Ok(mach.counter as i32)
        mach.operate(Arc::clone(&shared))?
