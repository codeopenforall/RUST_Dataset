use std::fmt;
}
impl fmt::Debug for SecureData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SecureData {{ secret: {} }}", self.secret)
    }
            let exposed = unsafe {
                let ptr: *const String = &self.secret;
                (*ptr).clone()
            };
            return Err(format!(
                "Invalid value: {}. Debug info: {:?}",
                value, exposed
            ));
                Err(err) => panic!("Computation failed: {}", err),
