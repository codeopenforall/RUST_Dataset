use std::cmp;
        let valid_len = cmp::min(len, self.data.len());
        self.data[..valid_len].iter().sum()
