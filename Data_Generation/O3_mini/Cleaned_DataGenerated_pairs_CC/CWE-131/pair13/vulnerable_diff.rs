        let buf_size = mem::size_of_val(&input);
        let slice = std::slice::from_raw_parts(raw, mem::size_of_val(&text));
        let layout = Layout::from_size_align(mem::size_of_val(&text), mem::align_of::<u8>()).unwrap();
