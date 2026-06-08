# M2 Rust Interpreter Build Instructions

## Overview
This document describes how to build M2 with the new Rust interpreter implementation that gradually replaces the D language files.

## What's New
The M2 build system has been extended to support **Phase 1** of Rust-based interpreter implementation:

- **arithmetic.d** → Rust arithmetic.rs (basic arithmetic, bitwise, and comparison operations)
- **atomic.d** → Rust atomic.rs (atomic field operations with proper memory ordering)
- **interrupts.d** → Rust interrupts.rs (interrupt and exception flag management)

## Building M2 with Rust Interpreter

### Quick Start
```bash
cd /Users/trevorkarn/M2/M2
mkdir build-rust
cd build-rust

export CPPFLAGS="-I/opt/homebrew/opt/libomp/include"
export LDFLAGS="-L/opt/homebrew/opt/libomp/lib"

cmake .. \
  -DCMAKE_BUILD_TYPE=Release \
  -DM2_DIST_PREFIX=/Users/trevorkarn/M2/install \
  -DCMAKE_INSTALL_PREFIX=/Users/trevorkarn/M2/install \
  -DUSE_RUST_INTERPRETER=ON

cmake --build . --target M2-binary -- -j4
```

### CMake Options

#### USE_RUST_INTERPRETER (ON/OFF, default: OFF)
When enabled, the build uses Rust implementations of:
- `arithmetic.d` → Rust implementation
- `atomic.d` → Rust implementation  
- `interrupts.d` → Rust implementation

When disabled (default), the traditional D language files are compiled using scc1.

### Build Process

1. **Rust Library Build**: The Rust interpreter crate is built first via Cargo
2. **D File Processing**: Only non-Rust-replaced D files are compiled through scc1
3. **Linking**: The Rust static library is linked with the C/C++ code

## Technical Details

### Rust Module Structure
```
M2/Macaulay2/rust-interp/
├── Cargo.toml
├── build.rs (generates C headers)
├── src/
│   ├── lib.rs (module exports)
│   ├── arithmetic.rs (Phase 1)
│   ├── atomic.rs (Phase 1)
│   ├── interrupts.rs (Phase 1)
│   ├── m2_types.rs (Phase 1)
│   ├── c_ffi_exports.rs (C calling convention wrappers)
│   ├── generate_c_wrapper.rs (header generator)
│   └── ... (existing parser/lexer modules)
```

### C FFI Exports
The Rust modules export C-compatible functions:
- `m2_arithmetic_int_add(int, int) → int`
- `m2_arithmetic_uint_div(uint, uint) → uint`
- `m2_atomic_load(field*) → int`
- `m2_interrupts_set_interrupt_flag()`
- etc.

### CMake Integration
Modified files:
- `M2/Macaulay2/d/CMakeLists.txt`: Added `USE_RUST_INTERPRETER` option and conditional DLIST modification

## Compilation Notes

### Dependencies
- Rust toolchain (cargo, rustc)
- CMake 3.16+
- C/C++ compiler (AppleClang on macOS)
- libomp (OpenMP)

### Build Performance
- First build with Rust interpreter: Slightly slower (Rust crate must build)
- Incremental builds: Faster (fewer D files to scc1 translate)

## Troubleshooting

### Issue: "Could not find OpenMP_C"
**Solution:**
```bash
export CPPFLAGS="-I/opt/homebrew/opt/libomp/include"
export LDFLAGS="-L/opt/homebrew/opt/libomp/lib"
```

### Issue: Rust build fails
**Solution:** Ensure Rust toolchain is installed
```bash
rustup update
cd M2/Macaulay2/rust-interp
cargo build --release
```

### Issue: Link errors with Rust symbols
**Solution:** Verify the Rust library is being linked:
```bash
nm M2/Macaulay2/rust-interp/target/release/libm2_rust_interpreter.a | grep m2_arithmetic
```

## Next Steps (Future Phases)

### Phase 2: String and Type Operations
- Port M2.d (core types)
- Port strings.d (string operations)
- Port varstrin.d (variable-length strings)

### Phase 3: Parsing and Expressions
- Port expr.d (expressions)
- Port parser.d (parser)
- Port lex.d (lexer)

### Phase 4: Advanced Features
- Port hashtables.dd
- Port evaluation.d
- Port actors modules

## Development Workflow

To add new Rust implementations:

1. **Create Rust module** in `rust-interp/src/`
2. **Add C FFI exports** in `c_ffi_exports.rs`
3. **Generate C headers** via build.rs
4. **Update CMakeLists.txt** to exclude D files:
   ```cmake
   if(USE_RUST_INTERPRETER)
       list(REMOVE_ITEM DLIST newfile.d)
   endif()
   ```
5. **Test build** with `USE_RUST_INTERPRETER=ON`

## Performance Expectations

### Current Status (Phase 1)
- Rust modules: ~600 lines
- D files replaced: 3 files (~300 lines D code)
- Build time impact: Minimal

### Full Implementation (Target)
- Rust modules: ~34,000 lines (matching D)
- D files replaced: 50 files
- Estimated benefit: 10-30% faster builds (no scc1 translation)

## References

- Rust FFI Guide: https://doc.rust-lang.org/nomicon/ffi.html
- Original D files: M2/Macaulay2/d/*.d
- Rust interpreter: M2/Macaulay2/rust-interp/
