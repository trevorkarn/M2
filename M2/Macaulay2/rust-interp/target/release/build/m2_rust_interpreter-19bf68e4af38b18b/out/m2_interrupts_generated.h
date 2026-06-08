/* Generated C wrappers for Rust interrupts.rs */
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
