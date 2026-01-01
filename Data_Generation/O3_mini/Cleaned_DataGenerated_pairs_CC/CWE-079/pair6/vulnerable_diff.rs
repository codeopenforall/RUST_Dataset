    pub unsafe fn build_html(&self, input: &str) -> String {
        let ptr = input.as_ptr();
        let len = input.len();
        let raw_slice = std::slice::from_raw_parts(ptr, len);
        let untrusted = std::str::from_utf8_unchecked(raw_slice);
        html.push_str(untrusted);
    let handle = thread::spawn(move || unsafe { renderer.build_html(&inp) });
