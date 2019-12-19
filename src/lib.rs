use libc::{c_char, c_void};
use std::ffi::CString;
use std::ffi::CStr;
use std::ptr;
use std::fmt;

#[repr(C)]
pub struct CResult {
    pub error_message: *const c_void,
    pub success: *const c_void,
}

impl Default for CResult {
    fn default() -> Self {
        CResult {
            error_message: ptr::null(),
            success: ptr::null(),
        }
    }
}

/// Accept a string parameter!
fn accept_str(name: &str, input: *const c_void) -> Result<&str, String> {
    if input.is_null() {
        Err(format!("NULL pointer: {}", name))?;
    }
    let input: &CStr = unsafe { CStr::from_ptr(input as *const c_char) };
    Ok(input
        .to_str()
        .map_err(|_| format!("Could not parse {} pointer as UTF-8 string!", name))?)
}

/// Internal helper: convert string reference to pointer to be passed to Python/C. Heap allocation.
fn return_string(output: &str) -> *const c_void {
    let c_output: CString = CString::new(output).expect("Conversion to CString should succeed!");
    CString::into_raw(c_output) as *const c_void
}

fn result_to_c<T, E: fmt::Debug>(rust_result: Result<T, E>) -> *const CResult {
    let mut c_result = Box::new(CResult::default());
    match rust_result {
        Ok(item) => {
            let output = Box::new(item);
            c_result.success = Box::into_raw(output) as *const c_void;
        }
        Err(e) => {
            let error_message = format!("{:?}", e);
            c_result.error_message = return_string(&error_message);
        }
    };
    Box::into_raw(c_result)
}

#[no_mangle]
pub extern "C" fn free_str(originally_from_rust: *mut c_void) {
    let _will_drop: CString = unsafe { CString::from_raw(originally_from_rust as *mut c_char) };
}

/// Note: not-recursive. Free Error Message Manually!
#[no_mangle]
pub extern "C" fn free_c_result(originally_from_rust: *mut CResult) {
    let _will_drop: Box<CResult> = unsafe { Box::from_raw(originally_from_rust) };
}

/// Free a boxed i32!
#[no_mangle]
pub extern "C" fn free_i32(originally_from_rust: *mut i32) {
    let _will_drop: Box<i32> = unsafe { Box::from_raw(originally_from_rust) };
}

#[no_mangle]
pub extern "C" fn c_operate(op: *const c_void, x: i32, y: i32) -> *const CResult {
    result_to_c(operate(op, x, y))
}

fn operate(op: *const c_void, x: i32, y: i32) -> Result<i32, String> {
    let op = accept_str("operator", op)?;
    Ok(match op {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        "%" => x % y,
        "^" => x ^ y,
        _ => Err(format!("Bad Operator: '{}'", op))?
    })
}


#[cfg(test)]
mod tests {
    use super::operate;

    #[test]
    fn it_works() {
        assert_eq!(operate("+", 2, 2).unwrap(), 4);
    }
}
