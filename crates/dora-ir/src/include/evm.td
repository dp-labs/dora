// This include is necessary to use `Dialect`

// Ref: https://ethereum.org/en/developers/docs/evm/opcodes/

include "mlir/IR/OpBase.td"

def EVM_Dialect : Dialect {
  let name = "evm";
}

class EVM_Op<string mnemonic, list<Trait> traits = []> :
    Op<EVM_Dialect, mnemonic, traits>;

// Stop Operation
def EVM_Stop : EVM_Op<"stop"> {
  let summary = "Halts execution.";
  let description = [{
    This operation halts the execution of the EVM program.
  }];

  let arguments = (ins);
  let results = (outs);

  let assemblyFormat = "";
}

// Add Operation
def EVM_Add : EVM_Op<"add"> {
  let summary = "Adds two 256-bit integers.";
  let description = [{
    This operation takes two `i256` operands from the stack and returns their sum.
  }];

  let arguments = (ins I<256>:$lhs, I<256>:$rhs);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// Mul Operation
def EVM_Mul : EVM_Op<"mul"> {
  let summary = "Multiplies two 256-bit integers.";
  let description = [{
    This operation takes two `i256` operands from the stack and returns their product.
  }];

  let arguments = (ins I<256>:$lhs, I<256>:$rhs);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// Sub Operation
def EVM_Sub : EVM_Op<"sub"> {
  let summary = "Subtracts the second 256-bit integer from the first.";
  let description = [{
    This operation takes two `i256` operands from the stack and returns their difference.
  }];

  let arguments = (ins I<256>:$lhs, I<256>:$rhs);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// Div Operation
def EVM_Div : EVM_Op<"div"> {
  let summary = "Divides two 256-bit integers.";
  let description = [{
    This operation takes two `i256` operands from the stack and returns their quotient.
  }];

  let arguments = (ins I<256>:$lhs, I<256>:$rhs);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// SDiv Operation
def EVM_SDiv : EVM_Op<"sdiv"> {
  let summary = "Divides two signed 256-bit integers.";
  let description = [{
    This operation takes two signed `i256` operands from the stack and returns their quotient.
  }];

  let arguments = (ins I<256>:$lhs, I<256>:$rhs);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// Mod Operation
def EVM_Mod : EVM_Op<"mod"> {
  let summary = "Returns the remainder of dividing two 256-bit integers.";
  let description = [{
    This operation takes two `i256` operands from the stack and returns their remainder.
  }];

  let arguments = (ins I<256>:$lhs, I<256>:$rhs);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// SMod Operation
def EVM_SMod : EVM_Op<"smod"> {
  let summary = "Returns the remainder of dividing two signed 256-bit integers.";
  let description = [{
    This operation takes two signed `i256` operands from the stack and returns their remainder.
  }];

  let arguments = (ins I<256>:$lhs, I<256>:$rhs);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// AddMod Operation
def EVM_AddMod : EVM_Op<"addmod"> {
  let summary = "Adds two 256-bit integers and returns the remainder of the division by a third.";
  let description = [{
    This operation takes three `i256` operands from the stack, adds the first two, and returns the remainder when divided by the third operand.
  }];

  let arguments = (ins I<256>:$lhs, I<256>:$rhs, I<256>:$modulus);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$lhs `,` $rhs `,` $modulus `:` type($lhs) `->` type($result)";
}

// MulMod Operation
def EVM_MulMod : EVM_Op<"mulmod"> {
  let summary = "Multiplies two 256-bit integers and returns the remainder of the division by a third.";
  let description = [{
    This operation takes three `i256` operands from the stack, multiplies the first two, and returns the remainder when divided by the third operand.
  }];

  let arguments = (ins I<256>:$lhs, I<256>:$rhs, I<256>:$modulus);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$lhs `,` $rhs `,` $modulus `:` type($lhs) `->` type($result)";
}

// Exp Operation
def EVM_Exp : EVM_Op<"exp"> {
  let summary = "Exponentiates two 256-bit integers.";
  let description = [{
    This operation takes two `i256` operands from the stack, raises the first operand to the power of the second, and returns the result.
  }];

  let arguments = (ins I<256>:$base, I<256>:$exponent);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$base `,` $exponent `:` type($base) `->` type($result)";
}

// SignExtend Operation
def EVM_SignExtend : EVM_Op<"signextend"> {
  let summary = "Extends the sign of a 256-bit integer.";
  let description = [{
    This operation takes two `i256` operands from the stack. The first operand specifies the byte position to extend from, and the second operand is the value whose sign is extended.
  }];

  let arguments = (ins I<256>:$byte, I<256>:$value);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$byte `,` $value `:` type($byte) `->` type($result)";
}

// Less-than comparison
def EVM_Lt : EVM_Op<"lt"> {
  let summary = "Compares if the first 256-bit integer is less than the second.";
  let description = [{
    This operation takes two `i256` operands from the stack and returns `1` if the first is less than the second, otherwise `0`.
  }];

  let arguments = (ins I<256>:$lhs, I<256>:$rhs);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// Greater-than comparison
def EVM_Gt : EVM_Op<"gt"> {
  let summary = "Compares if the first 256-bit integer is greater than the second.";
  let description = [{
    This operation takes two `i256` operands from the stack and returns `1` if the first is greater than the second, otherwise `0`.
  }];

  let arguments = (ins I<256>:$lhs, I<256>:$rhs);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// Signed less-than comparison
def EVM_Slt : EVM_Op<"slt"> {
  let summary = "Signed comparison if the first 256-bit integer is less than the second.";
  let description = [{
    This operation takes two `i256` operands from the stack, treats them as signed integers, and returns `1` if the first is less than the second, otherwise `0`.
  }];

  let arguments = (ins I<256>:$lhs, I<256>:$rhs);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// Signed greater-than comparison
def EVM_Sgt : EVM_Op<"sgt"> {
  let summary = "Signed comparison if the first 256-bit integer is greater than the second.";
  let description = [{
    This operation takes two `i256` operands from the stack, treats them as signed integers, and returns `1` if the first is greater than the second, otherwise `0`.
  }];

  let arguments = (ins I<256>:$lhs, I<256>:$rhs);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// Equality comparison
def EVM_Eq : EVM_Op<"eq"> {
  let summary = "Checks if two 256-bit integers are equal.";
  let description = [{
    This operation takes two `i256` operands from the stack and returns `1` if they are equal, otherwise `0`.
  }];

  let arguments = (ins I<256>:$lhs, I<256>:$rhs);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// 	Is-zero comparison
def EVM_IsZero : EVM_Op<"iszero"> {
  let summary = "Checks if a 256-bit integer is zero.";
  let description = [{
    This operation takes one `i256` operand from the stack and returns `1` if it is zero, otherwise `0`.
  }];

  let arguments = (ins I<256>:$value);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$value `:` type($value) `->` type($result)";
}

// Bitwise AND operation
def EVM_And : EVM_Op<"and"> {
  let summary = "Bitwise AND operation on two 256-bit integers.";
  let description = [{
    This operation takes two `i256` operands from the stack and returns their bitwise AND result.
  }];

  let arguments = (ins I<256>:$lhs, I<256>:$rhs);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// Bitwise OR operation
def EVM_Or : EVM_Op<"or"> {
  let summary = "Bitwise OR operation on two 256-bit integers.";
  let description = [{
    This operation takes two `i256` operands from the stack and returns their bitwise OR result.
  }];

  let arguments = (ins I<256>:$lhs, I<256>:$rhs);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// Bitwise XOR operation
def EVM_Xor : EVM_Op<"xor"> {
  let summary = "Bitwise XOR operation on two 256-bit integers.";
  let description = [{
    This operation takes two `i256` operands from the stack and returns their bitwise XOR result.
  }];

  let arguments = (ins I<256>:$lhs, I<256>:$rhs);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// Bitwise NOT operation
def EVM_Not : EVM_Op<"not"> {
  let summary = "Bitwise NOT operation on a 256-bit integer.";
  let description = [{
    This operation takes one `i256` operand from the stack and returns its bitwise NOT result.
  }];

  let arguments = (ins I<256>:$value);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$value `:` type($value) `->` type($result)";
}

// Retrieve single byte from word
def EVM_Byte : EVM_Op<"byte"> {
  let summary = "Extracts a byte from a 256-bit integer.";
  let description = [{
    This operation takes two `i256` operands from the stack: an index and a value. It returns the byte at the specified index from the 256-bit integer.
  }];

  let arguments = (ins I<256>:$index, I<256>:$value);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$index `,` $value `:` type($index) `->` type($result)";
}

// Left shift operation
def EVM_Shl : EVM_Op<"shl"> {
  let summary = "Left shifts a 256-bit integer by a specified number of bits.";
  let description = [{
    This operation takes two `i256` operands from the stack: the shift amount and the value. It shifts the value left by the specified number of bits.
  }];

  let arguments = (ins I<256>:$shift, I<256>:$value);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$shift `,` $value `:` type($shift) `->` type($result)";
}

// Logical right shift operation
def EVM_Shr : EVM_Op<"shr"> {
  let summary = "Logical right shifts a 256-bit integer by a specified number of bits.";
  let description = [{
    This operation takes two `i256` operands from the stack: the shift amount and the value. It shifts the value right logically (filling with zeros) by the specified number of bits.
  }];

  let arguments = (ins I<256>:$shift, I<256>:$value);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$shift `,` $value `:` type($shift) `->` type($result)";
}

// Arithmetic (signed) right shift operation
def EVM_Sar : EVM_Op<"sar"> {
  let summary = "Arithmetic right shifts a 256-bit integer by a specified number of bits.";
  let description = [{
    This operation takes two `i256` operands from the stack: the shift amount and the value. It shifts the value right arithmetically (preserving the sign) by the specified number of bits.
  }];

  let arguments = (ins I<256>:$shift, I<256>:$value);
  let results = (outs I<256>:$result);

  let assemblyFormat = "$shift `,` $value `:` type($shift) `->` type($result)";
}

// Keccak256 Operation
def EVM_Keccak256 : EVM_Op<"keccak256"> {
  let summary = "Computes the Keccak-256 hash of a memory region.";
  let description = [{
    This operation takes two `i256` operands from the stack, which specify the starting position and the length of the memory region to hash, and returns the 256-bit hash.
  }];

  let arguments = (ins I<256>:$start, I<256>:$length);
  let results = (outs I<256>:$hash);

  let assemblyFormat = "$start `,` $length `:` type($start) `->` type($hash)";
}

// Address Operation
def EVM_Address : EVM_Op<"address"> {
  let summary = "Gets the address of the executing contract.";
  let description = [{
    This operation returns the address of the contract currently executing.
  }];

  let arguments = (ins);
  let results = (outs I<256>:$address);

  let assemblyFormat = "`->` type($address)";
}

// Balance Operation
def EVM_Balance : EVM_Op<"balance"> {
  let summary = "Gets the balance of an account.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the address of an account, and returns the balance of that account.
  }];

  let arguments = (ins I<256>:$account);
  let results = (outs I<256>:$balance);

  let assemblyFormat = "$account `:` type($account) `->` type($balance)";
}

// Origin Operation
def EVM_Origin : EVM_Op<"origin"> {
  let summary = "Gets the address of the originator of the transaction.";
  let description = [{
    This operation returns the address of the account that sent the transaction.
  }];

  let arguments = (ins);
  let results = (outs I<256>:$origin);

  let assemblyFormat = "`->` type($origin)";
}

// Caller Operation
def EVM_Caller : EVM_Op<"caller"> {
  let summary = "Gets the address of the caller of the current execution.";
  let description = [{
    This operation returns the address of the account that called the current contract.
  }];

  let arguments = (ins);
  let results = (outs I<256>:$caller);

  let assemblyFormat = "`->` type($caller)";
}

// CallValue Operation
def EVM_CallValue : EVM_Op<"callvalue"> {
  let summary = "Gets the value (in Wei) sent with the call.";
  let description = [{
    This operation returns the amount of Wei sent with the call.
  }];

  let arguments = (ins);
  let results = (outs I<256>:$value);

  let assemblyFormat = "`->` type($value)";
}

// CalldataLoad Operation
def EVM_CalldataLoad : EVM_Op<"calldataload"> {
  let summary = "Loads a word of input data.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the byte offset in the input data, and returns the 256-bit word at that offset.
  }];

  let arguments = (ins I<256>:$offset);
  let results = (outs I<256>:$value);

  let assemblyFormat = "$offset `:` type($offset) `->` type($data)";
}

// CalldataSize Operation
def EVM_CalldataSize : EVM_Op<"calldatasize"> {
  let summary = "Gets the size of the input data.";
  let description = [{
    This operation returns the size of the input data for the current call.
  }];

  let arguments = (ins);
  let results = (outs I<256>:$size);

  let assemblyFormat = "`->` type($size)";
}

// CalldataCopy Operation
def EVM_CalldataCopy : EVM_Op<"calldatacopy"> {
  let summary = "Copies a portion of the input data to memory.";
  let description = [{
    This operation takes three `i256` operands from the stack: the destination memory offset, the source offset in the input data, and the number of bytes to copy.
  }];

  let arguments = (ins I<256>:$destOffset, I<256>:$dataOffset, I<256>:$length);
  let results = (outs);

  let assemblyFormat = "$destOffset `,` $dataOffset `,` $length `:` type($destOffset)";
}

// CodeSize Operation
def EVM_CodeSize : EVM_Op<"codesize"> {
  let summary = "Gets the size of the contract's code.";
  let description = [{
    This operation returns the size of the code in bytes of the contract currently executing.
  }];

  let arguments = (ins);
  let results = (outs I<256>:$size);

  let assemblyFormat = "`->` type($size)";
}

// CodeCopy Operation
def EVM_CodeCopy : EVM_Op<"codecopy"> {
  let summary = "Copies a portion of the contract's code to memory.";
  let description = [{
    This operation takes three `i256` operands from the stack: the destination memory offset, the source offset in the contract code, and the number of bytes to copy.
  }];

  let arguments = (ins I<256>:$destOffset, I<256>:$codeOffset, I<256>:$length);
  let results = (outs);

  let assemblyFormat = "$destOffset `,` $codeOffset `,` $length `:` type($destOffset)";
}

// GasPrice Operation
def EVM_GasPrice : EVM_Op<"gasprice"> {
  let summary = "Gets the gas price of the current transaction.";
  let description = [{
    This operation returns the gas price in Wei for the current transaction.
  }];

  let arguments = (ins);
  let results = (outs I<256>:$price);

  let assemblyFormat = "`->` type($price)";
}

// ExtCodeSize Operation
def EVM_ExtCodeSize : EVM_Op<"extcodesize"> {
  let summary = "Gets the size of another account's code.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the address of an account, and returns the size of that account's code in bytes.
  }];

  let arguments = (ins I<256>:$account);
  let results = (outs I<256>:$size);

  let assemblyFormat = "$account `:` type($account) `->` type($size)";
}

// ExtCodeCopy Operation
def EVM_ExtCodeCopy : EVM_Op<"extcodecopy"> {
  let summary = "Copies a portion of another account's code to memory.";
  let description = [{
    This operation takes four `i256` operands from the stack: the account address, the destination memory offset, the source offset in the account's code, and the number of bytes to copy.
  }];

  let arguments = (ins I<256>:$account, I<256>:$destOffset, I<256>:$codeOffset, I<256>:$length);
  let results = (outs);

  let assemblyFormat = "$account `,` $destOffset `,` $codeOffset `,` $length `:` type($account)";
}

// ReturndataSize Operation
def EVM_ReturndataSize : EVM_Op<"returndatasize"> {
  let summary = "Gets the size of the returned data from the last external call.";
  let description = [{
    This operation returns the size of the data returned by the last external call.
  }];

  let arguments = (ins);
  let results = (outs I<256>:$size);

  let assemblyFormat = "`->` type($size)";
}

// ReturndataCopy Operation
def EVM_ReturndataCopy : EVM_Op<"returndatacopy"> {
  let summary = "Copies the returned data from the last external call to memory.";
  let description = [{
    This operation takes three `i256` operands from the stack: the destination memory offset, the source offset in the returned data, and the number of bytes to copy.
  }];

  let arguments = (ins I<256>:$destOffset, I<256>:$dataOffset, I<256>:$length);
  let results = (outs);

  let assemblyFormat = "$destOffset `,` $dataOffset `,` $length `:` type($destOffset)";
}

// ExtCodeHash Operation
def EVM_ExtCodeHash : EVM_Op<"extcodehash"> {
  let summary = "Gets the hash of another account's code.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the address of an account, and returns the hash of that account's code.
  }];

  let arguments = (ins I<256>:$account);
  let results = (outs I<256>:$hash);

  let assemblyFormat = "$account `:` type($account) `->` type($hash)";
}

// BlockHash Operation
def EVM_BlockHash : EVM_Op<"blockhash"> {
  let summary = "Gets the block hash of a recent block.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the block number, and returns the hash of the specified block. Only the most recent 256 blocks are accessible.
  }];

  let arguments = (ins I<256>:$blockNumber);
  let results = (outs I<256>:$hash);

  let assemblyFormat = "$blockNumber `:` type($blockNumber) `->` type($hash)";
}

// Coinbase Operation
def EVM_Coinbase : EVM_Op<"coinbase"> {
  let summary = "Gets the address of the current block miner.";
  let description = [{
    This operation returns the address of the account that is currently mining the block.
  }];

  let arguments = (ins);
  let results = (outs I<256>:$minerAddress);

  let assemblyFormat = "`->` type($minerAddress)";
}

// Timestamp Operation
def EVM_Timestamp : EVM_Op<"timestamp"> {
  let summary = "Gets the timestamp of the current block.";
  let description = [{
    This operation returns the Unix timestamp of when the current block was mined.
  }];

  let arguments = (ins);
  let results = (outs I<256>:$timestamp);

  let assemblyFormat = "`->` type($timestamp)";
}

// Number Operation
def EVM_Number : EVM_Op<"number"> {
  let summary = "Gets the block number of the current block.";
  let description = [{
    This operation returns the block number of the current block.
  }];

  let arguments = (ins);
  let results = (outs I<256>:$blockNumber);

  let assemblyFormat = "`->` type($blockNumber)";
}

// PrevRandao Operation
def EVM_PrevRandao : EVM_Op<"prevrandao"> {
  let summary = "Gets the previous block's RANDAO value.";
  let description = [{
    This operation returns the RANDAO value from the previous block.
  }];

  let arguments = (ins);
  let results = (outs I<256>:$randao);

  let assemblyFormat = "`->` type($randao)";
}

// GasLimit Operation
def EVM_GasLimit : EVM_Op<"gaslimit"> {
  let summary = "Gets the gas limit of the current block.";
  let description = [{
    This operation returns the gas limit of the block currently being mined.
  }];

  let arguments = (ins);
  let results = (outs I<256>:$gasLimit);

  let assemblyFormat = "`->` type($gasLimit)";
}

// ChainId Operation
def EVM_ChainId : EVM_Op<"chainid"> {
  let summary = "Gets the chain ID of the current network.";
  let description = [{
    This operation returns the chain ID of the network currently being used.
  }];

  let arguments = (ins);
  let results = (outs I<256>:$chainId);

  let assemblyFormat = "`->` type($chainId)";
}

// SelfBalance Operation
def EVM_SelfBalance : EVM_Op<"selfbalance"> {
  let summary = "Gets the balance of the executing contract.";
  let description = [{
    This operation returns the balance of the contract currently executing.
  }];

  let arguments = (ins);
  let results = (outs I<256>:$balance);

  let assemblyFormat = "`->` type($balance)";
}

// BaseFee Operation
def EVM_BaseFee : EVM_Op<"basefee"> {
  let summary = "Gets the base fee per gas for the current block.";
  let description = [{
    This operation returns the base fee per gas for the block currently being mined.
  }];

  let arguments = (ins);
  let results = (outs I<256>:$baseFee);

  let assemblyFormat = "`->` type($baseFee)";
}

// BlobHash Operation
def EVM_BlobHash : EVM_Op<"blobhash"> {
  let summary = "Gets the hash of the blob at the specified index.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the index of the blob, and returns the hash of that blob.
  }];

  let arguments = (ins I<256>:$index);
  let results = (outs I<256>:$hash);

  let assemblyFormat = "$index `:` type($index) `->` type($hash)";
}

// BlobBaseFee Operation
def EVM_BlobBaseFee : EVM_Op<"blobbasefee"> {
  let summary = "Gets the base fee per blob for the current block.";
  let description = [{
    This operation returns the base fee per blob for the block currently being mined.
  }];

  let arguments = (ins);
  let results = (outs I<256>:$baseFee);

  let assemblyFormat = "`->` type($baseFee)";
}

// MLoad Operation
def EVM_MLoad : EVM_Op<"mload"> {
  let summary = "Loads a 256-bit word from memory.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the memory offset, and returns the 256-bit word stored at that memory location.
  }];

  let arguments = (ins I<256>:$offset);
  let results = (outs I<256>:$data);

  let assemblyFormat = "$offset `:` type($offset) `->` type($data)";
}

// MStore Operation
def EVM_MStore : EVM_Op<"mstore"> {
  let summary = "Stores a 256-bit word in memory.";
  let description = [{
    This operation takes two `i256` operands from the stack: the memory offset and the 256-bit word to store at that memory location.
  }];

  let arguments = (ins I<256>:$offset, I<256>:$data);
  let results = (outs);

  let assemblyFormat = "$offset `,` $data `:` type($offset)";
}

// MStore8 Operation
def EVM_MStore8 : EVM_Op<"mstore8"> {
  let summary = "Stores an 8-bit byte in memory.";
  let description = [{
    This operation takes two `i256` operands from the stack: the memory offset and the 8-bit byte to store at that memory location.
  }];

  let arguments = (ins I<256>:$offset, I<256>:$data);
  let results = (outs);

  let assemblyFormat = "$offset `,` $data `:` type($offset)";
}

// SLoad Operation
def EVM_SLoad : EVM_Op<"sload"> {
  let summary = "Loads a 256-bit word from storage.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the storage key, and returns the 256-bit word stored at that key.
  }];

  let arguments = (ins I<256>:$key);
  let results = (outs I<256>:$value);

  let assemblyFormat = "$key `:` type($key) `->` type($value)";
}

