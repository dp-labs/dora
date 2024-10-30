use std::fmt::Debug;

use melior::{
    ir::{BlockRef, Location, Module as MLIRModule},
    Context as MLIRContext,
};

/// The `Module` struct encapsulates an MLIR module, representing a top-level container for
/// functions, operations, and other constructs in the MLIR framework.
///
/// # Fields:
/// - `module`: An `MLIRModule` that holds the underlying MLIR module context. This context is responsible
///   for managing the lifetime and scope of the MLIR constructs within the module.
///
/// # Example Usage:
/// ```no_check
/// let mlir_module = MLIRModule::new();
/// let module = Module::new(mlir_module);
/// ```
///
/// # Notes:
/// - MLIR modules are the root container in an MLIR program and typically contain multiple functions
///   and operations.
/// - This struct allows for safe encapsulation of the underlying MLIR module to ensure consistent
///   context management across MLIR-related operations.
pub struct Module<'m> {
    /// The underlying MLIR module context that holds the top-level constructs and operations
    /// within the module.
    pub mlir_module: MLIRModule<'m>,
}

impl<'m> Module<'m> {
    /// Creates a new `Module` from an existing `MLIRModule`.
    ///
    /// # Arguments
    ///
    /// * `module` - An existing `MLIRModule` that will be wrapped in the `Module` structure.
    ///
    /// # Returns
    ///
    /// A `Module` instance that wraps the given `MLIRModule`.
    pub fn new(module: MLIRModule<'m>) -> Self {
        Self {
            mlir_module: module,
        }
    }

    /// Creates an empty `Module` within the given `MLIRContext`.
    ///
    /// # Arguments
    ///
    /// * `context` - The `MLIRContext` in which the empty module will be created.
    ///
    /// # Returns
    ///
    /// A new `Module` containing an empty `MLIRModule` initialized at an unknown location.
    pub fn empty(context: &MLIRContext) -> Self {
        Self {
            mlir_module: MLIRModule::new(Location::unknown(context)),
        }
    }

    /// Provides immutable access to the wrapped `MLIRModule`.
    ///
    /// # Returns
    ///
    /// A reference to the underlying `MLIRModule`.
    pub fn module(&self) -> &MLIRModule {
        &self.mlir_module
    }

    /// Provides mutable access to the wrapped `MLIRModule`.
    ///
    /// # Returns
    ///
    /// A mutable reference to the underlying `MLIRModule`.
    pub fn module_mut(&mut self) -> &'m mut MLIRModule {
        &mut self.mlir_module
    }

    /// Retrieves the body of the `MLIRModule` as a `BlockRef`.
    ///
    /// # Returns
    ///
    /// A `BlockRef` representing the body of the module.
    pub fn body(&self) -> BlockRef<'m, '_> {
        self.mlir_module.body()
    }

    /// Parses an `MLIRModule` from a string representation within a given context.
    ///
    /// # Arguments
    ///
    /// * `context` - The `MLIRContext` in which to parse the module.
    /// * `source` - The MLIR source code as a string.
    ///
    /// # Returns
    ///
    /// An `Option<Self>` containing the parsed `Module` if successful, or `None` if the parsing fails.
    pub fn parse(context: &MLIRContext, source: &str) -> Option<Self> {
        MLIRModule::parse(context, source).map(Self::new)
    }
}

impl Debug for Module<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.mlir_module.as_operation().to_string())
    }
}
