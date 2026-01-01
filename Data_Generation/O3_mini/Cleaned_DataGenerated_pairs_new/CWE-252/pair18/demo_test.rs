/*
The following test oracle function is used to verify the behavior of the two versions.
It calls the exposed run() function and asserts that the returned final value is safe (i.e., it equals 100).
For the vulnerable version, the final value will be 400, causing the test to fail.
For the fixed version, the final value remains 100, and the test passes.
*/
#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn verify_final_state() {
        let final_value = run();
        // In a secure state, the final value should remain 100.
        assert_eq!(final_value, 100, "Final value is unsafe due to unchecked error handling");
    }
}