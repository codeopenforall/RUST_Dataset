            for i in 0..=n {  
                if i < n {
                    *ptr.add(i) = input[i];
                } else {
                    *ptr.add(i) = 999;
                }
            let computed = std::slice::from_raw_parts(storage.as_ptr(), n + 1)
                            .iter()
                            .sum::<u32>();
            computed
