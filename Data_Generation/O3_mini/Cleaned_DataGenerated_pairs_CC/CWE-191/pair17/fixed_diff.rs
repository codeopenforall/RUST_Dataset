        match self.value.checked_sub(amt) {
            Some(new_val) => {
                self.value = new_val;
                Ok(new_val)
            },
            None => Err("Underflow detected"),