// SStore Operation
def EVM_SStore : EVM_Op<"sstore"> {
  let summary = "Stores a 256-bit word in storage.";
  let description = [{
    This operation takes two `i256` operands from the stack: the storage key and the 256-bit word to store at that key.
  }];

  let arguments = (ins I<256>:$key, I<256>:$value);
  let results = (outs);

  let assemblyFormat = "$key `,` $value `:` type($key)";
}

// MSize Operation
def EVM_MSize : EVM_Op<"msize"> {
  let summary = "Gets the size of the memory in bytes.";
  let description = [{
    This operation returns the current size of the memory in bytes.
  }];

  let arguments = (ins);
  let results = (outs I<256>:$size);

  let assemblyFormat = "`->` type($size)";
}

// Gas Operation
def EVM_Gas : EVM_Op<"gas"> {
  let summary = "Gets the remaining gas.";
  let description = [{
    This operation returns the amount of gas remaining for the execution.
  }];

  let arguments = (ins);
  let results = (outs I<256>:$gas);

  let assemblyFormat = "`->` type($gas)";
}

// TLoad Operation
def EVM_TLoad : EVM_Op<"tload"> {
  let summary = "Loads a 256-bit word from the transient storage.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the transient storage key, and returns the 256-bit word stored at that key.
  }];

  let arguments = (ins I<256>:$key);
  let results = (outs I<256>:$value);

  let assemblyFormat = "$key `:` type($key) `->` type($value)";
}

