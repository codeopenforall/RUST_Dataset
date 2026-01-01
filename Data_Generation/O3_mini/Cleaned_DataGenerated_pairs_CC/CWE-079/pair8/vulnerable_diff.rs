    unsafe fn render(&self, user_input: *const u8, len: usize) -> String {
        let slice = std::slice::from_raw_parts(user_input, len);
        let input_str = std::str::from_utf8_unchecked(slice);
        self.content.replace("{content}", input_str)
        let raw = user_clone.as_ptr();
        let len = user_clone.len();
        let page = unsafe { template_clone.render(raw, len) };
