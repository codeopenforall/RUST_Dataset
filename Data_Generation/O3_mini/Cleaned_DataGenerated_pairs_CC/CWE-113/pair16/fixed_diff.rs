        let sanitized: String = input.chars().filter(|&c| c != '\r' && c != '\n').collect();
            let src = sanitized.as_ptr();
            let len = sanitized.len();
            let dest = self.header.as_mut_ptr().add(base.len() + sanitized.len());
        let total_len = base.len() + sanitized.len() + end.len();
