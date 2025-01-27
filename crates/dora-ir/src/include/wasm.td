// This include is necessary to use `Dialect`
include "mlir/IR/OpBase.td"
include "mlir/IR/AttrTypeBase.td"

def WASM_Dialect : Dialect {
  let name = "wasm";
}

class WASM_Op<string mnemonic, list<Trait> traits = []> :
    Op<WASM_Dialect, mnemonic, traits>;

class WASM_Type<string name, string typeMnemonic, list<Trait> traits = []>
    : TypeDef<WASM_Dialect, name, traits> {
  let mnemonic = typeMnemonic;
}

def Wasm_FuncType : WASM_Type<"WASMFunc", "wasm.func"> {
  let summary = "Integer type with arbitrary precision up to a fixed limit";
  let description = [{
    Integer types have a designated bit width.
  }];
  let parameters = (ins "unsigned":$width);
  let assemblyFormat = "`<` $width `>`";
  let genVerifyDecl = 1;
}

// ~~~ Control instructions

// Unreachable Operation
def Wasm_UnreachableOp : WASM_Op<"unreachable"> {
    let summary = "unreachable operation";
    let description = [{
        Signals that an unreachable code path has been reached.
    }];
    let arguments = (ins);
    let results = (outs);
}

// No Operation (NOP)
def Wasm_NopOp : WASM_Op<"nop"> {
    let summary = "no operation";
    let description = [{
        Does nothing. Can be used for padding or alignment.
    }];
    let arguments = (ins);
    let results = (outs);
}

// Throw Code
def Wasm_ThrowOp : WASM_Op<"throw"> {
    let summary = "no operation";
    let description = [{
        Does nothing. Can be used for padding or alignment.
    }];
    let arguments = (ins I32:$code);
    let results = (outs);
}

// ~~~ Variable Instructions
// Get Local Operation
def Wasm_LocalGetOp : WASM_Op<"local_get"> {
    let summary = "get local variable";
    let description = [{
        Reads the value of a local variable.
    }];
    let arguments = (ins I32:$localIndex);
    let results = (outs I32:$value);
}

// Set Local Operation
def Wasm_LocalSetOp : WASM_Op<"local_set"> {
    let summary = "set local variable";
    let description = [{
        Sets the value of a local variable.
    }];
    let arguments = (ins I32:$localIndex, I32:$value);
    let results = (outs);
}

// Tes Local Operation
def Wasm_LocalTeeOp : WASM_Op<"local_tee"> {
    let summary = "set local variable";
    let description = [{
        Sets the value of a local variable.
    }];
    let arguments = (ins I32:$localIndex, I32:$value);
    let results = (outs);
}

// Get Global Operation
def Wasm_GlobalGetOp : WASM_Op<"global_get"> {
    let summary = "get global variable";
    let description = [{
        Reads the value of a global variable.
    }];
    let arguments = (ins I32:$globalIndex);
    let results = (outs I32:$value);
}

// Set Global Operation
def Wasm_GlobalSetOp : WASM_Op<"global_set"> {
    let summary = "set global variable";
    let description = [{
        Sets the value of a global variable.
    }];
    let arguments = (ins I32:$globalIndex, I32:$value);
    let results = (outs);
}

// ~~~ Memory operations
// Memory Load Operation
def Wasm_LoadOp : WASM_Op<"load"> {
    let summary = "load a value from memory";
    let description = [{
        This operation loads a value from memory at a specified address.
    }];

    let arguments = (ins AnyTypeOf<[I32, I64, F32, F64]>:$address);

    let results = (outs AnyTypeOf<[I32, I64, F32, F64]>:$value);

    let assemblyFormat = [{
        $address `:` type($address) `->` type($value)
    }];
}

// Memory Store Operation
def Wasm_StoreOp : WASM_Op<"store"> {
    let summary = "store a value to memory";
    let description = [{
        This operation stores a value in memory at a specified address.
    }];

    let arguments = (ins AnyTypeOf<[I32, I64, F32, F64]>:$address,
                    AnyTypeOf<[I32, I64, F32, F64]>:$value);

    let results = (outs);

    let assemblyFormat = [{
        $value `,` $address `:` type($address)
    }];
}

