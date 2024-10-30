fn main() {
    println!("cargo:rerun-if-changed=src/include/evm.td");
    println!("cargo:rerun-if-changed=src/include/wasm.td");
    println!("cargo:rerun-if-changed=src/include/dora.td");
}
