# Quick Start

This documentation is _NOT_ intended to be comprehensive; it is meant to be a quick guide for the most useful things.

## Cloning and Building `dora`

### System Requirements

The following hardware is recommended.

- 10GB+ of free disk space.
- 4GB+ RAM
- 2+ cores

### Dependencies

#### macOS and OS X

- `git`
- `Rust 1.82+`
- `LLVM 19`

You'll need LLVM installed and `llvm-config` in your `PATH`. Just download `llvm@19` using `brew`.

```shell
brew install llvm@19
```

Setup a environment variable called `MLIR_SYS_190_PREFIX`, `LLVM_SYS_190_PREFIX` and `TABLEGEN_190_PREFIX` pointing to the llvm directory:

```bash
export LLVM_SYS_190_PREFIX="$(brew --prefix llvm@19)"
export MLIR_SYS_190_PREFIX="$(brew --prefix llvm@19)"
export TABLEGEN_190_PREFIX="$(brew --prefix llvm@19)"
```

#### Linux

- `git`
- `Rust 1.82+`
- `LLVM 19`

If you are on Debian/Ubuntu, check out the repository [https://apt.llvm.org/](https://apt.llvm.org/) Then you can install with:

```bash
sudo apt-get install llvm-19 llvm-19-dev llvm-19-runtime clang-19 clang-tools-19 lld-19 libpolly-19-dev libmlir-19-dev mlir-19-tools
```

If you want to build from source, here are some indications:

<details><summary>Install LLVM from source instructions</summary>

```bash
wget https://github.com/llvm/llvm-project/releases/download/llvmorg-19.1.4/llvm-project-19.1.4.src.tar.xz
tar xf llvm-project-19.1.4.src.tar.xz

cd llvm-project-19.1.4.src
mkdir build
cd build

# The following cmake command configures the build to be installed to /opt/llvm-18
cmake -G Ninja ../llvm \
   -DLLVM_ENABLE_PROJECTS="mlir;clang;clang-tools-extra;lld;polly" \
   -DLLVM_BUILD_EXAMPLES=OFF \
   -DLLVM_TARGETS_TO_BUILD="Native" \
   -DCMAKE_INSTALL_PREFIX=/opt/llvm-18 \
   -DCMAKE_BUILD_TYPE=RelWithDebInfo \
   -DLLVM_PARALLEL_LINK_JOBS=4 \
   -DLLVM_ENABLE_BINDINGS=OFF \
   -DCMAKE_C_COMPILER=clang -DCMAKE_CXX_COMPILER=clang++ -DLLVM_ENABLE_LLD=ON \
   -DLLVM_ENABLE_ASSERTIONS=OFF

ninja install
```

</details>

Setup a environment variable called `MLIR_SYS_190_PREFIX`, `LLVM_SYS_190_PREFIX` and `TABLEGEN_190_PREFIX` pointing to the llvm directory:

```bash
export LLVM_SYS_190_PREFIX=/usr/lib/llvm-19
export MLIR_SYS_190_PREFIX=/usr/lib/llvm-19
export TABLEGEN_190_PREFIX=/usr/lib/llvm-19
export PATH=$PATH:/usr/lib/llvm-19/bin
```

Install other dependencies needed by this project.

```shell
apt-get install gcc g++ make git zlib1g-dev zstd libzstd-dev -y
```

### Cloning

You can just do a normal git clone:

```shell
git clone https://github.com/dp-labs/dora.git
cd dora
```

### Building

In the top level of the repo and run

```shell
cargo build -r --all
```

### Testing

#### Unit Testing

In the top level of the repo and run

```shell
make test
```

#### Benchmark

In the `tests/bench` folder and run

```shell
cargo bench
```

#### Snapshot

Install dependencies

```shell
cargo install cargo-insta
```

Run unit tests

```shell
cargo insta test
```

Update snapshots

```shell
cargo insta accept
```

### Formatting

In the top level of the repo and run

```shell
make fmt
```

### Linting

In the top level of the repo and run

```shell
make clippy
```
