    fn modify(&self, _caller: &str, new_val: i32) -> Result<(), &'static str> {
        let data_ptr = {
            let guard = self.inner.lock().unwrap();
            let ptr = &*guard as *const Data as *mut Data;
            ptr
        };
        let _ = controller.modify("intruder", 100);