// TStore Operation
def EVM_TStore : EVM_Op<"tstore"> {
  let summary = "Stores a 256-bit word in the transient storage.";
  let description = [{
    This operation takes two `i256` operands from the stack: the transient storage key and the 256-bit word to store at that key.
  }];

  let arguments = (ins I<256>:$key, I<256>:$value);
  let results = (outs);

  let assemblyFormat = "$key `,` $value `:` type($key)";
}

// MCopy Operation
def EVM_MCopy : EVM_Op<"mcopy"> {
  let summary = "Copies a portion of the memory to another location in memory.";
  let description = [{
    This operation takes three `i256` operands from the stack: the destination memory offset, the source memory offset, and the number of bytes to copy.
  }];

  let arguments = (ins I<256>:$destOffset, I<256>:$srcOffset, I<256>:$length);
  let results = (outs);

  let assemblyFormat = "$destOffset `,` $srcOffset `,` $length `:` type($destOffset)";
}

// Log0 Operation
def EVM_Log0 : EVM_Op<"log0"> {
  let summary = "Logs one topic with no data.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the memory offset, and logs an event with no additional topics.
  }];

  let arguments = (ins I<256>:$offset, I<256>:$length);
  let results = (outs);

  let assemblyFormat = "$offset `,` $length `:` type($offset)";
}

