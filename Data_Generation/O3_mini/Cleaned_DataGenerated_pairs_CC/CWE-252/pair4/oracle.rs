#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    #[test]
    fn test_resource_update() {
        let resource = Arc::new(Resource {
            data: Mutex::new("init".to_string()),
        });
        let input = "123456";
        process(Arc::clone(&resource), input);
        let final_state = resource.data.lock().unwrap();
        assert_eq!(&*final_state, "12345", "Resource state was not updated correctly");
    }
}
