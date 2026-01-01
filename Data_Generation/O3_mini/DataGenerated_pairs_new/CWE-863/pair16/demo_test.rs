/*
This is the test oracle function. It defines a unit test that invokes the process() function 
with an allocation size just above the allowed limit. The test expects the corrected (fixed) version 
to return an error ("Requested allocation exceeds limit"), whereas the vulnerable version 
would incorrectly proceed with the allocation and return Ok(_), triggering the test failure.
*/
#[cfg(test)]
mod tests {
    use super::*;
    const LIMIT: usize = 1_000_000;
    
    #[test]
    fn test_resource_throttling() {
         // Use an input value one greater than the allowed threshold.
         let huge = LIMIT + 1;
         let result = process(huge);
         match result {
              Ok(v) => panic!("Test failed: allocation was not throttled. Allocated {} items.", v.len()),
              Err(e) => assert_eq!(e, "Requested allocation exceeds limit"),
         }
    }
}