// Log1 Operation
def EVM_Log1 : EVM_Op<"log1"> {
  let summary = "Logs one topic with data.";
  let description = [{
    This operation takes two `i256` operands from the stack: the memory offset and the topic, and logs an event with one additional topic.
  }];

  let arguments = (ins I<256>:$offset, I<256>:$length, I<256>:$topic);
  let results = (outs);

  let assemblyFormat = "$offset `,` $length `,` $topic `:` type($offset)";
}

// Log2 Operation
def EVM_Log2 : EVM_Op<"log2"> {
  let summary = "Logs two topics with data.";
  let description = [{
    This operation takes three `i256` operands from the stack: the memory offset, the length, and two topics, and logs an event with two additional topics.
  }];

  let arguments = (ins I<256>:$offset, I<256>:$length, I<256>:$topic1, I<256>:$topic2);
  let results = (outs);

  let assemblyFormat = "$offset `,` $length `,` $topic1 `,` $topic2 `:` type($offset)";
}

// Log3 Operation
def EVM_Log3 : EVM_Op<"log3"> {
  let summary = "Logs three topics with data.";
  let description = [{
    This operation takes four `i256` operands from the stack: the memory offset, the length, and three topics, and logs an event with three additional topics.
  }];

  let arguments = (ins I<256>:$offset, I<256>:$length, I<256>:$topic1, I<256>:$topic2, I<256>:$topic3);
  let results = (outs);

  let assemblyFormat = "$offset `,` $length `,` $topic1 `,` $topic2 `,` $topic3 `:` type($offset)";
}

