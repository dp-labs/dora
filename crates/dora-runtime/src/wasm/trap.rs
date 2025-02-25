use std::any::Any;
use std::panic::panic_any;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[repr(u32)]
pub enum TrapCode {
    /// The current stack space was exhausted.
    ///
    /// On some platforms, a stack overflow may also be indicated by a segmentation fault from the
    /// stack guard page.
    StackOverflow = 0,

    /// A `heap_addr` instruction detected an out-of-bounds error.
    ///
    /// Note that not all out-of-bounds heap accesses are reported this way;
    /// some are detected by a segmentation fault on the heap unmapped or
    /// offset-guard pages.
    HeapAccessOutOfBounds = 1,

    /// A `heap_addr` instruction was misaligned.
    HeapMisaligned = 2,

    /// A `table_addr` instruction detected an out-of-bounds error.
    TableAccessOutOfBounds = 3,

    /// Indirect call to a null table entry.
    IndirectCallToNull = 4,

    /// Signature mismatch on indirect call.
    BadSignature = 5,

    /// An integer arithmetic operation caused an overflow.
    IntegerOverflow = 6,

    /// An integer division by zero.
    IntegerDivisionByZero = 7,

    /// Failed float-to-int conversion.
    BadConversionToInteger = 8,

    /// Code that was supposed to have been unreachable was reached.
    UnreachableCodeReached = 9,

    /// An atomic memory access was attempted with an unaligned pointer.
    UnalignedAtomic = 10,

    /// Out of gas.
    OutOfGas = 20,
}

impl TrapCode {
    /// Gets the message for this trap code
    pub fn message(&self) -> &str {
        match self {
            Self::StackOverflow => "call stack exhausted",
            Self::HeapAccessOutOfBounds => "out of bounds memory access",
            Self::HeapMisaligned => "misaligned heap",
            Self::TableAccessOutOfBounds => "undefined element: out of bounds table access",
            Self::IndirectCallToNull => "uninitialized element",
            Self::BadSignature => "indirect call type mismatch",
            Self::IntegerOverflow => "integer overflow",
            Self::IntegerDivisionByZero => "integer divide by zero",
            Self::BadConversionToInteger => "invalid conversion to integer",
            Self::UnreachableCodeReached => "unreachable",
            Self::UnalignedAtomic => "unaligned atomic access",
            Self::OutOfGas => "out of gas",
        }
    }
}

/// Implementation for raising a WASM trap
pub fn wasm_raise_trap(trap_code: TrapCode) -> ! {
    panic_any(trap_code)
}

/// Trap error to string
pub fn err_to_str(err: Box<dyn Any + Send>) -> String {
    if let Some(trap) = err.downcast_ref::<TrapCode>() {
        trap.message().to_string()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = err.downcast_ref::<&String>() {
        (*s).clone()
    } else if let Some(s) = err.downcast_ref::<String>() {
        (*s).clone()
    } else {
        "".to_string()
    }
}

/// WASM trap error to string
#[inline]
pub fn wasm_trap_to_str(trap: wasmer_vm::Trap) -> String {
    match trap {
        wasmer_vm::Trap::User(error) => error.to_string(),
        wasmer_vm::Trap::Wasm { .. } => "WASM generated code error".to_string(),
        wasmer_vm::Trap::Lib { trap_code, .. } => trap_code.message().to_string(),
        wasmer_vm::Trap::OOM { .. } => "WASM OOM error".to_string(),
    }
}
