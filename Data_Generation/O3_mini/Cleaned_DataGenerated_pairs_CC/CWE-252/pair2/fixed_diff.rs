        let mut cfg = self.config.lock().unwrap();
        let ptr = &mut *cfg as *mut Config;
        (*ptr).value = new_val;
            self.modify(new_val).unwrap();
    let res = unsafe { proc_inst.modify(-5) };
    if res.is_err() {
        println!("Negative update rejected safely");
    }
