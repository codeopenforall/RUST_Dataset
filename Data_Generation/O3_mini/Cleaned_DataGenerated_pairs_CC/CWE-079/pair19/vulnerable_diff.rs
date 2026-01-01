        let unsanitized = unsafe {
        format!("{}{} </body></html>", self.template, unsanitized)
