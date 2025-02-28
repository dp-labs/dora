# Dora Toolchains

## Ether Test Tool

- `dora-ethertest` is a tool for running [ethertest](https://github.com/ethereum/tests) and testing the Dora VM.
- `dora-blocktest` is a tool tool for running historical blocks and transactions on the Ethereum mainnet.

### Ether State Tests

#### Prepare Test Suites

```shell
git clone https://github.com/ethereum/tests
```

#### Debug Mode Execution

To run the tests in debug mode, follow these commands:

```shell
cargo run -r --bin dora-ethertest run tests/GeneralStateTests
```

#### Release Mode Execution

For executing the tests in release mode, use the following steps:

```shell
cargo install --path .
dora-ethertest run tests/GeneralStateTests
```

### Block Test

#### Debug Mode Execution

To run the tests in debug mode, follow these commands:

```shell
cargo run -r --bin dora-blocktest run data/blocks
```

#### Release Mode Execution

For executing the tests in release mode, use the following steps:

```shell
cargo install --path .
dora-blocktest run data/blocks
```