def Wasm_DataDropOp : WASM_Op<"data_drop"> {
    let arguments = (ins I32:$segment);
    let results = (outs);
}

def Wasm_MemInitOp : WASM_Op<"mem_init"> {
    let arguments = (ins I32:$mem, I32:$segment, I32:$dest, I32:$src, I32:$len);
    let results = (outs);
}

def Wasm_MemCopyOp : WASM_Op<"mem_copy"> {
    let arguments = (ins I32:$srcIndex, I32:$dest, I32:$srcPos, I32:$len);
    let results = (outs);
}

def Wasm_ImportedMemCopyOp : WASM_Op<"imported_mem_copy"> {
    let arguments = (ins I32:$srcIndex, I32:$dest, I32:$srcPos, I32:$len);
    let results = (outs);
}

def Wasm_MemFillOp : WASM_Op<"mem_fill"> {
    let arguments = (ins I32:$memIndex, I32:$dst, I32:$val, I32:$len);
    let results = (outs);
}

def Wasm_ImportedMemFillOp : WASM_Op<"imported_mem_fill"> {
    let arguments = (ins I32:$memIndex, I32:$dst, I32:$val, I32:$len);
    let results = (outs);
}

// Memory Size Operation
def Wasm_MemSizeOp : WASM_Op<"mem_size"> {
    let arguments = (ins I32:$mem);
    let results = (outs I32:$value);
}

// Memory Grow Operation
def Wasm_MemGrowOp : WASM_Op<"mem_grow"> {
    let arguments = (ins I32:$delta, I32:$mem);
    let results = (outs I32:$value);
}

def Wasm_ElemDropOp : WASM_Op<"elem_drop"> {
    let arguments = (ins I32:$index);
    let results = (outs);
}

def Wasm_TableCopyOp : WASM_Op<"table_copy"> {
    let arguments = (ins I32:$dstTable, I32:$srcTable, I32:$dst, I32:$src, I32:$len);
    let results = (outs);
}

def Wasm_TableInitOp : WASM_Op<"table_init"> {
    let arguments = (ins I32:$table, I32:$segment, I32:$dst, I32:$src, I32:$len);
    let results = (outs);
}

def Wasm_TableGetOp : WASM_Op<"table_get"> {
    let arguments = (ins I32:$table, I32:$elem);
    let results = (outs I32:$value);
}

def Wasm_ImportedTableGetOp : WASM_Op<"imported_table_get"> {
    let arguments = (ins I32:$table, I32:$elem);
    let results = (outs I32:$value);
}

def Wasm_TableSetOp : WASM_Op<"table_set"> {
    let arguments = (ins I32:$table, I32:$elem, I32:$value);
    let results = (outs);
}

def Wasm_ImportedTableSetOp : WASM_Op<"imported_table_set"> {
    let arguments = (ins I32:$table, I32:$elem, I32:$value);
    let results = (outs);
}

def Wasm_TableFillOp : WASM_Op<"table_fill"> {
    let arguments = (ins I32:$table, I32:$start, I32:$elem, I32:$len);
    let results = (outs);
}

def Wasm_TableGrowOp : WASM_Op<"table_grow"> {
    let arguments = (ins I32:$elem, I32:$delta, I32:$tableIndex);
    let results = (outs I32:$value);
}

def Wasm_ImportedTableGrowOp : WASM_Op<"imported_table_grow"> {
    let arguments = (ins I32:$elem, I32:$delta, I32:$tableIndex);
    let results = (outs I32:$value);
}

def Wasm_TableSizeOp : WASM_Op<"table_size"> {
    let arguments = (ins I32:$table);
    let results = (outs I32:$value);
}

def Wasm_ImportedTableSizeOp : WASM_Op<"imported_table_size"> {
    let arguments = (ins I32:$table);
    let results = (outs);
}

def Wasm_MemAtomicWaitOp : WASM_Op<"mem_atomic_wait"> {
    let arguments = (ins I32:$dst, I32:$count, I32:$timeout);
    let results = (outs);
}

