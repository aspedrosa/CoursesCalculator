use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use polars::prelude::*;

#[no_mangle]
pub extern "C" fn process_stage_results_csv(csv_path: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(csv_path) };
    let path = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return CString::new("Error: Invalid UTF-8 path").unwrap().into_raw(),
    };

    let result = match LazyCsvReader::new(path)
        .with_separator(b';')
        .with_has_header(true)
        .with_ignore_errors(true)
        .finish() {
        Ok(lazy) => {
            match lazy.collect() {
                Ok(df) => {
                    //let shape = df.shape();
                    //format!("Rust Polars Processed: {} rows, {} columns", shape.0, shape.1)
                    format!("{}", df)
                },
                Err(e) => format!("Error collecting: {}", e),
            }
        },
        Err(e) => format!("Error creating lazy reader: {}", e),
    };

    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        let _ = CString::from_raw(s);
    };
}

