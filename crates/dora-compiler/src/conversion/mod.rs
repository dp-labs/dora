pub mod builder;
pub mod rewriter;
pub mod walker;

#[cfg(test)]
mod tests;

pub use builder::OpBuilder;
pub use rewriter::{DeferredRewriter, Replacer, Rewriter, move_all_ops_before_op, replace_op};
pub use walker::{WalkFn, walk_operation};
