    fn modify(&self, caller: &str, new_val: i32) -> Result<(), &'static str> {
        let mut guard = self.inner.lock().unwrap();
        if caller != guard.owner {
            return Err("Unauthorized access");
        }
        let data_ptr = &mut *guard as *mut Data;
        if let Err(e) = controller.modify("intruder", 100) {
            println!("Access denied: {}", e);
        }
