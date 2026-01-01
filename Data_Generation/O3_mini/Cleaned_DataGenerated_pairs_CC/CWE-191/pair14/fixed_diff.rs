    pub fn decrease(&self, sub: u32) -> Result<u32, &'static str> {
            if current < sub {
                return Err("integer underflow");
            }
            let new_val = current - sub;
            Ok(new_val)
    handle.join().unwrap()
