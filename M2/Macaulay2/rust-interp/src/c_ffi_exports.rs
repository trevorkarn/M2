// C FFI wrappers to export Rust functions to C code
// These are the functions that C code can call

use crate::arithmetic;
use crate::atomic;
use crate::interrupts;
use crate::m2_types::{M2String, string_hash, string_compare, string_length};

// ===== Arithmetic FFI =====

#[no_mangle]
pub extern "C" fn m2_arithmetic_bool_not(x: bool) -> bool {
    arithmetic::bool_not(x)
}

#[no_mangle]
pub extern "C" fn m2_arithmetic_bool_or(x: bool, y: bool) -> bool {
    arithmetic::bool_or(x, y)
}

#[no_mangle]
pub extern "C" fn m2_arithmetic_bool_and(x: bool, y: bool) -> bool {
    arithmetic::bool_and(x, y)
}

#[no_mangle]
pub extern "C" fn m2_arithmetic_int_add(x: i32, y: i32) -> i32 {
    arithmetic::int_add(x, y)
}

#[no_mangle]
pub extern "C" fn m2_arithmetic_int_sub(x: i32, y: i32) -> i32 {
    arithmetic::int_sub(x, y)
}

#[no_mangle]
pub extern "C" fn m2_arithmetic_int_mul(x: i32, y: i32) -> i32 {
    arithmetic::int_mul(x, y)
}

#[no_mangle]
pub extern "C" fn m2_arithmetic_int_div(x: i32, y: i32) -> i32 {
    arithmetic::int_div(x, y)
}

#[no_mangle]
pub extern "C" fn m2_arithmetic_int_rem(x: i32, y: i32) -> i32 {
    arithmetic::int_rem(x, y)
}

#[no_mangle]
pub extern "C" fn m2_arithmetic_uint_add(x: u32, y: u32) -> u32 {
    arithmetic::uint_add(x, y)
}

#[no_mangle]
pub extern "C" fn m2_arithmetic_uint_sub(x: u32, y: u32) -> u32 {
    arithmetic::uint_sub(x, y)
}

#[no_mangle]
pub extern "C" fn m2_arithmetic_uint_mul(x: u32, y: u32) -> u32 {
    arithmetic::uint_mul(x, y)
}

#[no_mangle]
pub extern "C" fn m2_arithmetic_uint_div(x: u32, y: u32) -> u32 {
    arithmetic::uint_div(x, y)
}

#[no_mangle]
pub extern "C" fn m2_arithmetic_int_lt(x: i32, y: i32) -> bool {
    arithmetic::int_lt(x, y)
}

#[no_mangle]
pub extern "C" fn m2_arithmetic_int_le(x: i32, y: i32) -> bool {
    arithmetic::int_le(x, y)
}

#[no_mangle]
pub extern "C" fn m2_arithmetic_int_gt(x: i32, y: i32) -> bool {
    arithmetic::int_gt(x, y)
}

#[no_mangle]
pub extern "C" fn m2_arithmetic_int_ge(x: i32, y: i32) -> bool {
    arithmetic::int_ge(x, y)
}

#[no_mangle]
pub extern "C" fn m2_arithmetic_uint_lt(x: u32, y: u32) -> bool {
    arithmetic::uint_lt(x, y)
}

#[no_mangle]
pub extern "C" fn m2_arithmetic_uint_le(x: u32, y: u32) -> bool {
    arithmetic::uint_le(x, y)
}

#[no_mangle]
pub extern "C" fn m2_arithmetic_uint_gt(x: u32, y: u32) -> bool {
    arithmetic::uint_gt(x, y)
}

#[no_mangle]
pub extern "C" fn m2_arithmetic_uint_ge(x: u32, y: u32) -> bool {
    arithmetic::uint_ge(x, y)
}

// ===== Atomic FFI =====

#[no_mangle]
pub extern "C" fn m2_atomic_compiler_barrier() {
    atomic::compiler_barrier();
}

// ===== Interrupts FFI =====

#[no_mangle]
pub extern "C" fn m2_interrupts_clear_all_flags() {
    interrupts::clear_all_flags();
}

#[no_mangle]
pub extern "C" fn m2_interrupts_set_interrupt_flag() {
    interrupts::set_interrupt_flag();
}

#[no_mangle]
pub extern "C" fn m2_interrupts_clear_interrupt_flag() {
    interrupts::clear_interrupt_flag();
}

#[no_mangle]
pub extern "C" fn m2_interrupts_alarm(seconds: u32) -> i32 {
    interrupts::alarm(seconds)
}

#[no_mangle]
pub extern "C" fn m2_interrupts_clear_alarm() {
    interrupts::clear_alarm();
}
