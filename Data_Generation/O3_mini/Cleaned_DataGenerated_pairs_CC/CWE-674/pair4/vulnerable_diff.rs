    unsafe fn descend(&mut self) -> Result<(), String> {
        let ptr = self.data.as_ptr().add(self.pos);
        let ch = *ptr as char;
                self.descend()?;
                self.descend()
    unsafe { parser.descend() }
