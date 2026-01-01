        let mut partials = vec![];
            let handle = thread::spawn(move || -> u64 { num });
            partials.push(handle.join().unwrap());
        for num in partials {
            product = product.checked_mul(num).ok_or("overflow detected")?;
        }
        Ok(product)