def Wasm_MemAtomicNotifyOp : WASM_Op<"mem_atomic_notify"> {
    let arguments = (ins I32:$dst, I32:$count);
    let results = (outs);
}

// Splat Operation
def Wasm_SplatOp : WASM_Op<"splat"> {
    let summary = "add two operands of the same type";
    let description = [{
        This operation takes two operands of the same type and returns their sum.
    }];

    let arguments = (ins AnyTypeOf<[I32, I64]>:$input);

    let results = (outs AnyTypeOf<[I32, I64]>:$result);

    let assemblyFormat = [{
        `splat` $input `:` type($result)
    }];
}

// Clz Operation
def Wasm_ClzOp : WASM_Op<"clz"> {
    let summary = "add two operands of the same type";
    let description = [{
        This operation takes two operands of the same type and returns their sum.
    }];

    let arguments = (ins AnyTypeOf<[I32, I64]>:$input);

    let results = (outs AnyTypeOf<[I32, I64]>:$result);

    let assemblyFormat = [{
        `clz` $input `:` type($result)
    }];
}

// Ctz Operation
def Wasm_CtzOp : WASM_Op<"ctz"> {
    let summary = "add two operands of the same type";
    let description = [{
        This operation takes two operands of the same type and returns their sum.
    }];

    let arguments = (ins AnyTypeOf<[I32, I64]>:$input);

    let results = (outs AnyTypeOf<[I32, I64]>:$result);

    let assemblyFormat = [{
        `clz` $input `:` type($result)
    }];
}

// Popcnt Operation
def Wasm_PopcntOp : WASM_Op<"popcnt"> {
    let summary = "add two operands of the same type";
    let description = [{
        This operation takes two operands of the same type and returns their sum.
    }];

    let arguments = (ins AnyTypeOf<[I32, I64]>:$input);

    let results = (outs AnyTypeOf<[I32, I64]>:$result);

    let assemblyFormat = [{
        `popcnt` $input `:` type($result)
    }];
}

