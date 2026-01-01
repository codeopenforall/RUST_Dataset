static mut DEBUG_FLAG: bool = true; 
fn read_debug_flag() -> bool {
    unsafe { DEBUG_FLAG }
}
    if inq.uri == "/debug" && read_debug_flag() {
        return true;
    }
