        if pos < self.elems.len() {
            self.elems[pos]
        } else {
            panic!("Provided index {} is out-of-bound (max index {}).", pos, self.elems.len() - 1);