// ~~~ Numeric Instructions
// Add Operation
def Wasm_AddOp : WASM_Op<"add"> {
    let summary = "add two operands of the same type";
    let description = [{
        This operation takes two operands of the same type and returns their sum.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64, F32, F64]>:$lhs,
        AnyTypeOf<[I32, I64, F32, F64]>:$rhs
    );

    let results = (outs AnyTypeOf<[I32, I64, F32, F64]>:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// Sub Operation
def Wasm_SubOp : WASM_Op<"sub"> {
    let summary = "subtract two operands of the same type";
    let description = [{
        This operation takes two operands of the same type and returns their difference.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64, F32, F64]>:$lhs,
        AnyTypeOf<[I32, I64, F32, F64]>:$rhs
    );

    let results = (outs AnyTypeOf<[I32, I64, F32, F64]>:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// Mul Operation
def Wasm_MulOp : WASM_Op<"mul"> {
    let summary = "multiply two operands of the same type";
    let description = [{
        This operation takes two operands of the same type and returns their product.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64, F32, F64]>:$lhs,
        AnyTypeOf<[I32, I64, F32, F64]>:$rhs
    );

    let results = (outs AnyTypeOf<[I32, I64, F32, F64]>:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// Div Operation
def Wasm_DivOp : WASM_Op<"div"> {
    let summary = "multiply two operands of the same type";
    let description = [{
        This operation takes two operands of the same type and returns their product.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64, F32, F64]>:$lhs,
        AnyTypeOf<[I32, I64, F32, F64]>:$rhs
    );

    let results = (outs AnyTypeOf<[I32, I64, F32, F64]>:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// DivS Operation
def Wasm_DivSOp : WASM_Op<"divs"> {
    let summary = "multiply two operands of the same type";
    let description = [{
        This operation takes two operands of the same type and returns their product.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64]>:$lhs,
        AnyTypeOf<[I32, I64]>:$rhs
    );

    let results = (outs AnyTypeOf<[I32, I64]>:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// DivU Operation
def Wasm_DivUOp : WASM_Op<"divu"> {
    let summary = "multiply two operands of the same type";
    let description = [{
        This operation takes two operands of the same type and returns their product.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64]>:$lhs,
        AnyTypeOf<[I32, I64]>:$rhs
    );

    let results = (outs AnyTypeOf<[I32, I64]>:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// RemU Operation
def Wasm_RemUOp : WASM_Op<"remu"> {
    let summary = "multiply two operands of the same type";
    let description = [{
        This operation takes two operands of the same type and returns their product.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64]>:$lhs,
        AnyTypeOf<[I32, I64]>:$rhs
    );

    let results = (outs AnyTypeOf<[I32, I64]>:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// RemS Operation
def Wasm_RemSOp : WASM_Op<"rems"> {
    let summary = "multiply two operands of the same type";
    let description = [{
        This operation takes two operands of the same type and returns their product.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64]>:$lhs,
        AnyTypeOf<[I32, I64]>:$rhs
    );

    let results = (outs AnyTypeOf<[I32, I64]>:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// And Operation
def Wasm_AndOp : WASM_Op<"and"> {
    let summary = "bitwise AND two operands of the same type";
    let description = [{
        This operation takes two operands of the same type and returns their bitwise AND.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64]>:$lhs,
        AnyTypeOf<[I32, I64]>:$rhs
    );

    let results = (outs AnyTypeOf<[I32, I64]>:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// Or Operation
def Wasm_OrOp : WASM_Op<"or"> {
    let summary = "bitwise OR two operands of the same type";
    let description = [{
        This operation takes two operands of the same type and returns their bitwise OR.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64]>:$lhs,
        AnyTypeOf<[I32, I64]>:$rhs
    );

    let results = (outs AnyTypeOf<[I32, I64]>:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// Xor Operation
def Wasm_XOrOp : WASM_Op<"xor"> {
    let summary = "bitwise OR two operands of the same type";
    let description = [{
        This operation takes two operands of the same type and returns their bitwise OR.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64]>:$lhs,
        AnyTypeOf<[I32, I64]>:$rhs
    );

    let results = (outs AnyTypeOf<[I32, I64]>:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// Shl Operation
def Wasm_ShlOp : WASM_Op<"shl"> {
    let summary = "bitwise OR two operands of the same type";
    let description = [{
        This operation takes two operands of the same type and returns their bitwise OR.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64]>:$lhs,
        AnyTypeOf<[I32, I64]>:$rhs
    );

    let results = (outs AnyTypeOf<[I32, I64]>:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// ShrS Operation
def Wasm_ShrSOp : WASM_Op<"shrs"> {
    let summary = "bitwise OR two operands of the same type";
    let description = [{
        This operation takes two operands of the same type and returns their bitwise OR.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64]>:$lhs,
        AnyTypeOf<[I32, I64]>:$rhs
    );

    let results = (outs AnyTypeOf<[I32, I64]>:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// ShrU Operation
def Wasm_ShrUOp : WASM_Op<"shru"> {
    let summary = "bitwise OR two operands of the same type";
    let description = [{
        This operation takes two operands of the same type and returns their bitwise OR.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64]>:$lhs,
        AnyTypeOf<[I32, I64]>:$rhs
    );

    let results = (outs AnyTypeOf<[I32, I64]>:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// Eqz Operation
def Wasm_EqzOp : WASM_Op<"eqz"> {
    let summary = "Check for equality to zero";
    let description = [{
        This operation checks if an operand of either i32 or i64 type is equal to zero.
    }];

    let arguments = (ins AnyTypeOf<[I32, I64]>:$val);

    let results = (outs I32:$result);

    let assemblyFormat = [{
        $value `:` type($value) `->` type($result)
    }];
}

// Eq Operation
def Wasm_EqOp : WASM_Op<"eq"> {
    let summary = "Equality comparison between two operands of the same type";
    let description = [{
        This operation compares two operands of the same type for equality and returns the result.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64, F32, F64]>:$lhs,
        AnyTypeOf<[I32, I64, F32, F64]>:$rhs
    );

    let results = (outs I32:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// Ne Operation
def Wasm_NeOp : WASM_Op<"ne"> {
    let summary = "Not equality comparison between two operands of the same type";
    let description = [{
        This operation compares two operands of the same type for not equality and returns the result.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64, F32, F64]>:$lhs,
        AnyTypeOf<[I32, I64, F32, F64]>:$rhs
    );

    let results = (outs AnyTypeOf<[I32, I64, F32, F64]>:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// Lts Operation
def Wasm_LtsOp : WASM_Op<"lts"> {
    let summary = "Not equality comparison between two operands of the same type";
    let description = [{
        This operation compares two operands of the same type for not equality and returns the result.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64]>:$lhs,
        AnyTypeOf<[I32, I64]>:$rhs
    );

    let results = (outs I32:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// Ltu Operation
def Wasm_LtuOp : WASM_Op<"ltu"> {
    let summary = "Not equality comparison between two operands of the same type";
    let description = [{
        This operation compares two operands of the same type for not equality and returns the result.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64]>:$lhs,
        AnyTypeOf<[I32, I64]>:$rhs
    );

    let results = (outs AnyTypeOf<[I32, I64]>:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// Gts Operation
def Wasm_GtsOp : WASM_Op<"gts"> {
    let summary = "Not equality comparison between two operands of the same type";
    let description = [{
        This operation compares two operands of the same type for not equality and returns the result.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64]>:$lhs,
        AnyTypeOf<[I32, I64]>:$rhs
    );

    let results = (outs I32:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// Gtu Operation
def Wasm_GtuOp : WASM_Op<"gtu"> {
    let summary = "Not equality comparison between two operands of the same type";
    let description = [{
        This operation compares two operands of the same type for not equality and returns the result.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64]>:$lhs,
        AnyTypeOf<[I32, I64]>:$rhs
    );

    let results = (outs I32:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// Les Operation
def Wasm_LesOp : WASM_Op<"les"> {
    let summary = "Not equality comparison between two operands of the same type";
    let description = [{
        This operation compares two operands of the same type for not equality and returns the result.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64]>:$lhs,
        AnyTypeOf<[I32, I64]>:$rhs
    );

    let results = (outs I32:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// Leu Operation
def Wasm_LeuOp : WASM_Op<"leu"> {
    let summary = "Not equality comparison between two operands of the same type";
    let description = [{
        This operation compares two operands of the same type for not equality and returns the result.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64]>:$lhs,
        AnyTypeOf<[I32, I64]>:$rhs
    );

    let results = (outs I32:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// Ges Operation
def Wasm_GesOp : WASM_Op<"ges"> {
    let summary = "Not equality comparison between two operands of the same type";
    let description = [{
        This operation compares two operands of the same type for not equality and returns the result.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64]>:$lhs,
        AnyTypeOf<[I32, I64]>:$rhs
    );

    let results = (outs I32:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// Geu Operation
def Wasm_GeuOp : WASM_Op<"geu"> {
    let summary = "Not equality comparison between two operands of the same type";
    let description = [{
        This operation compares two operands of the same type for not equality and returns the result.
    }];

    let arguments = (ins
        AnyTypeOf<[I32, I64]>:$lhs,
        AnyTypeOf<[I32, I64]>:$rhs
    );

    let results = (outs I32:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// Flt Operation
def Wasm_FltOp : WASM_Op<"flt"> {
    let summary = "Not equality comparison between two operands of the same type";
    let description = [{
        This operation compares two operands of the same type for not equality and returns the result.
    }];

    let arguments = (ins
        AnyTypeOf<[F32, F64]>:$lhs,
        AnyTypeOf<[F32, F64]>:$rhs
    );

    let results = (outs I32:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// Fgt Operation
def Wasm_FgtOp : WASM_Op<"fgt"> {
    let summary = "Not equality comparison between two operands of the same type";
    let description = [{
        This operation compares two operands of the same type for not equality and returns the result.
    }];

    let arguments = (ins
        AnyTypeOf<[F32, F64]>:$lhs,
        AnyTypeOf<[F32, F64]>:$rhs
    );

    let results = (outs I32:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// Fle Operation
def Wasm_FleOp : WASM_Op<"fle"> {
    let summary = "Not equality comparison between two operands of the same type";
    let description = [{
        This operation compares two operands of the same type for not equality and returns the result.
    }];

    let arguments = (ins
        AnyTypeOf<[F32, F64]>:$lhs,
        AnyTypeOf<[F32, F64]>:$rhs
    );

    let results = (outs I32:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

// Fge Operation
def Wasm_FgeOp : WASM_Op<"fge"> {
    let summary = "Not equality comparison between two operands of the same type";
    let description = [{
        This operation compares two operands of the same type for not equality and returns the result.
    }];

    let arguments = (ins
        AnyTypeOf<[F32, F64]>:$lhs,
        AnyTypeOf<[F32, F64]>:$rhs
    );

    let results = (outs I32:$result);

    let assemblyFormat = [{
        $lhs `,` $rhs `:` type($lhs) `->` type($result)
    }];
}

def Wasm_RotlOp : WASM_Op<"rotl"> {
    let arguments = (ins
        AnyTypeOf<[I32, I64, F32, F64]>:$lhs,
        AnyTypeOf<[I32, I64, F32, F64]>:$rhs
    );
    let results = (outs AnyTypeOf<[I32, I64, F32, F64]>:$result);
}

def Wasm_RotrOp : WASM_Op<"rotr"> {
    let arguments = (ins
        AnyTypeOf<[I32, I64, F32, F64]>:$lhs,
        AnyTypeOf<[I32, I64, F32, F64]>:$rhs
    );
    let results = (outs AnyTypeOf<[I32, I64, F32, F64]>:$result);
}

def Wasm_MaxOp : WASM_Op<"max"> {
    let arguments = (ins
        AnyTypeOf<[I32, I64, F32, F64]>:$lhs,
        AnyTypeOf<[I32, I64, F32, F64]>:$rhs
    );
    let results = (outs AnyTypeOf<[I32, I64, F32, F64]>:$result);
}

def Wasm_MinOp : WASM_Op<"min"> {
    let arguments = (ins
        AnyTypeOf<[I32, I64, F32, F64]>:$lhs,
        AnyTypeOf<[I32, I64, F32, F64]>:$rhs
    );
    let results = (outs AnyTypeOf<[I32, I64, F32, F64]>:$result);
}

def Wasm_CopySignOp : WASM_Op<"copysign"> {
    let arguments = (ins
        AnyTypeOf<[I32, I64, F32, F64]>:$lhs,
        AnyTypeOf<[I32, I64, F32, F64]>:$rhs
    );
    let results = (outs AnyTypeOf<[I32, I64, F32, F64]>:$result);
}

def Wasm_CeilOp : WASM_Op<"ceil"> {
    let arguments = (ins
        AnyTypeOf<[I32, I64, F32, F64]>:$input
    );
    let results = (outs AnyTypeOf<[I32, I64, F32, F64]>:$result);
}

def Wasm_FloorOp : WASM_Op<"floor"> {
    let arguments = (ins
        AnyTypeOf<[I32, I64, F32, F64]>:$input
    );
    let results = (outs AnyTypeOf<[I32, I64, F32, F64]>:$result);
}

def Wasm_TruncOp : WASM_Op<"trunc"> {
    let arguments = (ins
        AnyTypeOf<[I32, I64, F32, F64]>:$input
    );
    let results = (outs AnyTypeOf<[I32, I64, F32, F64]>:$result);
}

def Wasm_NearestOp : WASM_Op<"nearest"> {
    let arguments = (ins
        AnyTypeOf<[I32, I64, F32, F64]>:$input
    );
    let results = (outs AnyTypeOf<[I32, I64, F32, F64]>:$result);
}

def Wasm_SqrtOp : WASM_Op<"sqrt"> {
    let arguments = (ins
        AnyTypeOf<[F32, F64]>:$input
    );
    let results = (outs AnyTypeOf<[F32, F64]>:$result);
}

def Wasm_AbsOp : WASM_Op<"abs"> {
    let arguments = (ins
        AnyTypeOf<[I32, I64, F32, F64]>:$input
    );
    let results = (outs AnyTypeOf<[I32, I64, F32, F64]>:$result);
}

def Wasm_NegOp : WASM_Op<"neg"> {
    let arguments = (ins
        AnyTypeOf<[I32, I64, F32, F64]>:$input
    );
    let results = (outs AnyTypeOf<[I32, I64, F32, F64]>:$result);
}

// Select Operation
def Wasm_SelectOp : WASM_Op<"select"> {
    let arguments = (ins
        AnyTypeOf<[I32, I64, F32, F64]>:$lhs,
        AnyTypeOf<[I32, I64, F32, F64]>:$rhs,
        AnyTypeOf<[I32, I64, F32, F64]>:$cond
    );

    let results = (outs AnyTypeOf<[I32, I64, F32, F64]>:$result);
}

def Wasm_CallIndirectOp : WASM_Op<"call_indirect"> {
    let arguments = (ins
        I32:$lhs,
        I32:$rhs,
        I8:$cond
    );
    let results = (outs I32:$value);
}

// ~~~ Conversion Instructions

def Wasm_I32WrapI64Op : WASM_Op<"i32_wrap_i64"> {
    let arguments = (ins I64:$input);
    let results = (outs I32:$result);
}

// I32 Truncation Operations
def Wasm_I32TruncF32SOp : WASM_Op<"i32_trunc_f32s"> {
    let arguments = (ins F32:$input);
    let results = (outs I32:$result);
}

def Wasm_I32TruncF32UOp : WASM_Op<"i32_trunc_f32u"> {
    let arguments = (ins F32:$input);
    let results = (outs I32:$result);
}

def Wasm_I32TruncF64SOp : WASM_Op<"i32_trunc_f64s"> {
    let arguments = (ins F64:$input);
    let results = (outs I32:$result);
}

def Wasm_I32TruncF64UOp : WASM_Op<"i32_trunc_f64u"> {
    let arguments = (ins F64:$input);
    let results = (outs I32:$result);
}

// I64 Extension Operations
def Wasm_I64ExtendI32SOp : WASM_Op<"i64_extend_i32s"> {
    let arguments = (ins I32:$input);
    let results = (outs I64:$result);
}

def Wasm_I64ExtendI32UOp : WASM_Op<"i64_extend_i32u"> {
    let arguments = (ins I32:$input);
    let results = (outs I64:$result);
}

// I64 Truncation Operations
def Wasm_I64TruncF32SOp : WASM_Op<"i64_trunc_f32s"> {
    let arguments = (ins F32:$input);
    let results = (outs I64:$result);
}

def Wasm_I64TruncF32UOp : WASM_Op<"i64_trunc_f32u"> {
    let arguments = (ins F32:$input);
    let results = (outs I64:$result);
}

def Wasm_I64TruncF64SOp : WASM_Op<"i64_trunc_f64s"> {
    let arguments = (ins F64:$input);
    let results = (outs I64:$result);
}

def Wasm_I64TruncF64UOp : WASM_Op<"i64_trunc_f64u"> {
    let arguments = (ins F64:$input);
    let results = (outs I64:$result);
}

// F32 Conversion Operations
def Wasm_F32ConvertI32SOp : WASM_Op<"f32_convert_i32s"> {
    let arguments = (ins I32:$input);
    let results = (outs F32:$result);
}

def Wasm_F32ConvertI32UOp : WASM_Op<"f32_convert_i32u"> {
    let arguments = (ins I32:$input);
    let results = (outs F32:$result);
}

def Wasm_F32ConvertI64SOp : WASM_Op<"f32_convert_i64s"> {
    let arguments = (ins I64:$input);
    let results = (outs F32:$result);
}

def Wasm_F32ConvertI64UOp : WASM_Op<"f32_convert_i64u"> {
    let arguments = (ins I64:$input);
    let results = (outs F32:$result);
}

// F32 to F64 Demote and F64 to F32 Promote
def Wasm_F32DemoteF64Op : WASM_Op<"f32_demote_f64"> {
    let arguments = (ins F64:$input);
    let results = (outs F32:$result);
}

def Wasm_F64ConvertI32SOp : WASM_Op<"f64_convert_i32s"> {
    let arguments = (ins I32:$input);
    let results = (outs F64:$result);
}

def Wasm_F64ConvertI32UOp : WASM_Op<"f64_convert_i32u"> {
    let arguments = (ins I32:$input);
    let results = (outs F64:$result);
}

def Wasm_F64ConvertI64SOp : WASM_Op<"f64_convert_i64s"> {
    let arguments = (ins I64:$input);
    let results = (outs F64:$result);
}

def Wasm_F64ConvertI64UOp : WASM_Op<"f64_convert_i64u"> {
    let arguments = (ins I64:$input);
    let results = (outs F64:$result);
}

// F64 to F32 Promote
def Wasm_F64PromoteF32Op : WASM_Op<"f64_promote_f32"> {
    let arguments = (ins F32:$input);
    let results = (outs F64:$result);
}

// Reinterpret Operations
def Wasm_I32ReinterpretF32Op : WASM_Op<"i32_reinterpret_f32"> {
    let arguments = (ins F32:$input);
    let results = (outs I32:$result);
}

def Wasm_I64ReinterpretF64Op : WASM_Op<"i64_reinterpret_f64"> {
    let arguments = (ins F64:$input);
    let results = (outs I64:$result);
}

def Wasm_F32ReinterpretI32Op : WASM_Op<"f32_reinterpret_i32"> {
    let arguments = (ins I32:$input);
    let results = (outs F32:$result);
}

def Wasm_F64ReinterpretI64Op : WASM_Op<"f64_reinterpret_i64"> {
    let arguments = (ins I64:$input);
    let results = (outs F64:$result);
}

// I32 and I64 Extensions
def Wasm_I32Extend8SOp : WASM_Op<"i32_extend8s"> {
    let arguments = (ins I8:$input);
    let results = (outs I32:$result);
}

def Wasm_I32Extend16SOp : WASM_Op<"i32_extend16s"> {
    let arguments = (ins I16:$input);
    let results = (outs I32:$result);
}

def Wasm_I64Extend8SOp : WASM_Op<"i64_extend8s"> {
    let arguments = (ins I8:$input);
    let results = (outs I64:$result);
}

def Wasm_I64Extend16SOp : WASM_Op<"i64_extend16s"> {
    let arguments = (ins I16:$input);
    let results = (outs I64:$result);
}

def Wasm_I64Extend32SOp : WASM_Op<"i64_extend32s"> {
    let arguments = (ins I32:$input);
    let results = (outs I64:$result);
}

// I32 Truncation Saturation Operations
def Wasm_I32TruncSatF32SOp : WASM_Op<"i32_trunc_sat_f32s"> {
    let arguments = (ins F32:$input);
    let results = (outs I32:$result);
}

def Wasm_I32TruncSatF32UOp : WASM_Op<"i32_trunc_sat_f32u"> {
    let arguments = (ins F32:$input);
    let results = (outs I32:$result);
}

def Wasm_I32TruncSatF64SOp : WASM_Op<"i32_trunc_sat_f64s"> {
    let arguments = (ins F64:$input);
    let results = (outs I32:$result);
}

def Wasm_I32TruncSatF64UOp : WASM_Op<"i32_trunc_sat_f64u"> {
    let arguments = (ins F64:$input);
    let results = (outs I32:$result);
}

// I64 Truncation Saturation Operations
def Wasm_I64TruncSatF32SOp : WASM_Op<"i64_trunc_sat_f32s"> {
    let arguments = (ins F32:$input);
    let results = (outs I64:$result);
}

def Wasm_I64TruncSatF32UOp : WASM_Op<"i64_trunc_sat_f32u"> {
    let arguments = (ins F32:$input);
    let results = (outs I64:$result);
}

def Wasm_I64TruncSatF64SOp : WASM_Op<"i64_trunc_sat_f64s"> {
    let arguments = (ins F64:$input);
    let results = (outs I64:$result);
}

def Wasm_I64TruncSatF64UOp : WASM_Op<"i64_trunc_sat_f64u"> {
    let arguments = (ins F64:$input);
    let results = (outs I64:$result);
}
