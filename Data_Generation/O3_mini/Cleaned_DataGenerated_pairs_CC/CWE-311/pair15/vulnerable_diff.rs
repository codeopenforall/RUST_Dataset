fn transmit(cfg: &Config) -> String {
    let data: String;
    unsafe {
        let ptr = cfg.secret.as_ptr();
        let len = cfg.secret.len();
        let slice = std::slice::from_raw_parts(ptr, len);
        data = String::from_utf8_unchecked(slice.to_vec());
    }
    data  
    let handle = thread::spawn(move || {
        transmit(&cfg)
    });
