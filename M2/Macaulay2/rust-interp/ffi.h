#ifndef M2_RUST_INTERP_FFI_H
#define M2_RUST_INTERP_FFI_H

#ifdef __cplusplus
extern "C" {
#endif

char *m2_rust_parse_and_convert(const char *input);
void m2_rust_free_string(char *ptr);

#ifdef __cplusplus
}
#endif

#endif // M2_RUST_INTERP_FFI_H
