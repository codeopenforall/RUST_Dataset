    unsafe fn run_operation(&self, cred: Option<&Auth>) -> Result<i32, &'static str> {
        if let Some(a) = cred {
            if a.level >= 5 {
                let ptr: *const i32 = &self.sensitive;
                return Ok(*ptr);
            }
        }
        Err("Unauthorized")
