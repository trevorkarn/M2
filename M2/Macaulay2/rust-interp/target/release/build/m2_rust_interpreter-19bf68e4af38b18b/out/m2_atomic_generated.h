/* Generated C wrappers for Rust atomic.rs */
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