// Log4 Operation
def EVM_Log4 : EVM_Op<"log4"> {
  let summary = "Logs four topics with data.";
  let description = [{
    This operation takes five `i256` operands from the stack: the memory offset, the length, and four topics, and logs an event with four additional topics.
  }];

  let arguments = (ins I<256>:$offset, I<256>:$length, I<256>:$topic1, I<256>:$topic2, I<256>:$topic3, I<256>:$topic4);
  let results = (outs);

  let assemblyFormat = "$offset `,` $length `,` $topic1 `,` $topic2 `,` $topic3 `,` $topic4 `:` type($offset)";
}

// DataLoad Operation
def EVM_DataLoad : EVM_Op<"dataload"> {
  let summary = "Load a word of the data section.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the byte offset in the data section of EOF container, and returns the 256-bit word at that offset.
  }];

  let arguments = (ins I<256>:$offset);

  let results = (outs I<256>:$data);

  let assemblyFormat = "$offset `:` type($offset) `->` type($data)";
}

// DataLoadN Operation
def EVM_DataLoadN : EVM_Op<"dataloadn"> {
  let summary = "Load a word of the data section.";
  let description = [{
    This operation takes one `i16` argument, which is the byte offset in the data section of EOF container, and returns the 256-bit word at that offset.
  }];

  let arguments = (ins I16:$offset);

  let results = (outs I<256>:$data);

  let assemblyFormat = "$offset `:` type($offset) `->` type($data)";
}

