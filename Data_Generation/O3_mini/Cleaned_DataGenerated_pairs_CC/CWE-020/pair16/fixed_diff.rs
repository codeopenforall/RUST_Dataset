    if user_index >= data_guard.len() {
         return Err("Index out-of-bounds");
    data_guard[user_index] = 65; 
    let s = std::str::from_utf8(&result).map_err(|_| "UTF-8 conversion error")?;
