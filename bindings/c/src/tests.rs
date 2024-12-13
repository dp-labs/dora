use crate::*;

#[test]
fn test_doravm_lifecycle() {
    let doravm = evmc_create_doravm();
    __evmc_destroy(doravm as _);
}