// DataSize Operation
def EVM_DataSize : EVM_Op<"datasize"> {
  let summary = "Gets the size of the data section.";
  let description = [{
    This operation returns the size of the data section for the current call.
  }];

  let arguments = (ins);
  let results = (outs I<256>:$size);

  let assemblyFormat = "`->` type($size)";
}

// DataCopy Operation
def EVM_DataCopy : EVM_Op<"datacopy"> {
  let summary = "Copies a portion of the data section to memory.";
  let description = [{
    This operation takes three `i256` operands from the stack: the destination memory offset, the source offset in the data section, and the number of bytes to copy.
  }];

  let arguments = (ins I<256>:$memOffset, I<256>:$offset, I<256>:$size);
  let results = (outs);

  let assemblyFormat = "$memOffset `,` $offset `,` $size `:` type($memOffset)";
}

// CallF Operation
def EVM_CallF : EVM_Op<"callf"> {
  let summary = "Calls a function.";
  let description = [{
    This operation takes one `i16` argument, which is the metadata index in the type section, and call into a function or subroutine, then push an execution state onto the return stack.
  }];

  let arguments = (ins I16:$target_section_index);
  let results = (outs);

  let assemblyFormat = "$target_section_index `:` type($target_section_index)";
}

// RetF Operation
def EVM_RetF : EVM_Op<"retf"> {
  let summary = "Returns data from the function execution.";
  let description = [{
    This operations sets the Context from CALLF, a function or subroutine, to corresponding values (current section index, program counter).
  }];

  let arguments = (ins);
  let results = (outs);

  let assemblyFormat = "";
}

