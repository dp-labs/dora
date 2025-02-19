# EVM Examples

Firstly, you can follow the [readme](../../README.md) documents to install or build the `dora` binary, then you can enter the corresponding folder to run the example with the `Makefile`.

- [Fibonacci](./fibonacci/)
- [Factorial](./factorial/)
- [Counter](./counter/)
- [ERC20 Transfer](./erc20_transfer/)

# Additional Examples

- [Airdrop](./airdrop/)
- [BSwap64](./bswap64/)

  ```solidity
  contract C {
      function to_little_endian_64(uint64 value) public pure returns (bytes memory ret) {
          ret = new bytes(8);
          bytes8 bytesValue = bytes8(value);
          // Byteswapping during copying to bytes.
          ret[0] = bytesValue[7];
          ret[1] = bytesValue[6];
          ret[2] = bytesValue[5];
          ret[3] = bytesValue[4];
          ret[4] = bytesValue[3];
          ret[5] = bytesValue[2];
          ret[6] = bytesValue[1];
          ret[7] = bytesValue[0];
      }
  }
  ```

- [BSwap64 OPT](./bswap64_opt/)

  ```solidity
  contract C {
    function to_little_endian_64(uint64 x) public pure returns (bytes memory ret) {
        uint64 swapped =
         (((x & 0xff00000000000000) >> 56)
        | ((x & 0x00ff000000000000) >> 40)
        | ((x & 0x0000ff0000000000) >> 24)
        | ((x & 0x000000ff00000000) >> 8)
        | ((x & 0x00000000ff000000) << 8)
        | ((x & 0x0000000000ff0000) << 24)
        | ((x & 0x000000000000ff00) << 40)
        | ((x & 0x00000000000000ff) << 56));

        ret = new bytes(8);
        assembly {
            mstore(add(ret, 0x20), shl(192, swapped))
        }
    }
  }
  ```

- [Fiat Token](./fiat_token/)
- [Hash 10k](./hash_10k/)
- [Push0 Proxy](./push0_proxy/)
- [Seaport](./seaport/)
- [SnailTracer](./snailtracer/)
- [Uniswap V2 Pair](./uniswap_v2_pair/)
- [UniV2 Router](./univ2_router/)
- [USDC Proxy](./usdc_proxy/)
- [WETH](./weth/)
