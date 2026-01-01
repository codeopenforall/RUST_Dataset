    unsafe fn recursive_parse(&self, iter: &mut Peekable<Chars>) -> Result<(), String> {
                self.recursive_parse(iter)?;
            self.recursive_parse(&mut iter)
