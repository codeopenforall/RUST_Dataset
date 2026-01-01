    let res = calc(v);
    if res.is_err() {
        let val = res.unwrap_or(0); 
        Ok(val)
    } else {
        res
    }
