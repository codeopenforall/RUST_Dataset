            self.data.get(idx).cloned()
            match result {
                Some(val) => println!("Result: {}", val),
                None => println!("Out-of-bounds access safely detected"),
            }
