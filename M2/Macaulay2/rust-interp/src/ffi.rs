use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::{convert, lexer::tokenize, parser::parse};

#[no_mangle]
pub extern "C" fn m2_rust_parse_and_convert(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return std::ptr::null_mut();
    }

    let input = unsafe { CStr::from_ptr(input) };
    let source = match input.to_str() {
        Ok(text) => text,
        Err(_) => return std::ptr::null_mut(),
    };

    let mut output = String::new();

    match tokenize(source, "<rust-interp>") {
        Ok(tokens) => {
            output.push_str("[rust] tokens:\n");
            output.push_str(&format!("{:?}\n", tokens));
            match parse(&tokens) {
                Ok(tree) => {
                    output.push_str("[rust] parse tree:\n");
                    output.push_str(&format!("{:?}\n", tree));
                    let code = convert(tree);
                    output.push_str("[rust] converted code:\n");
                    output.push_str(&format!("{:?}\n", code));
                }
                Err(err) => {
                    output.push_str("[rust] parse error: ");
                    output.push_str(&err.to_string());
                    output.push('\n');
                }
            }
        }
        Err(err) => {
            output.push_str("[rust] lex error: ");
            output.push_str(&err.to_string());
            output.push('\n');
        }
    }

    match CString::new(output) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn m2_rust_free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}
