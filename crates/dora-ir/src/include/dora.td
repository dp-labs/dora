// This include is necessary to use `Dialect`
include "mlir/IR/OpBase.td"

def Dora_Dialect : Dialect {
  let name = "dora";
}

class Dora_Op<string mnemonic, list<Trait> traits = []> :
    Op<Dora_Dialect, mnemonic, traits>;

// Add Operation
def Dora_Add : Dora_Op<"add"> {
  let summary = "Add two operands of the same type";
  let description = [{
      This operation takes two operands of the same type and returns their sum.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$lhs,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rhs
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = [{
    $lhs `,` $rhs `:` type($lhs) `->` type($result)
  }];
}

// Mul Operation
def Dora_Mul : Dora_Op<"mul"> {
  let summary = "Multiplies two 256-bit integers.";
  let description = [{
    This operation takes two `i256` operands from the stack and returns their product.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$lhs,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rhs
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// Sub Operation
def Dora_Sub : Dora_Op<"sub"> {
  let summary = "Subtracts the second 256-bit integer from the first.";
  let description = [{
    This operation takes two `i256` operands from the stack and returns their difference.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$lhs,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rhs
  );

    let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);


  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// Div Operation
def Dora_Div : Dora_Op<"div"> {
  let summary = "Divides two 256-bit integers.";
  let description = [{
    This operation takes two `i256` operands from the stack and returns their quotient.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$lhs,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rhs
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// SDiv Operation
def Dora_SDiv : Dora_Op<"sdiv"> {
  let summary = "Divides two signed 256-bit integers.";
  let description = [{
    This operation takes two signed `i256` operands from the stack and returns their quotient.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$lhs,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rhs
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// Mod Operation
def Dora_Mod : Dora_Op<"mod"> {
  let summary = "Returns the remainder of dividing two 256-bit integers.";
  let description = [{
    This operation takes two `i256` operands from the stack and returns their remainder.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$lhs,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rhs
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// SMod Operation
def Dora_SMod : Dora_Op<"smod"> {
  let summary = "Returns the remainder of dividing two signed 256-bit integers.";
  let description = [{
    This operation takes two signed `i256` operands from the stack and returns their remainder.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$lhs,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rhs
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// AddMod Operation
def Dora_AddMod : Dora_Op<"addmod"> {
  let summary = "Adds two 256-bit integers and returns the remainder of the division by a third.";
  let description = [{
    This operation takes three `i256` operands from the stack, adds the first two, and returns the remainder when divided by the third operand.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$lhs,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rhs,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$modulus
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$lhs `,` $rhs `,` $modulus `:` type($lhs) `->` type($result)";
}

// MulMod Operation
def Dora_MulMod : Dora_Op<"mulmod"> {
  let summary = "Multiplies two 256-bit integers and returns the remainder of the division by a third.";
  let description = [{
    This operation takes three `i256` operands from the stack, multiplies the first two, and returns the remainder when divided by the third operand.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$lhs,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rhs,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$modulus
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$lhs `,` $rhs `,` $modulus `:` type($lhs) `->` type($result)";
}

// Exp Operation
def Dora_Exp : Dora_Op<"exp"> {
  let summary = "Exponentiates two 256-bit integers.";
  let description = [{
    This operation takes two `i256` operands from the stack, raises the first operand to the power of the second, and returns the result.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$lhs,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rhs
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$base `,` $exponent `:` type($base) `->` type($result)";
}

// SignExtend Operation
def Dora_SignExtend : Dora_Op<"signextend"> {
  let summary = "Extends the sign of a 256-bit integer.";
  let description = [{
    This operation takes two `i256` operands from the stack. The first operand specifies the byte position to extend from, and the second operand is the value whose sign is extended.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$lhs,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rhs
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$byte `,` $value `:` type($byte) `->` type($result)";
}

// Less-than comparison
def Dora_Lt : Dora_Op<"lt"> {
  let summary = "Compares if the first 256-bit integer is less than the second.";
  let description = [{
    This operation takes two `i256` operands from the stack and returns `1` if the first is less than the second, otherwise `0`.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$lhs,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rhs
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// Greater-than comparison
def Dora_Gt : Dora_Op<"gt"> {
  let summary = "Compares if the first 256-bit integer is greater than the second.";
  let description = [{
    This operation takes two `i256` operands from the stack and returns `1` if the first is greater than the second, otherwise `0`.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$lhs,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rhs
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// Signed less-than comparison
def Dora_Slt : Dora_Op<"slt"> {
  let summary = "Signed comparison if the first 256-bit integer is less than the second.";
  let description = [{
    This operation takes two `i256` operands from the stack, treats them as signed integers, and returns `1` if the first is less than the second, otherwise `0`.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$lhs,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rhs
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// Signed greater-than comparison
def Dora_Sgt : Dora_Op<"sgt"> {
  let summary = "Signed comparison if the first 256-bit integer is greater than the second.";
  let description = [{
    This operation takes two `i256` operands from the stack, treats them as signed integers, and returns `1` if the first is greater than the second, otherwise `0`.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$lhs,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rhs
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// Equality comparison
def Dora_Eq : Dora_Op<"eq"> {
  let summary = "Checks if two 256-bit integers are equal.";
  let description = [{
    This operation takes two `i256` operands from the stack and returns `1` if they are equal, otherwise `0`.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$lhs,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rhs
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// 	Is-zero comparison
def Dora_IsZero : Dora_Op<"iszero"> {
  let summary = "Checks if a 256-bit integer is zero.";
  let description = [{
    This operation takes one `i256` operand from the stack and returns `1` if it is zero, otherwise `0`.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$value
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$value `:` type($value) `->` type($result)";
}

// Bitwise AND operation
def Dora_And : Dora_Op<"and"> {
  let summary = "Bitwise AND operation on two 256-bit integers.";
  let description = [{
    This operation takes two `i256` operands from the stack and returns their bitwise AND result.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$lhs,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rhs
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// Bitwise OR operation
def Dora_Or : Dora_Op<"or"> {
  let summary = "Bitwise OR operation on two 256-bit integers.";
  let description = [{
    This operation takes two `i256` operands from the stack and returns their bitwise OR result.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$lhs,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rhs
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// Bitwise XOR operation
def Dora_Xor : Dora_Op<"xor"> {
  let summary = "Bitwise XOR operation on two 256-bit integers.";
  let description = [{
    This operation takes two `i256` operands from the stack and returns their bitwise XOR result.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$lhs,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rhs
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$lhs `,` $rhs `:` type($lhs) `->` type($result)";
}

// Bitwise NOT operation
def Dora_Not : Dora_Op<"not"> {
  let summary = "Bitwise NOT operation on a 256-bit integer.";
  let description = [{
    This operation takes one `i256` operand from the stack and returns its bitwise NOT result.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$value
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$value `:` type($value) `->` type($result)";
}

// Retrieve single byte from word
def Dora_Byte : Dora_Op<"byte"> {
  let summary = "Extracts a byte from a 256-bit integer.";
  let description = [{
    This operation takes two `i256` operands from the stack: an index and a value. It returns the byte at the specified index from the 256-bit integer.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$index,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$value
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$index `,` $value `:` type($index) `->` type($result)";
}

// Left shift operation
def Dora_Shl : Dora_Op<"shl"> {
  let summary = "Left shifts a 256-bit integer by a specified number of bits.";
  let description = [{
    This operation takes two `i256` operands from the stack: the shift amount and the value. It shifts the value left by the specified number of bits.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$shift,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$value
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$shift `,` $value `:` type($shift) `->` type($result)";
}

// Logical right shift operation
def Dora_Shr : Dora_Op<"shr"> {
  let summary = "Logical right shifts a 256-bit integer by a specified number of bits.";
  let description = [{
    This operation takes two `i256` operands from the stack: the shift amount and the value. It shifts the value right logically (filling with zeros) by the specified number of bits.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$shift,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$value
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$shift `,` $value `:` type($shift) `->` type($result)";
}

// Arithmetic (signed) right shift operation
def Dora_Sar : Dora_Op<"sar"> {
  let summary = "Arithmetic right shifts a 256-bit integer by a specified number of bits.";
  let description = [{
    This operation takes two `i256` operands from the stack: the shift amount and the value. It shifts the value right arithmetically (preserving the sign) by the specified number of bits.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$shift,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$value
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$shift `,` $value `:` type($shift) `->` type($result)";
}

// Rotate left operation
def Dora_Rotl : Dora_Op<"rotl"> {
  let summary = "Rotates a 256-bit integer to the left by a specified number of bits.";
  let description = [{
    This operation takes two `i256` operands from the stack: the rotation amount and the value. It rotates the value left by the specified number of bits, wrapping the bits around to the right when they overflow.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rotate,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$value
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$rotate `,` $value `:` type($rotate) `->` type($result)";
}

// Rotate right operation
def Dora_Rotr : Dora_Op<"rotr"> {
  let summary = "Rotates a 256-bit integer to the right by a specified number of bits.";
  let description = [{
    This operation takes two `i256` operands from the stack: the rotation amount and the value. It rotates the value right by the specified number of bits, wrapping the bits around to the left when they overflow.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rotate,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$value
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);

  let assemblyFormat = "$rotate `,` $value `:` type($rotate) `->` type($result)";
}

// Count leading zeros operation
def Dora_Clz : Dora_Op<"clz"> {
  let summary = "Counts the leading zeros of a value.";
  let description = [{
    This operation takes a single operand of any integer type and returns the number of leading zero bits. 
    If the value is zero, the result will be equal to the bit width of the type.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>]>:$value
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>]>:$result);

  let assemblyFormat = "$value `:` type($value) `->` type($result)";
}

// Count trailing zeros operation
def Dora_Ctz : Dora_Op<"ctz"> {
  let summary = "Counts the trailing zeros of a value.";
  let description = [{
    This operation takes a single operand of any integer type and returns the number of trailing zero bits. 
    If the value is zero, the result will be equal to the bit width of the type.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>]>:$value
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>]>:$result);

  let assemblyFormat = "$value `:` type($value) `->` type($result)";
}

def Dora_Popcnt : Dora_Op<"popcnt"> {
  let summary = "Counts the number of 1-bits in the binary representation of a value.";
  let description = [{
    This operation takes a single operand of any integer type and returns the count of bits set to 1 in the operand.
    The result will be of the same bit width as the input type.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>]>:$value
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>]>:$result);

  let assemblyFormat = "$value `:` type($value) `->` type($result)";
}

// Select Operation
def Dora_SelectOp :Dora_Op<"select"> {
    let arguments = (ins
        AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$lhs,
        AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$rhs,
        AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$cond
    );

    let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$result);
}

// Keccak256 Operation
def Dora_Keccak256 : Dora_Op<"keccak256"> {
  let summary = "Computes the Keccak-256 hash of a memory region.";
  let description = [{
    This operation takes two `i256` operands from the stack, which specify the starting position and the length of the memory region to hash, and returns the 256-bit hash.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$start,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$length
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$hash);

  let assemblyFormat = "$start `,` $length `:` type($start) `->` type($hash)";
}

// Address Operation
def Dora_Address : Dora_Op<"address"> {
  let summary = "Gets the address of the executing contract.";
  let description = [{
    This operation returns the address of the contract currently executing.
  }];

  let arguments = (ins);
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$address);

  let assemblyFormat = "`->` type($address)";
}

// Balance Operation
def Dora_Balance : Dora_Op<"balance"> {
  let summary = "Gets the balance of an account.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the address of an account, and returns the balance of that account.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$account
  );

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$balance);

  let assemblyFormat = "$account `:` type($account) `->` type($balance)";
}

// Origin Operation
def Dora_Origin : Dora_Op<"origin"> {
  let summary = "Gets the address of the originator of the transaction.";
  let description = [{
    This operation returns the address of the account that sent the transaction.
  }];

  let arguments = (ins);
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$origin);

  let assemblyFormat = "`->` type($origin)";
}

// Caller Operation
def Dora_Caller : Dora_Op<"caller"> {
  let summary = "Gets the address of the caller of the current execution.";
  let description = [{
    This operation returns the address of the account that called the current contract.
  }];

  let arguments = (ins);
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$caller);

  let assemblyFormat = "`->` type($caller)";
}

// CallValue Operation
def Dora_CallValue : Dora_Op<"callvalue"> {
  let summary = "Gets the value (in Wei) sent with the call.";
  let description = [{
    This operation returns the amount of Wei sent with the call.
  }];

  let arguments = (ins);
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$value);

  let assemblyFormat = "`->` type($value)";
}

// CalldataLoad Operation
def Dora_CalldataLoad : Dora_Op<"calldataload"> {
  let summary = "Loads a word of the input data.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the byte offset in the input data, and returns the 256-bit word at that offset.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$offset
  );

  let results = (outs I<256>:$value);

  let assemblyFormat = "$offset `:` type($offset) `->` type($data)";
}

// CalldataSize Operation
def Dora_CalldataSize : Dora_Op<"calldatasize"> {
  let summary = "Gets the size of the input data.";
  let description = [{
    This operation returns the size of the input data for the current call.
  }];

  let arguments = (ins);
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$size);

  let assemblyFormat = "`->` type($size)";
}

// CalldataCopy Operation
def Dora_CalldataCopy : Dora_Op<"calldatacopy"> {
  let summary = "Copies a portion of the input data to memory.";
  let description = [{
    This operation takes three `i256` operands from the stack: the destination memory offset, the source offset in the input data, and the number of bytes to copy.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$destOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$dataOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$length
  );
  let results = (outs);

  let assemblyFormat = "$destOffset `,` $dataOffset `,` $length `:` type($destOffset)";
}

// CodeSize Operation
def Dora_CodeSize : Dora_Op<"codesize"> {
  let summary = "Gets the size of the contract's code.";
  let description = [{
    This operation returns the size of the code in bytes of the contract currently executing.
  }];

  let arguments = (ins);
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$size);

  let assemblyFormat = "`->` type($size)";
}

// CodeCopy Operation
def Dora_CodeCopy : Dora_Op<"codecopy"> {
  let summary = "Copies a portion of the contract's code to memory.";
  let description = [{
    This operation takes three `i256` operands from the stack: the destination memory offset, the source offset in the contract code, and the number of bytes to copy.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$destOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$codeOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$length
  );
  let results = (outs);

  let assemblyFormat = "$destOffset `,` $codeOffset `,` $length `:` type($destOffset)";
}

// GasPrice Operation
def Dora_GasPrice : Dora_Op<"gasprice"> {
  let summary = "Gets the gas price of the current transaction.";
  let description = [{
    This operation returns the gas price in Wei for the current transaction.
  }];

  let arguments = (ins);
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$price);

  let assemblyFormat = "`->` type($price)";
}

// ExtCodeSize Operation
def Dora_ExtCodeSize : Dora_Op<"extcodesize"> {
  let summary = "Gets the size of another account's code.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the address of an account, and returns the size of that account's code in bytes.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$account
  );
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$size);

  let assemblyFormat = "$account `:` type($account) `->` type($size)";
}

// ExtCodeCopy Operation
def Dora_ExtCodeCopy : Dora_Op<"extcodecopy"> {
  let summary = "Copies a portion of another account's code to memory.";
  let description = [{
    This operation takes four `i256` operands from the stack: the account address, the destination memory offset, the source offset in the account's code, and the number of bytes to copy.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$account,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$destOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$codeOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$length
  );
  let results = (outs);

  let assemblyFormat = "$account `,` $destOffset `,` $codeOffset `,` $length `:` type($account)";
}

// Stop Operation
def Dora_Stop : Dora_Op<"stop"> {
  let summary = "Halts execution.";
  let description = [{
    This operation halts the execution of the EVM program.
  }];

  let arguments = (ins);
  let results = (outs);

  let assemblyFormat = "";
}

// ReturndataSize Operation
def Dora_ReturndataSize : Dora_Op<"returndatasize"> {
  let summary = "Gets the size of the returned data from the last external call.";
  let description = [{
    This operation returns the size of the data returned by the last external call.
  }];

  let arguments = (ins);
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$size);

  let assemblyFormat = "`->` type($size)";
}

// ReturndataCopy Operation
def Dora_ReturndataCopy : Dora_Op<"returndatacopy"> {
  let summary = "Copies the returned data from the last external call to memory.";
  let description = [{
    This operation takes three `i256` operands from the stack: the destination memory offset, the source offset in the returned data, and the number of bytes to copy.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$destOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$dataOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$length
  );
  let results = (outs);

  let assemblyFormat = "$destOffset `,` $dataOffset `,` $length `:` type($destOffset)";
}

// ExtCodeHash Operation
def Dora_ExtCodeHash : Dora_Op<"extcodehash"> {
  let summary = "Gets the hash of another account's code.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the address of an account, and returns the hash of that account's code.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$account
  );
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$hash); 

  let assemblyFormat = "$account `:` type($account) `->` type($hash)";
}

// BlockHash Operation
def Dora_BlockHash : Dora_Op<"blockhash"> {
  let summary = "Gets the block hash of a recent block.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the block number, and returns the hash of the specified block. Only the most recent 256 blocks are accessible.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$blockNumber
  );
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$hash); 

  let assemblyFormat = "$blockNumber `:` type($blockNumber) `->` type($hash)";
}

// Coinbase Operation
def Dora_Coinbase : Dora_Op<"coinbase"> {
  let summary = "Gets the address of the current block miner.";
  let description = [{
    This operation returns the address of the account that is currently mining the block.
  }];

  let arguments = (ins);
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$minerAddress); 

  let assemblyFormat = "`->` type($minerAddress)";
}

// Timestamp Operation
def Dora_Timestamp : Dora_Op<"timestamp"> {
  let summary = "Gets the timestamp of the current block.";
  let description = [{
    This operation returns the Unix timestamp of when the current block was mined.
  }];

  let arguments = (ins);
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$timestamp); 

  let assemblyFormat = "`->` type($timestamp)";
}

// Number Operation
def Dora_Number : Dora_Op<"number"> {
  let summary = "Gets the block number of the current block.";
  let description = [{
    This operation returns the block number of the current block.
  }];

  let arguments = (ins);
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$blockNumber); 

  let assemblyFormat = "`->` type($blockNumber)";
}

// PrevRandao Operation
def Dora_PrevRandao : Dora_Op<"prevrandao"> {
  let summary = "Gets the previous block's RANDAO value.";
  let description = [{
    This operation returns the RANDAO value from the previous block.
  }];

  let arguments = (ins);
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$randao); 

  let assemblyFormat = "`->` type($randao)";
}

// GasLimit Operation
def Dora_GasLimit : Dora_Op<"gaslimit"> {
  let summary = "Gets the gas limit of the current block.";
  let description = [{
    This operation returns the gas limit of the block currently being mined.
  }];

  let arguments = (ins);
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$gasLimit); 

  let assemblyFormat = "`->` type($gasLimit)";
}

// ChainId Operation
def Dora_ChainId : Dora_Op<"chainid"> {
  let summary = "Gets the chain ID of the current network.";
  let description = [{
    This operation returns the chain ID of the network currently being used.
  }];

  let arguments = (ins);
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$chainId); 

  let assemblyFormat = "`->` type($chainId)";
}

// SelfBalance Operation
def Dora_SelfBalance : Dora_Op<"selfbalance"> {
  let summary = "Gets the balance of the executing contract.";
  let description = [{
    This operation returns the balance of the contract currently executing.
  }];

  let arguments = (ins);
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$balance); 

  let assemblyFormat = "`->` type($balance)";
}

// BaseFee Operation
def Dora_BaseFee : Dora_Op<"basefee"> {
  let summary = "Gets the base fee per gas for the current block.";
  let description = [{
    This operation returns the base fee per gas for the block currently being mined.
  }];

  let arguments = (ins);
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$baseFee); 

  let assemblyFormat = "`->` type($baseFee)";
}

// BlobHash Operation
def Dora_BlobHash : Dora_Op<"blobhash"> {
  let summary = "Gets the hash of the blob at the specified index.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the index of the blob, and returns the hash of that blob.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$index
  );
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$hash); 

  let assemblyFormat = "$index `:` type($index) `->` type($hash)";
}

// BlobBaseFee Operation
def Dora_BlobBaseFee : Dora_Op<"blobbasefee"> {
  let summary = "Gets the base fee per blob for the current block.";
  let description = [{
    This operation returns the base fee per blob for the block currently being mined.
  }];

  let arguments = (ins);
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$baseFee); 

  let assemblyFormat = "`->` type($baseFee)";
}

// MLoad Operation
def Dora_MLoad : Dora_Op<"mload"> {
  let summary = "Loads a 256-bit word from memory.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the memory offset, and returns the 256-bit word stored at that memory location.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$offset
  );
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$data); 

  let assemblyFormat = "$offset `:` type($offset) `->` type($data)";
}

// MStore Operation
def Dora_MStore : Dora_Op<"mstore"> {
  let summary = "Stores a 256-bit word in memory.";
  let description = [{
    This operation takes two `i256` operands from the stack: the memory offset and the 256-bit word to store at that memory location.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$offset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$data
  );
  let results = (outs); 

  let assemblyFormat = "$offset `,` $data `:` type($offset)";
}

// MStore8 Operation
def Dora_MStore8 : Dora_Op<"mstore8"> {
  let summary = "Stores an 8-bit byte in memory.";
  let description = [{
    This operation takes two `i256` operands from the stack: the memory offset and the 8-bit byte to store at that memory location.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$offset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$data
  );
  let results = (outs); 

  let assemblyFormat = "$offset `,` $data `:` type($offset)";
}

// SLoad Operation
def Dora_SLoad : Dora_Op<"sload"> {
  let summary = "Loads a 256-bit word from storage.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the storage key, and returns the 256-bit word stored at that key.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$offset
  );
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$data); 

  let assemblyFormat = "$key `:` type($key) `->` type($value)";
}

// SStore Operation
def Dora_SStore : Dora_Op<"sstore"> {
  let summary = "Stores a 256-bit word in storage.";
  let description = [{
    This operation takes two `i256` operands from the stack: the storage key and the 256-bit word to store at that key.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$key,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$value
  );
  let results = (outs); 

  let assemblyFormat = "$key `,` $value `:` type($key)";
}

// MSize Operation
def Dora_MSize : Dora_Op<"msize"> {
  let summary = "Gets the size of the memory in bytes.";
  let description = [{
    This operation returns the current size of the memory in bytes.
  }];

  let arguments = (ins);
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$size); 

  let assemblyFormat = "`->` type($size)";
}

// Gas Operation
def Dora_Gas : Dora_Op<"gas"> {
  let summary = "Gets the remaining gas.";
  let description = [{
    This operation returns the amount of gas remaining for the execution.
  }];

  let arguments = (ins);
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$gas); 

  let assemblyFormat = "`->` type($gas)";
}

// TLoad Operation
def Dora_TLoad : Dora_Op<"tload"> {
  let summary = "Loads a 256-bit word from the transient storage.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the transient storage key, and returns the 256-bit word stored at that key.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$key
  );
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$value); 

  let assemblyFormat = "$key `:` type($key) `->` type($value)";
}

// TStore Operation
def Dora_TStore : Dora_Op<"tstore"> {
  let summary = "Stores a 256-bit word in the transient storage.";
  let description = [{
    This operation takes two `i256` operands from the stack: the transient storage key and the 256-bit word to store at that key.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$key,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$value
  );
  let results = (outs); 

  let assemblyFormat = "$key `,` $value `:` type($key)";
}

// MCopy Operation
def Dora_MCopy : Dora_Op<"mcopy"> {
  let summary = "Copies a portion of the memory to another location in memory.";
  let description = [{
    This operation takes three `i256` operands from the stack: the destination memory offset, the source memory offset, and the number of bytes to copy.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$destOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$srcOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$length
  );
  let results = (outs); 

  let assemblyFormat = "$destOffset `,` $srcOffset `,` $length `:` type($destOffset)";
}

// Log0 Operation
def Dora_Log0 : Dora_Op<"log0"> {
  let summary = "Logs one topic with no data.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the memory offset, and logs an event with no additional topics.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$offset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$length
  );
  let results = (outs); 

  let assemblyFormat = "$offset `,` $length `:` type($offset)";
}

// Log1 Operation
def Dora_Log1 : Dora_Op<"log1"> {
  let summary = "Logs one topic with data.";
  let description = [{
    This operation takes two `i256` operands from the stack: the memory offset and the topic, and logs an event with one additional topic.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$offset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$length,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$topic
  );
  let results = (outs);

  let assemblyFormat = "$offset `,` $length `,` $topic `:` type($offset)";
}

// Log2 Operation
def Dora_Log2 : Dora_Op<"log2"> {
  let summary = "Logs two topics with data.";
  let description = [{
    This operation takes three `i256` operands from the stack: the memory offset, the length, and two topics, and logs an event with two additional topics.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$offset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$length,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$topic1,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$topic2
  );
  let results = (outs);

  let assemblyFormat = "$offset `,` $length `,` $topic1 `,` $topic2 `:` type($offset)";
}

// Log3 Operation
def Dora_Log3 : Dora_Op<"log3"> {
  let summary = "Logs three topics with data.";
  let description = [{
    This operation takes four `i256` operands from the stack: the memory offset, the length, and three topics, and logs an event with three additional topics.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$offset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$length,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$topic1,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$topic2,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$topic3
  );
  let results = (outs);

  let assemblyFormat = "$offset `,` $length `,` $topic1 `,` $topic2 `,` $topic3 `:` type($offset)";
}

// Log4 Operation
def Dora_Log4 : Dora_Op<"log4"> {
  let summary = "Logs four topics with data.";
  let description = [{
    This operation takes five `i256` operands from the stack: the memory offset, the length, and four topics, and logs an event with four additional topics.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$offset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$length,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$topic1,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$topic2,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$topic3,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$topic4
  );
  let results = (outs);

  let assemblyFormat = "$offset `,` $length `,` $topic1 `,` $topic2 `,` $topic3 `,` $topic4 `:` type($offset)";
}

// DataLoad Operation
def Dora_DataLoad : Dora_Op<"dataload"> {
  let summary = "Load a word of the data section.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the byte offset in the data section of EOF container, and returns the 256-bit word at that offset.
  }];

  let arguments = (ins AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$offset);

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$data);

  let assemblyFormat = "$offset `:` type($offset) `->` type($data)";
}

// DataLoadN Operation
def Dora_DataLoadN : Dora_Op<"dataloadn"> {
  let summary = "Load a word of the data section.";
  let description = [{
    This operation takes one `16` argument, which is the byte offset in the data section of EOF container, and returns the 256-bit word at that offset.
  }];

  let arguments = (ins AnyTypeOf<[I16, F16]>:$offset);

  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$data);

  let assemblyFormat = "$offset `:` type($offset) `->` type($data)";
}

// DataSize Operation
def Dora_DataSize : Dora_Op<"datasize"> {
  let summary = "Gets the size of the data section.";
  let description = [{
    This operation returns the size of the data section for the current call.
  }];

  let arguments = (ins);
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$size);

  let assemblyFormat = "`->` type($size)";
}

// DataCopy Operation
def Dora_DataCopy : Dora_Op<"datacopy"> {
  let summary = "Copies a portion of the data section to memory.";
  let description = [{
    This operation takes three `i256` operands from the stack: the destination memory offset, the source offset in the data section, and the number of bytes to copy.
  }];

  let arguments = (ins
      AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$memOffset,
      AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$offset,
      AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$size
    );
  let results = (outs);

  let assemblyFormat = "$memOffset `,` $offset `,` $size `:` type($memOffset)";
}

// CallF Operation
def Dora_CallF : Dora_Op<"callf"> {
  let summary = "Calls a function.";
  let description = [{
    This operation takes one `i16` argument, which is the metadata index in the type section, and call into a function or subroutine, then push an execution state onto the return stack.
  }];

  let arguments = (ins AnyTypeOf<[I16, F16]>:$target_section_index);
  let results = (outs);

  let assemblyFormat = "$target_section_index `:` type($target_section_index)";
}

// RetF Operation
def Dora_RetF : Dora_Op<"retf"> {
  let summary = "Returns data from the function execution.";
  let description = [{
    This operations sets the Context from CALLF, a function or subroutine, to corresponding values (current section index, program counter).
  }];

  let arguments = (ins);
  let results = (outs);

  let assemblyFormat = "";
}

// EofCreate Operation
def Dora_EofCreate : Dora_Op<"eofcreate"> {
  let summary = "Creates a new contract with a deterministic address.";
  let description = [{
    This operation takes one `i8` argument, which is the index of EOF subcontainer in the containers, and four `i256` operands from the stack: the amount of Ether to send, a salt value, the memory offset of the initialization code and the length of the initialization code, and It returns the address of the created contract, which is determined using the salt and the caller's address.
  }];

  let arguments = (ins
    I8:$initContainerIndex,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$value,
    AnyTypeOf<[I<256>, F<256>]>:$salt,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$inputOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$inputSize
  );
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$address);

  let assemblyFormat = "$initContainerIndex `,` $value `,` $salt `,` $inputOffset `,` $inputSize `:` type($initContainerIndex) `->` type($address)";
}

// ReturnContract Operation
def Dora_ReturnContract : Dora_Op<"returncontract"> {
  let summary = "Appends auxiliary data from memory stack to deployed contract";
  let description = [{
    This operation takes one `i8` argument, which is the index of EOF containers and two `i256` operand from the stack:  the memory offset of the auxiliary data and the length of the auxiliary data to copy, and It appended to deployed container's data.
  }];

  let arguments = (ins
    I8:$deployContainerIndex,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$auxDataOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$auxDataSize
  );
  let results = (outs);

  let assemblyFormat = "$deployContainerIndex `,` $auxDataOffset `,` $auxDataSize `:` type($deployContainerIndex)";
}

// Create Operation
def Dora_Create : Dora_Op<"create"> {
  let summary = "Creates a new contract.";
  let description = [{
    This operation takes three `i256` operands from the stack: the amount of Ether to send, the memory offset of the initialization code, and the length of the initialization code, and returns the address of the created contract.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$value,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$offset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$length
  );
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$address);

  let assemblyFormat = "$value `,` $offset `,` $length `:` type($value) `->` type($address)";
}

// Call Operation
def Dora_Call : Dora_Op<"call"> {
  let summary = "Calls a contract.";
  let description = [{
    This operation takes seven `i256` operands from the stack: the gas, the address, the amount of Ether to send, the memory offset of the input data, the length of the input data, the memory offset of the output data, and the length of the output data, and returns 1 if the call succeeds and 0 if it fails.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$gas,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$address,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$value,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$inputOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$inputLength,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$outputOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$outputLength
  );
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$success);

  let assemblyFormat = "$gas `,` $address `,` $value `,` $inputOffset `,` $inputLength `,` $outputOffset `,` $outputLength `:` type($gas) `->` type($success)";
}

// Callcode Operation
def Dora_Callcode : Dora_Op<"callcode"> {
  let summary = "Calls a contract's code without modifying its storage.";
  let description = [{
    This operation is similar to `call`, but it uses the caller's storage rather than the storage of the called contract.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$gas,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$address,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$value,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$inputOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$inputLength,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$outputOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$outputLength
  );
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$success);

  let assemblyFormat = "$gas `,` $address `,` $value `,` $inputOffset `,` $inputLength `,` $outputOffset `,` $outputLength `:` type($gas) `->` type($success)";
}

// Return Operation
def Dora_Return : Dora_Op<"return"> {
  let summary = "Returns data from the contract execution.";
  let description = [{
    This operation takes two `i256` operands from the stack: the memory offset of the return data and the length of the return data, and returns from the execution with the specified data.
  }];

  let arguments = (ins I<256>:$offset, I<256>:$length);
  let results = (outs);

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$offset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$length
  );
  let results = (outs);

  let assemblyFormat = "$offset `,` $length `:` type($offset)";
}

// Delegatecall Operation
def Dora_Delegatecall : Dora_Op<"delegatecall"> {
  let summary = "Calls a contract while keeping the caller's context.";
  let description = [{
    This operation is similar to `callcode`, but it also keeps the caller's sender and value.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$gas,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$address,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$inputOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$inputLength,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$outputOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$outputLength
  );
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$success);

  let assemblyFormat = "$gas `,` $address `,` $inputOffset `,` $inputLength `,` $outputOffset `,` $outputLength `:` type($gas) `->` type($success)";
}

// Create2 Operation
def Dora_Create2 : Dora_Op<"create2"> {
  let summary = "Creates a new contract with a deterministic address.";
  let description = [{
    This operation takes four `i256` operands from the stack: the amount of Ether to send, the memory offset of the initialization code, the length of the initialization code, and a salt value. It returns the address of the created contract, which is determined using the salt and the caller's address.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$value,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$offset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$length,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$salt
  );
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$address);

  let assemblyFormat = "$value `,` $offset `,` $length `,` $salt `:` type($value) `->` type($address)";
}

// ReturndataLoad Operation
def Dora_ReturndataLoad : Dora_Op<"returndataload"> {
  let summary = "Loads a word from the returned data from the last external call.";
  let description = [{
    This operation takes one `i256` operand from the stack: the offset in the returned data, and pushes the loaded word onto the stack.
  }];

  let arguments = (ins AnyTypeOf<[I16, F16]>:$offset);
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$data);

  let assemblyFormat = "$offset `:` type($offset) `->` type($data)";
}

// ExtCall Operation
def Dora_ExtCall : Dora_Op<"extcall"> {
  let summary = "Similar to Call.";
  let description = [{
    This operation takes four `i256` operands from the stack: the target address, the memory offset of the input data, the length of the input data to copy, the amount of Ether to send, and It returns 0 if the call succeeds, 1 for revert if sub context reverted and 2 if it fails.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$targetAddress,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$inputOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$inputSize,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$value
  );
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$status);

  let assemblyFormat = "$targetAddress `,` $inputOffset `,` $inputSize `,` $value `:` type($targetAddress) `->` type($status)";
}

// ExtDelegatecall Operation
def Dora_ExtDelegatecall : Dora_Op<"extdelegatecall"> {
  let summary = "Similar to Delegatecall.";
  let description = [{
    This operation is similar to `extcall`, but it also keeps the caller's sender and value.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$targetAddress,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$inputOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$inputSize
  );
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$status);

  let assemblyFormat = "$targetAddress `,` $inputOffset `,` $inputSize `:` type($targetAddress) `->` type($status)";
}

// Staticcall Operation
def Dora_Staticcall : Dora_Op<"staticcall"> {
  let summary = "Calls a contract in a static context.";
  let description = [{
    This operation is similar to `call`, but it does not allow state modifications.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$gas,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$address,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$inputOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$inputLength,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$outputOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$outputLength
  );
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$success);

  let assemblyFormat = "$gas `,` $address `,` $inputOffset `,` $inputLength `,` $outputOffset `,` $outputLength `:` type($gas) `->` type($success)";
}

// ExtStaticcall Operation
def Dora_ExtStaticcall : Dora_Op<"extstaticcall"> {
  let summary = "Similar to Staticcall.";
  let description = [{
    This operation is similar to `extcall`, but it does not allow state modifications.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$targetAddress,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$inputOffset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$inputSize
  );
  let results = (outs AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$status);

  let assemblyFormat = "$targetAddress `,` $inputOffset `,` $inputSize `:` type($targetAddress) `->` type($status)";
}

// Revert Operation
def Dora_Revert : Dora_Op<"revert"> {
  let summary = "Reverts the execution and returns data.";
  let description = [{
    This operation takes two `i256` operands from the stack: the memory offset of the return data and the length of the return data, and reverts the execution, returning the specified data.
  }];

  let arguments = (ins
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$offset,
    AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$length
  );
  let results = (outs);

  let assemblyFormat = "$offset `,` $length `:` type($offset)";
}

// Invalid Operation
def Dora_Invalid : Dora_Op<"invalid"> {
  let summary = "Halts execution with an invalid opcode.";
  let description = [{
    This operation halts the execution with an invalid opcode, causing an exception and reverting any state changes made during the transaction.
  }];

  let arguments = (ins);
  let results = (outs);

  let assemblyFormat = "";
}

// Selfdestruct Operation
def Dora_Selfdestruct : Dora_Op<"selfdestruct"> {
  let summary = "Terminates the contract and sends its balance to a specified address.";
  let description = [{
    This operation takes one `i256` operand from the stack, which is the address to send the remaining balance to, and terminates the contract, removing it from the blockchain.
  }];

  let arguments = (ins AnyTypeOf<[I32, I64, I<128>, I<256>, F32, F64, F128, F<256>]>:$recipient);
  let results = (outs);

  let assemblyFormat = "$recipient `:` type($recipient)";
}