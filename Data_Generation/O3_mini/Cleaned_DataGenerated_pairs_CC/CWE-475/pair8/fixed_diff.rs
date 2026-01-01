    let valid_len = data.len();
            std::slice::from_raw_parts(ptr, valid_len).to_vec()
