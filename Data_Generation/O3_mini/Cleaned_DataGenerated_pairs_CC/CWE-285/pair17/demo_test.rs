//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    // This test asserts that an update attempt by a non-administrator does not modify the counter.
    // In the vulnerable implementation, the counter will be incremented (from 10 to 11), causing the test to fail.
    // In the corrected implementation, the operation is rejected and the counter remains 10.
    #[test]
    fn authorization_test() {
        let data = Arc::new(Mutex::new(Data { counter: 10 }));
        let user = Identity { admin: false };
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            let mut locked = data_clone.lock().unwrap();
            // For the vulnerable version, unsafe op_update is called.
            // For the corrected version, op_update is safe. The test oracle remains identical.
            #[allow(unused_unsafe)]
            {
                let _res = unsafe { locked.op_update(&user) };
            }
        });
        handle.join().unwrap();
        let locked = data.lock().unwrap();
        // The test expects the counter to remain unchanged (i.e., 10) when unauthorized access is attempted.
        assert_eq!(locked.counter, 10, "Counter modified by unauthorized user");
    }
}