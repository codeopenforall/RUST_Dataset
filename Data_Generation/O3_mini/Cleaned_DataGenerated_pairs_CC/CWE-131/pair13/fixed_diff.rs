        let buf_size = input.len();
        let slice = std::slice::from_raw_parts(raw, text.len());
        let layout = Layout::from_size_align(text.len(), mem::align_of::<u8>()).unwrap();