// EofCreate Operation
def EVM_EofCreate : EVM_Op<"eofcreate"> {
  let summary = "Creates a new contract with a deterministic address.";
  let description = [{
    This operation takes one `i8` argument, which is the index of EOF subcontainer in the containers, and four `i256` operands from the stack: the amount of Ether to send, a salt value, the memory offset of the initialization code and the length of the initialization code, and It returns the address of the created contract, which is determined using the salt and the caller's address.
  }];

  let arguments = (ins I8:$initContainerIndex, I<256>:$value, I<256>:$salt, I<256>:$inputOffset, I<256>:$inputSize);
  let results = (outs I<256>:$address);

  let assemblyFormat = "$initContainerIndex `,` $value `,` $salt `,` $inputOffset `,` $inputSize `:` type($initContainerIndex) `->` type($address)";
}

// ReturnContract Operation
def EVM_ReturnContract : EVM_Op<"returncontract"> {
  let summary = "Appends auxiliary data from memory stack to deployed contract";
  let description = [{
    This operation takes one `i8` argument, which is the index of EOF containers and two `i256` operand from the stack:  the memory offset of the auxiliary data and the length of the auxiliary data to copy, and It appended to deployed container's data.
  }];

  let arguments = (ins I8:$deployContainerIndex, I<256>:$auxDataOffset, I<256>:$auxDataSize);
  let results = (outs);

  let assemblyFormat = "$deployContainerIndex `,` $auxDataOffset `,` $auxDataSize `:` type($deployContainerIndex)";
}

// Create Operation
def EVM_Create : EVM_Op<"create"> {
  let summary = "Creates a new contract.";
  let description = [{
    This operation takes three `i256` operands from the stack: the amount of Ether to send, the memory offset of the initialization code, and the length of the initialization code, and returns the address of the created contract.
  }];

  let arguments = (ins I<256>:$value, I<256>:$offset, I<256>:$length);
  let results = (outs I<256>:$address);

  let assemblyFormat = "$value `,` $offset `,` $length `:` type($value) `->` type($address)";
}

// Call Operation
def EVM_Call : EVM_Op<"call"> {
  let summary = "Calls a contract.";
  let description = [{
    This operation takes seven `i256` operands from the stack: the gas, the address, the amount of Ether to send, the memory offset of the input data, the length of the input data, the memory offset of the output data, and the length of the output data, and returns 1 if the call succeeds and 0 if it fails.
  }];

  let arguments = (ins I<256>:$gas, I<256>:$address, I<256>:$value, I<256>:$inputOffset, I<256>:$inputLength, I<256>:$outputOffset, I<256>:$outputLength);
  let results = (outs I<256>:$success);

  let assemblyFormat = "$gas `,` $address `,` $value `,` $inputOffset `,` $inputLength `,` $outputOffset `,` $outputLength `:` type($gas) `->` type($success)";
}

// Callcode Operation
def EVM_Callcode : EVM_Op<"callcode"> {
  let summary = "Calls a contract's code without modifying its storage.";
  let description = [{
    This operation is similar to `call`, but it uses the caller's storage rather than the storage of the called contract.
  }];

  let arguments = (ins I<256>:$gas, I<256>:$address, I<256>:$value, I<256>:$inputOffset, I<256>:$inputLength, I<256>:$outputOffset, I<256>:$outputLength);
  let results = (outs I<256>:$success);

  let assemblyFormat = "$gas `,` $address `,` $value `,` $inputOffset `,` $inputLength `,` $outputOffset `,` $outputLength `:` type($gas) `->` type($success)";
}

// Return Operation
def EVM_Return : EVM_Op<"return"> {
  let summary = "Returns data from the contract execution.";
  let description = [{
    This operation takes two `i256` operands from the stack: the memory offset of the return data and the length of the return data, and returns from the execution with the specified data.
  }];

  let arguments = (ins I<256>:$offset, I<256>:$length);
  let results = (outs);

  let assemblyFormat = "$offset `,` $length `:` type($offset)";
}

// Delegatecall Operation
def EVM_Delegatecall : EVM_Op<"delegatecall"> {
  let summary = "Calls a contract while keeping the caller's context.";
  let description = [{
    This operation is similar to `callcode`, but it also keeps the caller's sender and value.
  }];

  let arguments = (ins I<256>:$gas, I<256>:$address, I<256>:$inputOffset, I<256>:$inputLength, I<256>:$outputOffset, I<256>:$outputLength);
  let results = (outs I<256>:$success);

  let assemblyFormat = "$gas `,` $address `,` $inputOffset `,` $inputLength `,` $outputOffset `,` $outputLength `:` type($gas) `->` type($success)";
}

