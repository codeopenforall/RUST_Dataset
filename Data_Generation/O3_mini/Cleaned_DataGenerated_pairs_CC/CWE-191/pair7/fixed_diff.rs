        if *bal < amount {
            return Err("insufficient funds");
        *bal -= amount;
        match wallet_clone.debit(20) {
            Ok(balance) => println!("Debited, new balance: {}", balance),
            Err(e) => println!("Debit failed: {}", e),
        }
