            match (*ptr).checked_add(add) {
                Some(sum) => *ptr = sum,
                None => *ptr = u32::MAX,
            }
