# Dora C Binding Example

## Developing

### Prerequisites

- CMake >= 3.10
- C++ Compiler with C++17 Support
- LLVM 19
- Cargo

If you build on macos, you can set the environment to prevent link errors.

```shell
# Set cargo build target on macos
export MACOSX_DEPLOYMENT_TARGET='12.0'
```

### Build

Use cmake to build the whole project.

```shell
cargo build -r
cmake -S . -B build
cmake --build build --parallel
```

### Run

```shell
build/dora-c-example
```
