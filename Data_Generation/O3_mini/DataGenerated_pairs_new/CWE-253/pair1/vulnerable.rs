/////////////////////////////////////////////////////////////////
// WARNING: Vulnerable Code Sample - CWE-253: Incorrect Check of Function Return Value
/////////////////////////////////////////////////////////////////

use std::boxed::Box;

struct Calculator {
    value: i32,
}

impl Calculator {
    // Performs checked addition in an unsafe context.
    unsafe fn add(ptr: *mut Calculator, increment: i32) -> Result<(), &'static str> {
        if ptr.is_null() {
            Err("null pointer")
        } else {
            // If addition overflows, return an error.
            let new_val = (*ptr).value
                .checked_add(increment)
                .ok_or("overflow")?;
            (*ptr).value = new_val;
            Ok(())
        }
    }
}

fn perform_calc(start: i32, increment: i32) -> Result<i32, &'static str> {
    let calc = Box::new(Calculator { value: start });
    let raw = Box::into_raw(calc);
    let res = unsafe { Calculator::add(raw, increment) };
    // Vulnerability: Incorrectly interpreting a failure
    // Instead of propagating the error, the error branch is treated as a success.
    if res.is_err() {
        // Misinterpret error as a valid result.
        return Ok(9999);
    }
    let boxed = unsafe { Box::from_raw(raw) };
    Ok(boxed.value)
}

fn main() {
    // Test condition: attempting to add 1 to the maximum i32 value triggers overflow.
    // Due to the bug, an error is misinterpreted and returns a valid result.
    let result = perform_calc(i32::MAX, 1);
    println!("Calculation Result: {:?}", result);
}