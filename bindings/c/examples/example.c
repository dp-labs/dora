#include <evmc/helpers.h>
#include <evmc/loader.h>

#include "example_host.h"
#include "dora.h"

#include <inttypes.h>
#include <stdio.h>

int main(int argc, char* argv[])
{
    (void)argc;
    (void)argv;
    struct evmc_vm* vm = evmc_create_doravm();
    if (!vm)
        return EVMC_LOADER_VM_CREATION_FAILURE;
    if (!evmc_is_abi_compatible(vm))
        return EVMC_LOADER_ABI_VERSION_MISMATCH;

    // EVM bytecode goes here. This is the fibonacci example
    const uint8_t code[] = {95, 53, 95, 96, 1, 91, 130, 21, 96, 26, 87, 129, 129, 1, 145, 80, 144, 145, 96, 1, 144, 3, 145, 96, 5, 86, 91, 145, 80, 80, 95, 82, 96, 32, 95, 243};
    const uint8_t input[] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 80};
    const size_t code_size = sizeof(code) - 1;
    const evmc_uint256be value = {{}};
    const evmc_address addr = {{0, 1, 2}};
    const int64_t gas = 200000;
    struct evmc_tx_context tx_context = {
        .block_number = 42,
        .block_timestamp = 66,
        .block_gas_limit = gas * 2,
    };
    const struct evmc_host_interface* host = example_host_get_interface();
    struct evmc_host_context* ctx = example_host_create_context(tx_context);
    struct evmc_message msg = {
        .kind = EVMC_CALL,
        .sender = addr,
        .recipient = addr,
        .value = value,
        .input_data = input,
        .input_size = sizeof(input),
        .gas = gas,
        .depth = 0,
    };
    printf("VM Name: %s\n", vm->name);
    struct evmc_result result = evmc_execute(vm, host, ctx, EVMC_CANCUN, &msg, code, code_size);
    printf("Execution result:\n");
    int exit_code = 0;
    if (result.status_code != EVMC_SUCCESS)
    {
        printf("  EVM execution failure: %d\n", result.status_code);
        exit_code = result.status_code;
    }
    else
    {
        printf("  Gas used: %" PRId64 "\n", gas - result.gas_left);
        printf("  Gas left: %" PRId64 "\n", result.gas_left);
        printf("  Output size: %zd\n", result.output_size);
        printf("  Output: ");
        for (size_t i = 0; i < result.output_size; i++)
            printf("%02x", result.output_data[i]);
        printf("\n");
        const evmc_bytes32 storage_key = {{0}};
        evmc_bytes32 storage_value = host->get_storage(ctx, &msg.recipient, &storage_key);
        printf("  Storage at 0x00..00: ");
        for (size_t i = 0; i < sizeof(storage_value.bytes) / sizeof(storage_value.bytes[0]); i++)
            printf("%02x", storage_value.bytes[i]);
        printf("\n");
    }
    evmc_release_result(&result);
    example_host_destroy_context(ctx);
    evmc_destroy(vm);
    return exit_code;
}
