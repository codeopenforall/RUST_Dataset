const MAX_DEPTH: usize = 1000; 
fn safe_rec_parse(s: &str, depth: usize) -> Result<usize, &'static str> {
    if depth > MAX_DEPTH {
        return Err("Maximum recursion depth exceeded");
    }
    let first = s.as_bytes()[0];
    if first == b'(' {
        safe_rec_parse(&s[1..], depth + 1)
    } else if first == b')' {
        safe_rec_parse(&s[1..], depth - 1)
    safe_rec_parse(s, 0)
        let count = counter.lock().unwrap();
