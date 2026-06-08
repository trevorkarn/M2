// Rust implementation of arithmetic.d
// Provides basic arithmetic types and operations

pub use std::ffi::c_void;

// Re-export basic integer types to match D exports
pub type Nothing = ();
pub type SizeT = usize;
pub type Uchar = u8;
pub type Short = i16;
pub type Ushort = u16;
pub type Uint = u32;
pub type Long = i64;
pub type Ulong = u64;
pub type Longlong = i128;
pub type Ulonglong = u128;
pub type Int8T = i8;
pub type Uint8T = u8;
pub type Int16T = i16;
pub type Uint16T = u16;
pub type Int32T = i32;
pub type Uint32T = u32;
pub type Int64T = i64;
pub type Uint64T = u64;
pub type HashT = u64;
pub type M2Float = f32;

// Arithmetic operations on integers
pub fn bool_not(x: bool) -> bool {
    !x
}

pub fn bool_or(x: bool, y: bool) -> bool {
    x | y
}

pub fn bool_and(x: bool, y: bool) -> bool {
    x & y
}

pub fn bool_xor(x: bool, y: bool) -> bool {
    x ^ y
}

pub fn int_or(x: i32, y: i32) -> i32 {
    x | y
}

pub fn int_and(x: i32, y: i32) -> i32 {
    x & y
}

pub fn int_xor(x: i32, y: i32) -> i32 {
    x ^ y
}

pub fn int_not(x: i32) -> i32 {
    !x
}

pub fn int_shr(x: i32, y: i32) -> i32 {
    x >> y
}

pub fn int_shl(x: i32, y: i32) -> i32 {
    x << y
}

pub fn uint_or(x: u32, y: u32) -> u32 {
    x | y
}

pub fn uint_and(x: u32, y: u32) -> u32 {
    x & y
}

pub fn uint_xor(x: u32, y: u32) -> u32 {
    x ^ y
}

pub fn uint_shr(x: u32, y: i32) -> u32 {
    x >> y
}

pub fn uint_shl(x: u32, y: i32) -> u32 {
    x << y
}

pub fn ushort_or(x: u16, y: u16) -> u16 {
    x | y
}

pub fn ushort_and(x: u16, y: u16) -> u16 {
    x & y
}

pub fn ushort_xor(x: u16, y: u16) -> u16 {
    x ^ y
}

pub fn ushort_shr(x: u16, y: i32) -> u16 {
    x >> y
}

pub fn ushort_shl(x: u16, y: i32) -> u16 {
    (x << y) as u16
}

// Comparison operations
pub fn int_lt(x: i32, y: i32) -> bool {
    x < y
}

pub fn int_le(x: i32, y: i32) -> bool {
    x <= y
}

pub fn int_gt(x: i32, y: i32) -> bool {
    x > y
}

pub fn int_ge(x: i32, y: i32) -> bool {
    x >= y
}

pub fn uint_lt(x: u32, y: u32) -> bool {
    x < y
}

pub fn uint_le(x: u32, y: u32) -> bool {
    x <= y
}

pub fn uint_gt(x: u32, y: u32) -> bool {
    x > y
}

pub fn uint_ge(x: u32, y: u32) -> bool {
    x >= y
}

// Arithmetic operations
pub fn int_add(x: i32, y: i32) -> i32 {
    x.wrapping_add(y)
}

pub fn int_sub(x: i32, y: i32) -> i32 {
    x.wrapping_sub(y)
}

pub fn int_mul(x: i32, y: i32) -> i32 {
    x.wrapping_mul(y)
}

pub fn int_div(x: i32, y: i32) -> i32 {
    if y == 0 {
        0 // Match C behavior (undefined, but return 0)
    } else {
        x / y
    }
}

pub fn int_rem(x: i32, y: i32) -> i32 {
    if y == 0 {
        0
    } else {
        x % y
    }
}

pub fn uint_add(x: u32, y: u32) -> u32 {
    x.wrapping_add(y)
}

pub fn uint_sub(x: u32, y: u32) -> u32 {
    x.wrapping_sub(y)
}

pub fn uint_mul(x: u32, y: u32) -> u32 {
    x.wrapping_mul(y)
}

pub fn uint_div(x: u32, y: u32) -> u32 {
    if y == 0 {
        0
    } else {
        x / y
    }
}

pub fn uint_rem(x: u32, y: u32) -> u32 {
    if y == 0 {
        0
    } else {
        x % y
    }
}

pub fn double_lt(x: f64, y: f64) -> bool {
    x < y
}

pub fn double_le(x: f64, y: f64) -> bool {
    x <= y
}

pub fn double_gt(x: f64, y: f64) -> bool {
    x > y
}

pub fn double_ge(x: f64, y: f64) -> bool {
    x >= y
}

pub fn double_eq(x: f64, y: f64) -> bool {
    (x - y).abs() < f64::EPSILON
}

pub fn double_ne(x: f64, y: f64) -> bool {
    (x - y).abs() >= f64::EPSILON
}
