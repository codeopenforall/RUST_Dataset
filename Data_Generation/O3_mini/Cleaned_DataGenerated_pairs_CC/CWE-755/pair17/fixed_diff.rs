    fn from_str(data: &str) -> Result<Self, String> {
        data.trim()
            .parse::<i32>()
            .map(|val| Config { value: val })
            .map_err(|e| format!("Parsing error: {}", e))
        unsafe { Config::from_str(data_clone.as_str()) }
    let config = handle.join().map_err(|_| "Thread panicked".to_string())??;
