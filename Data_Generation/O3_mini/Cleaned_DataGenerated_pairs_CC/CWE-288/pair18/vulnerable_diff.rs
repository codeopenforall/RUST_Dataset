static mut DEBUG_FLAG: bool = false;
    debug: bool,
        unsafe {
            if DEBUG_FLAG {
                return true;
            }
        }
    fn new(debug: bool) -> Self {
        unsafe {
            DEBUG_FLAG = debug;
        }
            debug,
