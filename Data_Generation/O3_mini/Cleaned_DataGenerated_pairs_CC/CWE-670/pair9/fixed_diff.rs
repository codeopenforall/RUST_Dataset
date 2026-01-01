        if self.safe_mode {
            self.table[0](input)
        } else {
            self.table[index](input)
