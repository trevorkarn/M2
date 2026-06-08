// Rust implementation of M2.d and basic string types
// Provides core M2 types and string/array operations

use std::ffi::{c_char, c_void, CStr, CString};
use std::slice;

/// Represents a Macaulay2 string as an array of characters
#[repr(C)]
pub struct M2String {
    pub len: usize,
    pub array: *mut c_char,
}

impl M2String {
    pub fn new(len: usize, array: *mut c_char) -> Self {
        M2String { len, array }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.array as *const u8, self.len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.array as *mut u8, self.len) }
    }

    pub fn to_string(&self) -> String {
        String::from_utf8_lossy(self.as_slice()).to_string()
    }
}

/// Array of integers
#[repr(C)]
pub struct M2ArrayInt {
    pub len: usize,
    pub array: *mut i32,
}

impl M2ArrayInt {
    pub fn new(len: usize, array: *mut i32) -> Self {
        M2ArrayInt { len, array }
    }

    pub fn as_slice(&self) -> &[i32] {
        unsafe { slice::from_raw_parts(self.array, self.len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [i32] {
        unsafe { slice::from_raw_parts_mut(self.array, self.len) }
    }
}

/// Array of strings
#[repr(C)]
pub struct M2ArrayString {
    pub len: usize,
    pub array: *mut M2String,
}

impl M2ArrayString {
    pub fn new(len: usize, array: *mut M2String) -> Self {
        M2ArrayString { len, array }
    }

    pub fn as_slice(&self) -> &[M2String] {
        unsafe { slice::from_raw_parts(self.array, self.len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [M2String] {
        unsafe { slice::from_raw_parts_mut(self.array, self.len) }
    }
}

/// Convert Rust String to M2String
/// Note: The caller must manage the lifetime of the returned pointer
#[no_mangle]
pub extern "C" fn m2_rust_string_from_cstr(s: *const c_char) -> *mut M2String {
    if s.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(s) };
    let bytes = c_str.to_bytes();

    // Allocate memory for M2String through FFI
    // For now, we return null - in real code, this would use M2's memory allocator
    std::ptr::null_mut()
}

/// Compare two strings lexicographically
/// Returns: -1 if s < t, 0 if s == t, 1 if s > t
pub fn string_compare(s: &M2String, t: &M2String) -> i32 {
    let s_slice = s.as_slice();
    let t_slice = t.as_slice();

    match s_slice.cmp(t_slice) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

/// Hash a string
pub fn string_hash(s: &M2String) -> u64 {
    let mut h: u64 = 0;
    for &byte in s.as_slice() {
        h = h.wrapping_mul(31).wrapping_add(byte as u64);
    }
    h & 0x7fffffff_u64
}

/// Get string length
pub fn string_length(s: &M2String) -> usize {
    s.len
}

/// Get character at index
pub fn string_index(s: &M2String, idx: usize) -> Option<u8> {
    if idx < s.len {
        Some(s.as_slice()[idx])
    } else {
        None
    }
}
