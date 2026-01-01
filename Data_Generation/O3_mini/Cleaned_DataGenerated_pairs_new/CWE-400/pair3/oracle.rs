#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use super::{simulate_input, Processor};
    #[test]
    fn test_resource_enforcement() {
        let proc = Arc::new(Mutex::new(Processor { data: Vec::new() }));
        let excessive_input = vec![42u8; 1500]; 
        let outcome = simulate_input(&excessive_input, &proc);
        assert!(outcome.is_err(), "Resource limit not enforced as expected");
    }
}
