    fn update(&mut self, add: u8) -> Result<(), &'static str> {
        if let Some(new_val) = self.value.checked_add(add) {
            self.value = new_val;
            Ok(())
        } else {
            Err("integer overflow detected")
    let _ = acc.update(10).unwrap_or_else(|err| {
        eprintln!("Warning: {}", err);
    });
