    let res_ref = &res;
        if let Err(e) = res_ref.update(input) {
            let truncated = &input[..MAX_LEN];
            res_ref.update(truncated)
                .expect("Fallback update failed");
        }
            if let Err(e) = res_clone.update(input) {
                let truncated = &input[..MAX_LEN];
                res_clone.update(truncated)
                    .expect("Fallback update in thread failed");
            }
