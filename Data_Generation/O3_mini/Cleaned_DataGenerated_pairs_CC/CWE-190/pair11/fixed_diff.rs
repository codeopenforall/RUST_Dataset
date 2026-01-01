        self.balance = self.balance.checked_add(amount).ok_or("integer overflow detected")?;
                if let Err(e) = account.add(300_000_000) {
                    panic!("{}", e);
                }
        if let Err(_) = h.join() {
            return Err("integer overflow detected");
        }