// Create2 Operation
def EVM_Create2 : EVM_Op<"create2"> {
  let summary = "Creates a new contract with a deterministic address.";
  let description = [{
    This operation takes four `i256` operands from the stack: the amount of Ether to send, the memory offset of the initialization code, the length of the initialization code, and a salt value. It returns the address of the created contract, which is determined using the salt and the caller's address.
  }];

  let arguments = (ins I<256>:$value, I<256>:$offset, I<256>:$length, I<256>:$salt);
  let results = (outs I<256>:$address);

  let assemblyFormat = "$value `,` $offset `,` $length `,` $salt `:` type($value) `->` type($address)";
}

// ReturndataLoad Operation
def EVM_ReturndataLoad : EVM_Op<"returndataload"> {
  let summary = "Loads a word from the returned data from the last external call.";
  let description = [{
    This operation takes one `i256` operand from the stack: the offset in the returned data, and pushes the loaded word onto the stack.
  }];

  let arguments = (ins I<256>:$offset);
  let results = (outs I<256>:$data);

  let assemblyFormat = "$offset `:` type($offset) `->` type($data)";
}

// ExtCall Operation
def EVM_ExtCall : EVM_Op<"extcall"> {
  let summary = "Similar to Call.";
  let description = [{
    This operation takes four `i256` operands from the stack: the target address, the memory offset of the input data, the length of the input data to copy, the amount of Ether to send, and It returns 0 if the call succeeds, 1 for revert if sub context reverted and 2 if it fails.
  }];

  let arguments = (ins I<256>:$targetAddress, I<256>:$inputOffset, I<256>:$inputSize, I<256>:$value);
  let results = (outs I<256>:$status);

  let assemblyFormat = "$targetAddress `,` $inputOffset `,` $inputSize `,` $value `:` type($targetAddress) `->` type($status)";
}

// ExtDelegatecall Operation
def EVM_ExtDelegatecall : EVM_Op<"extdelegatecall"> {
  let summary = "Similar to Delegatecall.";
  let description = [{
    This operation is similar to `extcall`, but it also keeps the caller's sender and value.
  }];

  let arguments = (ins I<256>:$targetAddress, I<256>:$inputOffset, I<256>:$inputSize);
  let results = (outs I<256>:$status);

  let assemblyFormat = "$targetAddress `,` $inputOffset `,` $inputSize `:` type($targetAddress) `->` type($status)";
}

// Staticcall Operation
def EVM_Staticcall : EVM_Op<"staticcall"> {
  let summary = "Calls a contract in a static context.";
  let description = [{
    This operation is similar to `call`, but it does not allow state modifications.
  }];

  let arguments = (ins I<256>:$gas, I<256>:$address, I<256>:$inputOffset, I<256>:$inputLength, I<256>:$outputOffset, I<256>:$outputLength);
  let results = (outs I<256>:$success);

  let assemblyFormat = "$gas `,` $address `,` $inputOffset `,` $inputLength `,` $outputOffset `,` $outputLength `:` type($gas) `->` type($success)";
}

// ExtStaticcall Operation
def EVM_ExtStaticcall : EVM_Op<"extstaticcall"> {
  let summary = "Similar to Staticcall.";
  let description = [{
    This operation is similar to `extcall`, but it does not allow state modifications.
  }];

  let arguments = (ins I<256>:$targetAddress, I<256>:$inputOffset, I<256>:$inputSize);
  let results = (outs I<256>:$status);

  let assemblyFormat = "$targetAddress `,` $inputOffset `,` $inputSize `:` type($targetAddress) `->` type($status)";
}

// Revert Operation
def EVM_Revert : EVM_Op<"revert"> {
  let summary = "Reverts the execution and returns data.";
  let description = [{
    This operation takes two `i256` operands from the stack: the memory offset of the return data and the length of the return data, and reverts the execution, returning the specified data.
  }];

  let arguments = (ins I<256>:$offset, I<256>:$length);
  let results = (outs);

  let assemblyFormat = "$offset `,` $length `:` type($offset)";
}

// Invalid Operation
def EVM_Invalid : EVM_Op<"invalid"> {
  let summary = "Halts execution with an invalid opcode.";
  let description = [{
    This operation halts the execution with an invalid opcode, causing an exception and reverting any state changes made during the transaction.
  }];

  let arguments = (ins);
  let results = (outs);

  let assemblyFormat = "";
}

// Selfdestruct Operation
def EVM_Selfdestruct : EVM_Op<"selfdestruct"> {
  let summary = "Terminates the contract and sends its balance to a specified address.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the address to send the remaining balance to, and terminates the contract, removing it from the blockchain.
  }];

  let arguments = (ins I<256>:$recipient);
  let results = (outs);

  let assemblyFormat = "$recipient `:` type($recipient)";
}