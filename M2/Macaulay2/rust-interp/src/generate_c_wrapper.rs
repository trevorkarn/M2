// Generate C wrapper code for Rust implementations
// This produces C code that can be compiled and linked with the rest of M2

use std::fs;
use std::path::Path;

/// Generate C code wrappers for Rust arithmetic module
pub fn generate_arithmetic_c() -> String {
    r#"/* Generated C wrappers for Rust arithmetic.rs */
#ifndef __cplusplus
#include <stdint.h>
#include <stdbool.h>
#else
#include <cstdint>
#include <cstdbool>
#endif

/* Type definitions matching D exports */
typedef void M2_nothing;
typedef size_t M2_size_t;
typedef unsigned char M2_uchar;
typedef short M2_short;
typedef unsigned short M2_ushort;
typedef unsigned int M2_uint;
typedef long M2_long;
typedef unsigned long M2_ulong;
typedef long long M2_longlong;
typedef unsigned long long M2_ulonglong;
typedef int8_t M2_int8_t;
typedef uint8_t M2_uint8_t;
typedef int16_t M2_int16_t;
typedef uint16_t M2_uint16_t;
typedef int32_t M2_int32_t;
typedef uint32_t M2_uint32_t;
typedef int64_t M2_int64_t;
typedef uint64_t M2_uint64_t;
typedef uint64_t M2_hash_t;
typedef float M2_float;

/* Arithmetic operation forward declarations - these are implemented in Rust */
extern bool m2_arithmetic_bool_not(bool x);
extern bool m2_arithmetic_bool_or(bool x, bool y);
extern bool m2_arithmetic_bool_and(bool x, bool y);
extern bool m2_arithmetic_bool_xor(bool x, bool y);

extern int32_t m2_arithmetic_int_or(int32_t x, int32_t y);
extern int32_t m2_arithmetic_int_and(int32_t x, int32_t y);
extern int32_t m2_arithmetic_int_xor(int32_t x, int32_t y);
extern int32_t m2_arithmetic_int_not(int32_t x);
extern int32_t m2_arithmetic_int_shr(int32_t x, int32_t y);
extern int32_t m2_arithmetic_int_shl(int32_t x, int32_t y);

extern uint32_t m2_arithmetic_uint_or(uint32_t x, uint32_t y);
extern uint32_t m2_arithmetic_uint_and(uint32_t x, uint32_t y);
extern uint32_t m2_arithmetic_uint_xor(uint32_t x, uint32_t y);
extern uint32_t m2_arithmetic_uint_shr(uint32_t x, int32_t y);
extern uint32_t m2_arithmetic_uint_shl(uint32_t x, int32_t y);

extern bool m2_arithmetic_int_lt(int32_t x, int32_t y);
extern bool m2_arithmetic_int_le(int32_t x, int32_t y);
extern bool m2_arithmetic_int_gt(int32_t x, int32_t y);
extern bool m2_arithmetic_int_ge(int32_t x, int32_t y);

extern uint32_t m2_arithmetic_uint_add(uint32_t x, uint32_t y);
extern uint32_t m2_arithmetic_uint_sub(uint32_t x, uint32_t y);
extern uint32_t m2_arithmetic_uint_mul(uint32_t x, uint32_t y);
extern uint32_t m2_arithmetic_uint_div(uint32_t x, uint32_t y);
extern uint32_t m2_arithmetic_uint_rem(uint32_t x, uint32_t y);

extern int32_t m2_arithmetic_int_add(int32_t x, int32_t y);
extern int32_t m2_arithmetic_int_sub(int32_t x, int32_t y);
extern int32_t m2_arithmetic_int_mul(int32_t x, int32_t y);
extern int32_t m2_arithmetic_int_div(int32_t x, int32_t y);
extern int32_t m2_arithmetic_int_rem(int32_t x, int32_t y);

extern bool m2_arithmetic_double_lt(double x, double y);
extern bool m2_arithmetic_double_le(double x, double y);
extern bool m2_arithmetic_double_gt(double x, double y);
extern bool m2_arithmetic_double_ge(double x, double y);
extern bool m2_arithmetic_double_eq(double x, double y);
extern bool m2_arithmetic_double_ne(double x, double y);
"#.to_string()
}

