//! Connection to external EVM tracer.

use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

extern "C" {
    fn CreateTrace(str: *const c_char) -> *const c_char;
    fn FreeString(str: *const c_char);
}

/// Creates the l2 trace
pub fn l2trace(config: &str) -> Result<String, String> {
    // Create a string we can pass into Go
    let c_config = CString::new(config).expect("invalid config");

    // Generate the trace externally
    let result = unsafe { CreateTrace(c_config.as_ptr()) };

    // Convert the returned string to something we can use in Rust again.
    // Also make sure the returned data is copied to rust managed memory.
    let c_result = unsafe { CStr::from_ptr(result) };
    let result = c_result
        .to_str()
        .expect("Error translating EVM trace from library")
        .to_string();

    // We can now free the returned string (memory managed by Go)
    unsafe { FreeString(c_result.as_ptr()) };

    // Return the trace
    match result.is_empty() || result.starts_with("Failed") {
        true => Err(result),
        false => Ok(result),
    }
}
