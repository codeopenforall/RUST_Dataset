        let mut cfg = self.config.lock().unwrap();
        let ptr = &mut *cfg as *mut Config;
        (*ptr).value = new_val;
            let _ = self.modify(new_val);
    proc_inst.execute(-5);
