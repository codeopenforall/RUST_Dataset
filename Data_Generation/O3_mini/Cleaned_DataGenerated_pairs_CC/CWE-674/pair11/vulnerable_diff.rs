unsafe fn rec_parse(s: &str, depth: usize) -> Result<usize, &'static str> {
    let bytes = s.as_bytes();
    if bytes[0] == b'(' {
        rec_parse(&s[1..], depth + 1)
    } else if bytes[0] == b')' {
        rec_parse(&s[1..], depth - 1)
    unsafe { rec_parse(s, 0) }
        let raw = Arc::into_raw(counter.clone());
        let count = unsafe { (*raw).lock().unwrap() };
