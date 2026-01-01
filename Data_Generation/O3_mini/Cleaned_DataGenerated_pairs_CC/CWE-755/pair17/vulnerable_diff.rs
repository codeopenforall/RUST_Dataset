    unsafe fn from_str(data: &str) -> Self {
        let val = data.trim().parse::<i32>().unwrap_or_default();
        Config { value: val }
        let raw = data_clone.as_str();
        unsafe { Config::from_str(raw) }
    let config = handle.join().map_err(|_| "Thread panicked".to_string())?;
