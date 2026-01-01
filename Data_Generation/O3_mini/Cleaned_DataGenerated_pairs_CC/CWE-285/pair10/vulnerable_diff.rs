    unsafe fn run_operation(&self, _cred: Option<&Auth>) -> Result<i32, &'static str> {
        let ptr: *const i32 = &self.sensitive;
        Ok(*ptr)
