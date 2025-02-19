# Dora WASM Fibonacci Example

This wasm code run the `fib(10)` and use the `console.log` host to print the result to stdout with string data structure and heap memory allocation and GC.

## How

Run the example

```shell
dora run fib.wasm
```

We can use the [wasm-tools](https://github.com/bytecodealliance/wasm-tools) to print the text format of the WASM module to stdout.

```shell
wasm-tools print fib.wasm
```
