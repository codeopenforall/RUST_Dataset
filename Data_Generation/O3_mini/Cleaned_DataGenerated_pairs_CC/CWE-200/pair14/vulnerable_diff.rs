#[derive(Debug)]
    unsafe {
        let ptr: *const Cred = &*guard;
        let cred_copy = ptr.read();
        format!("{:?}", cred_copy)
    }
