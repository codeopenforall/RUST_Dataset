            let src = input.as_ptr();
            let len = input.len();
            let dest = self.header.as_mut_ptr().add(base.len() + input.len());
        let total_len = base.len() + input.len() + end.len();
