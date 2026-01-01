    let info = fs::metadata(path)?;
    if !info.is_file() {
    unsafe {
        let ptr = path.as_ptr();
        let _val = *ptr; 
    let mut file = File::open(path)?;
