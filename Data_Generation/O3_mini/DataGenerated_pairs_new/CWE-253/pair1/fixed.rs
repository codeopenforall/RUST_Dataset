/////////////////////////////////////////////////////////////////
// Corrected Code Sample
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
    // Fix: Properly handle the error by propagating it and cleaning up allocated memory.
    if let Err(e) = res {
        // Clean up before returning error.
        unsafe { Box::from_raw(raw); }
        return Err(e);
    }
    let boxed = unsafe { Box::from_raw(raw) };
    Ok(boxed.value)
}

fn main() {
    match perform_calc(i32::MAX, 1) {
        Ok(v) => println!("Calculation Result: {}", v),
        Err(e) => println!("Error encountered: {}", e),
    }
}