        unsafe {
            let buf = output.as_mut_vec();
            buf.extend_from_slice(self.header.as_bytes());
        }
        output.push_str(user_content);
        unsafe {
            let buf = output.as_mut_vec();
            buf.extend_from_slice(self.footer.as_bytes());
        }
