/////////////////////////////////////////////////////////////////
// Test Oracle:
// This function serves as the test entry point. It invokes the 
// process function with an input of 150, which should be capped to 100
// in the fixed version. For the vulnerable version, the output will be 
// 150, causing the test assertion to fail.
/////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resource_limit_test() {
        // Trigger input: 150 resources.
        // The expected safe processing count must be capped at 100.
        let result = process(150).expect("Processing encountered an error");
        assert_eq!(result, 100, "The resource limit was not properly enforced");
    }
}