/// Generate C code wrappers for Rust m2_types module
pub fn generate_m2_types_c() -> String {
    r#"/* Generated C wrappers for Rust m2_types.rs */
#include <stddef.h>

/* M2String structure - matches Rust M2String repr(C) */
typedef struct {
    size_t len;
    char *array;
} M2_string_struct;

typedef M2_string_struct *M2_string;

/* Array types */
typedef struct {
    size_t len;
    int *array;
} M2_arrayint_struct;

typedef M2_arrayint_struct *M2_arrayint;

typedef struct {
    size_t len;
    M2_string *array;
} M2_ArrayString_struct;

typedef M2_ArrayString_struct *M2_ArrayString;

/* String operations forward declarations */
extern int32_t m2_types_string_compare(M2_string s, M2_string t);
extern uint64_t m2_types_string_hash(M2_string s);
extern size_t m2_types_string_length(M2_string s);
"#.to_string()
}

/// Generate C code wrappers for Rust atomic module
pub fn generate_atomic_c() -> String {
    r#"/* Generated C wrappers for Rust atomic.rs */
#include <stdint.h>

/* AtomicField structure */
typedef struct {
    int32_t field;
} M2_atomic_field;

extern int32_t m2_atomic_load(M2_atomic_field *field);
extern void m2_atomic_store(M2_atomic_field *field, int32_t value);
extern int32_t m2_atomic_fetch_add(M2_atomic_field *field, int32_t delta);
extern bool m2_atomic_test(M2_atomic_field *field);
extern void m2_atomic_compiler_barrier(void);
"#.to_string()
}

/// Generate C code wrappers for Rust interrupts module
pub fn generate_interrupts_c() -> String {
    r#"/* Generated C wrappers for Rust interrupts.rs */
#include <stdint.h>
#include <stdbool.h>
#include <unistd.h>

extern bool m2_interrupts_interrupt_shield(void);
extern void m2_interrupts_set_interrupt_shield(bool value);
extern bool m2_interrupts_interrupt_pending(void);
extern void m2_interrupts_set_interrupt_pending(bool value);
extern bool m2_interrupts_alarmed_flag(void);
extern void m2_interrupts_set_alarmed_flag(bool value);
extern bool m2_interrupts_stepping_flag(void);
extern void m2_interrupts_set_stepping_flag(bool value);
extern int32_t m2_interrupts_step_count(void);
extern void m2_interrupts_set_step_count(int32_t value);
extern int32_t m2_interrupts_micro_step_count(void);
extern void m2_interrupts_set_micro_step_count(int32_t value);
extern void m2_interrupts_determine_exception_flag(void);
extern int32_t m2_interrupts_alarm(uint32_t seconds);
extern void m2_interrupts_clear_alarm(void);
extern void m2_interrupts_clear_all_flags(void);
extern void m2_interrupts_set_interrupt_flag(void);
extern void m2_interrupts_set_alarmed_flag_internal(void);
extern void m2_interrupts_set_stepping_flag_internal(void);
extern void m2_interrupts_clear_interrupt_flag(void);
extern void m2_interrupts_clear_alarmed_flag_internal(void);
extern void m2_interrupts_clear_stepping_flag(void);
"#.to_string()
}

/// Write all generated C header files to a directory
pub fn write_generated_headers(output_dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(output_dir)?;
    
    fs::write(
        output_dir.join("m2_arithmetic_generated.h"),
        generate_arithmetic_c(),
    )?;
    
    fs::write(
        output_dir.join("m2_types_generated.h"),
        generate_m2_types_c(),
    )?;
    
    fs::write(
        output_dir.join("m2_atomic_generated.h"),
        generate_atomic_c(),
    )?;
    
    fs::write(
        output_dir.join("m2_interrupts_generated.h"),
        generate_interrupts_c(),
    )?;
    
    Ok(())
}
