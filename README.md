<h1 align="center">Dora: The Ultimate Layer for on-chain Computing and Verification</h1>

<p align="center">
  <img src="https://github.com/dp-labs/dora/workflows/CI/badge.svg">
  <img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square">
  <img src="https://img.shields.io/github/license/dp-labs/dora.svg">
</p>

## Introduction

Dora is a modular machine layer designed to bring real-world computing use cases on-chain, featuring in-house ZK verification capabilities.

## Status

Dora is currently in the MVP stage, with support for EVM and WASM functionalities. It is actively under development and will soon be ready for production use.

## Why

Web3's programming and VM layers have attracted a growing developer community. Dora takes this a step further by introducing a next-gen blockchain programming and execution layer that addresses key needs:

- Extreme performance and lower costs: Optimized execution tailored for real-world on-chain use cases.
- Multi-language support and toolchain flexibility: Enabling a wide range of programming languages and toolchains, supporting EVM and WASM specifications within a single VM.
- Borderless on-chain use cases: Eliminating barriers to achieve seamless interoperability across a variety of on-chain use cases.
- Seamless integration with co-processors: Providing native support for collaboration with various co-processors, such as zkVM, for enhanced capabilities.

## Key Features in Early Stage

- **Super-fast EVM Spec Implementation**: Ultra-fast, EVM-compatible execution that integrates seamlessly with Ethereum clients, enabling a smooth adoption for developers and projects.
- **WASM Support and Interoperability**: Multi-language support through WASM bytecode, leveraging mature toolchains for development flexibility, while ensuring interoperability between diverse contracts.
- **Built-in Verification**: Enhances smart contract security with integrated zero-knowledge proof (ZKP) verification, boosting trust and reliability.

## Use Cases

VM is ideal for blockchain applications like:

- Micro-payments
- On-chain decentralized exchanges (DEXs)
- On-chain gaming
- AI inference
- Cross-chain protocols

## Examples

- Check out Dora EVM and WASM contract examples [here](./examples/).
- Check out Foundry, Hardhat and Rust toolchain examples [here](https://github.com/dp-labs/toolchain-examples) for Dora EVM and WASM.

## Benchmarks

Dora performed well in basic test cases. The benchmark tests and results are shown [here](./tests/bench/).

## Installation

Firstly, see the [developing guide](./docs/dev/guide.md) to install all build dependencies.

### Cargo

```shell
cargo install --git https://github.com/dp-labs/dora.git dora-cli
```

### Build from Source

In the `crates/dora-cli` folder and run

```shell
cargo build -r
```

Then you can find the binary `dora` in the `target/release` folder.

## How it Works

The Dora compiler is built on a unified intermediate representation(IR) layer tailored for blockchain applications, following a streamlined process to transform and optimize code into an ideal executable format. After thorough analysis and verification, the VM executes machine code generated from compiled native modules, which optimized for both main processors and co-processors. See [Tech design](./docs/design/tech.md) for more information.

## Roadmap

We are committed to continuous innovation, with key milestones including:

- Q4 2024: Launch the stable EVM version, complete with full test coverage and development tools.
- Q1 2025: Release the stable WASM version and interoperability with contracts written in Rust.
- Q2 2025: Introduce support for zkVM co-processors to enhance zero-knowledge proof capabilities.
- Q3 2025: SDKs and toolchains across multiple programming languages.

## Contributing

We welcome contributions to Dora! Whether you're a developer, a user with a bug report, or someone looking to improve our documentation, your involvement is valuable! See the [developing guide](./docs/dev/guide.md) for more information.

## Help Wanted

We are actively developing Dora and invite interested developers to join us. We need help with the following, see the issue list for more information.

## Contact

Join our public [Telegram group](https://t.me/+_OlJfYAc9QM2ODVl)
