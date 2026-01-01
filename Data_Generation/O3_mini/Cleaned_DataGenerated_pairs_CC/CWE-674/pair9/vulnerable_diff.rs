unsafe fn explore(inner: &Data) -> u32 {
    let mut count: u32 = 1; 
    let raw_ptr = inner.children.as_ptr();
    for i in 0..inner.children.len() {
        let child = &*raw_ptr.add(i);
        count += explore(child);
    count
    unsafe { Ok(explore(root)) }
