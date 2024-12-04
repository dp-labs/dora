<h1 align="center">Dora: The Ultimate Layer for on-chain Computing and Verification</h1>

<p align="center">
  <img src="https://github.com/dp-labs/dora/workflows/CI/badge.svg">
  <img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square">
  <img src="https://img.shields.io/github/license/dp-labs/dora.svg">
</p>

## Introduction

Dora is a modular machine layer designed to bring real-world computing use cases on-chain, featuring in-house ZK verification capabilities.

## Status

Dora is currently in the MVP stage, with support for EVM functionalities. It is actively under development and will soon be ready for production use.

## Why

Web3's programming and VM layers have attracted a growing developer community. Dora takes this a step further by introducing a next-gen blockchain programming and execution layer that addresses key needs:

- Extreme performance and lower costs: Optimized execution for real-world use cases.
- Multi-language support and toolchain flexibility: Accommodates a variety of programming languages and preferred toolchains.
- Borderless on-chain use cases: Removes barriers to enable seamless on-chain interoperability.
- Seamless integration with co-processors: Native support for collaboration with various co-processors, such as zkVM.

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

Check out contract examples [here](./examples/).

## Benchmarks

Dora performed well in basic test cases. The benchmark results are shown below. For more benchmark tests, see [here](./tests/bench/).

- `Fib(70)`

![7891731069869_ pic](https://github.com/user-attachments/assets/acc20aae-9f98-4323-b4e4-b64cb1536515)

- `Counter`

![7881731069869_ pic](https://github.com/user-attachments/assets/bac100ed-01ea-4106-aaf1-1b486624713f)


## Installation

Firstly, see the [developing guide](./docs/dev/guide.md) to install all build dependencies.

### Cargo

```shell
cargo install --git https://github.com/dp-labs/dora.git dora-cli
```

### Build from Source

In the top level of the repo and run

```shell
make build
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

For direct contact with the author, email: `ethan@dplabs.xyz` or message on Telegram: `@lethanxl`
