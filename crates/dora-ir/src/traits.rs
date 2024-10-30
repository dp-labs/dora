use std::fmt::Debug;

/// The `IRTypes` trait defines the associated types that represent key components of
/// an Intermediate Representation (IR) structure in a compiler or virtual machine.
/// It serves as an abstraction for various IR elements such as types, values, regions,
/// basic blocks, and operations, ensuring they adhere to certain traits like `Copy`, `Eq`,
/// and `Debug`, which are essential for their manipulation and debugging within the IR.
///
/// This trait is useful in scenarios where a system operates on or transforms IR elements
/// in a type-safe and generic manner, providing clear definitions for how these components
/// behave within the context of the IR.
///
/// # Associated Types:
/// - `Type`: Represents the IR's type system, which defines the types of values,
///   operations, and blocks in the IR. Must be `Copy`, `Eq`, and `Debug`.
/// - `Value`: Represents a single value in the IR, such as an SSA (Static Single Assignment)
///   value. Must be `Copy`, `Eq`, and `Debug`.
/// - `Region`: Represents a region within the IR, which is often a container for
///   basic blocks or operations. Must be `Eq` and `Debug`.
/// - `BasicBlock`: Represents a basic block in the IR, which is a sequence of operations
///   or instructions. Must be `Copy`, `Eq`, and `Debug`.
/// - `Operation`: Represents an operation in the IR, which can perform computations,
///   manipulate values, or control execution flow. Must be `Eq` and `Debug`.
///
/// # Example Usage:
/// Implementations of this trait define how the IR elements are structured, allowing
/// transformations or analyses to be performed on IRs that conform to these type
/// definitions.
///
/// This trait is typically used when building compiler infrastructures,
/// virtual machines, or interpreters that need to work generically over IR elements.
pub trait IRTypes: Sized {
    /// Represents the type system of the IR. This can refer to primitive types, complex types,
    /// or user-defined types in the IR.
    ///
    /// Must implement `Copy`, `Eq`, and `Debug` to ensure type information can be safely
    /// copied, compared, and debugged.
    type Type: Copy + Eq + Debug;

    /// Represents values in the IR, such as SSA values or constants.
    ///
    /// Must implement `Copy`, `Eq`, and `Debug` to ensure values can be safely copied,
    /// compared, and debugged.
    type Value: Copy + Eq + Debug;

    /// Represents a region in the IR, typically containing a sequence of basic blocks
    /// or operations.
    ///
    /// Must implement `Eq` and `Debug` to ensure regions can be compared and debugged.
    type Region: Eq + Debug;

    /// Represents a basic block in the IR. A basic block is a sequence of operations
    /// executed linearly.
    ///
    /// Must implement `Copy`, `Eq`, and `Debug` to ensure blocks can be copied, compared,
    /// and debugged.
    type BasicBlock: Eq + Debug + Copy;

    /// Represents an operation in the IR, which can either produce values or control
    /// the flow of execution.
    ///
    /// Must implement `Eq` and `Debug` to ensure operations can be compared and debugged.
    type Operation: Eq + Debug;
}
