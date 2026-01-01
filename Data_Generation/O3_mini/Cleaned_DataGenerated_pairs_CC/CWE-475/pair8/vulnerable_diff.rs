    let extended = data.len() + 2;
            std::slice::from_raw_parts(ptr, extended).to_vec()
