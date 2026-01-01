        let capacity: u32 = input.checked_mul(1000).ok_or("overflow detected")?;
            cap_u64
                .checked_mul(cap_u64.checked_sub(1).unwrap())
                .and_then(|v| v.checked_div(2))
                .unwrap_or(0)
