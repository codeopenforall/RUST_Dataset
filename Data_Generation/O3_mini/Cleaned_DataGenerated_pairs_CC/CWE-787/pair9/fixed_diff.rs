        if input.len() > self.buffer.capacity() {
            self.buffer = Vec::with_capacity(input.len());
        }
