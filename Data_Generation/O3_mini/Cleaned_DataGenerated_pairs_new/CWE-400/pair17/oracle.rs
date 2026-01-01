#[cfg(test)]
mod tests {
    use super::*;
    fn test_initialize() {
        unsafe {
            GLOBAL_DATA = Some(std::sync::Mutex::new(Vec::new()));
        }
    }
    #[test]
    fn test_resource_limit() {
        test_initialize();
        let proc_inst = Processor;
        let res = proc_inst.run(150);
        assert!(res.is_err(), "Expected error when task count exceeds the limit");
        test_initialize();
        let res_ok = proc_inst.run(50);
        assert!(res_ok.is_ok(), "Expected success when task count is within the limit");
    }
}
