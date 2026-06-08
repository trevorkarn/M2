// Cargo build script to generate C wrapper headers
use std::env;
use std::path::PathBuf;

fn main() {
    // Get the output directory for build artifacts
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Include generated headers in Rust code
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    
    // Generate C headers
    let mut gen = m2_build_gen::CHeaderGenerator::new();
    gen.generate_arithmetic_header(&out_dir);
    gen.generate_m2_types_header(&out_dir);
    gen.generate_atomic_header(&out_dir);
    gen.generate_interrupts_header(&out_dir);
}

// Inline generator since we can't easily import from lib
mod m2_build_gen {
    use std::fs;
    use std::path::Path;

    pub struct CHeaderGenerator;

    impl CHeaderGenerator {
        pub fn new() -> Self {
            CHeaderGenerator
        }

        pub fn generate_arithmetic_header(&self, out_dir: &Path) {
            let header = r#"/* Generated C wrappers for Rust arithmetic.rs */
#ifndef M2_ARITHMETIC_GENERATED_H
#define M2_ARITHMETIC_GENERATED_H
#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

bool m2_arithmetic_bool_not(bool x);
bool m2_arithmetic_bool_or(bool x, bool y);
bool m2_arithmetic_bool_and(bool x, bool y);
bool m2_arithmetic_bool_xor(bool x, bool y);
int32_t m2_arithmetic_int_add(int32_t x, int32_t y);
int32_t m2_arithmetic_int_sub(int32_t x, int32_t y);
int32_t m2_arithmetic_int_mul(int32_t x, int32_t y);
int32_t m2_arithmetic_int_div(int32_t x, int32_t y);
int32_t m2_arithmetic_int_rem(int32_t x, int32_t y);
uint32_t m2_arithmetic_uint_add(uint32_t x, uint32_t y);
uint32_t m2_arithmetic_uint_sub(uint32_t x, uint32_t y);
uint32_t m2_arithmetic_uint_mul(uint32_t x, uint32_t y);
uint32_t m2_arithmetic_uint_div(uint32_t x, uint32_t y);

#ifdef __cplusplus
}
#endif
#endif /* M2_ARITHMETIC_GENERATED_H */
"#;
            let _ = fs::write(out_dir.join("m2_arithmetic_generated.h"), header);
        }

        pub fn generate_m2_types_header(&self, out_dir: &Path) {
            let header = r#"/* Generated C wrappers for Rust m2_types.rs */
#ifndef M2_TYPES_GENERATED_H
#define M2_TYPES_GENERATED_H
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    size_t len;
    char *array;
} M2_string_struct;

typedef M2_string_struct *M2_string;

#ifdef __cplusplus
}
#endif
#endif /* M2_TYPES_GENERATED_H */
"#;
            let _ = fs::write(out_dir.join("m2_types_generated.h"), header);
        }

        pub fn generate_atomic_header(&self, out_dir: &Path) {
            let header = r#"/* Generated C wrappers for Rust atomic.rs */
#ifndef M2_ATOMIC_GENERATED_H
#define M2_ATOMIC_GENERATED_H
#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    int32_t field;
} M2_atomic_field;

int32_t m2_atomic_load(M2_atomic_field *field);
void m2_atomic_store(M2_atomic_field *field, int32_t value);
int32_t m2_atomic_fetch_add(M2_atomic_field *field, int32_t delta);
bool m2_atomic_test(M2_atomic_field *field);
void m2_atomic_compiler_barrier(void);

#ifdef __cplusplus
}
#endif
#endif /* M2_ATOMIC_GENERATED_H */
"#;
            let _ = fs::write(out_dir.join("m2_atomic_generated.h"), header);
        }

        pub fn generate_interrupts_header(&self, out_dir: &Path) {
            let header = r#"/* Generated C wrappers for Rust interrupts.rs */
#ifndef M2_INTERRUPTS_GENERATED_H
#define M2_INTERRUPTS_GENERATED_H
#include <stdbool.h>
#include <unistd.h>

#ifdef __cplusplus
extern "C" {
#endif

bool m2_interrupts_interrupt_shield(void);
void m2_interrupts_set_interrupt_shield(bool value);
void m2_interrupts_clear_all_flags(void);
void m2_interrupts_set_interrupt_flag(void);
void m2_interrupts_clear_interrupt_flag(void);

#ifdef __cplusplus
}
#endif
#endif /* M2_INTERRUPTS_GENERATED_H */
"#;
            let _ = fs::write(out_dir.join("m2_interrupts_generated.h"), header);
        }
    }
}
