/* Generated C wrappers for Rust arithmetic.rs */
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
