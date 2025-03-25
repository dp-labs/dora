use crate::errors::{CompileError, Result};
use anyhow::bail;
use smallvec::SmallVec;

pub type BasicBlock<'c, 'a> = melior::ir::BlockRef<'c, 'a>;
pub type BasicValue<'c, 'a> = melior::ir::Value<'c, 'a>;
pub type PhiValue<'c> = (melior::ir::Type<'c>, melior::ir::Location<'c>);

/// Represents a control frame in a control flow structure, encapsulating the state of blocks,
/// loops, and conditional branches within a function. This is used to manage control flow during
/// the execution of WebAssembly-like bytecode.
///
/// The `ControlFrame` is parameterized by lifetime `'c` (context) and `'a` (allocator), allowing
/// control frames to hold references to basic blocks and PHI nodes tied to the control flow.
///
/// # Variants
///
/// * `Block` - Represents a basic block structure in the control flow. It contains:
///     - `next`: The next basic block to execute after this block completes.
///     - `phis`: A collection of PHI nodes for SSA values in the block.
///     - `stack_size_snapshot`: A snapshot of the stack size at the time this block was entered.
///
/// * `Loop` - Represents a loop control structure in the flow. It contains:
///     - `body`: The body of the loop (the block where the loop begins).
///     - `next`: The next basic block to execute after the loop completes.
///     - `phis`: A collection of PHI nodes for SSA values.
///     - `loop_body_phis`: PHI nodes for SSA values in the loop body.
///     - `stack_size_snapshot`: A snapshot of the stack size at the time this loop was entered.
///
/// * `IfElse` - Represents a conditional branching structure with both "then" and "else" paths.
///     - `if_then`: The block to execute if the condition evaluates to true.
///     - `if_else`: The block to execute if the condition evaluates to false.
///     - `next`: The next basic block to execute after the branching completes.
///     - `then_phis`: PHI nodes for SSA values in the "then" branch.
///     - `else_phis`: PHI nodes for SSA values in the "else" branch.
///     - `next_phis`: PHI nodes for SSA values after the branch completes.
///     - `stack_size_snapshot`: A snapshot of the stack size at the time the branch was entered.
///     - `if_else_state`: Current state of the conditional, either `If` or `Else`.
#[derive(Debug)]
pub enum ControlFrame<'c, 'a> {
    Block {
        next: BasicBlock<'c, 'a>,
        phis: SmallVec<[PhiValue<'c>; 1]>,
        stack_size_snapshot: usize,
    },
    Loop {
        body: BasicBlock<'c, 'a>,
        next: BasicBlock<'c, 'a>,
        phis: SmallVec<[PhiValue<'c>; 1]>,
        loop_body_phis: SmallVec<[PhiValue<'c>; 1]>,
        stack_size_snapshot: usize,
    },
    IfElse {
        if_then: BasicBlock<'c, 'a>,
        if_else: BasicBlock<'c, 'a>,
        next: BasicBlock<'c, 'a>,
        then_phis: SmallVec<[PhiValue<'c>; 1]>,
        else_phis: SmallVec<[PhiValue<'c>; 1]>,
        next_phis: SmallVec<[PhiValue<'c>; 1]>,
        stack_size_snapshot: usize,
        if_else_state: IfElseState,
    },
}

/// Represents the current state of an `IfElse` control frame.
///
/// # Variants
///
/// * `If` - The "then" block of the conditional branch is currently being executed.
/// * `Else` - The "else" block of the conditional branch is currently being executed.
#[derive(Debug)]
pub enum IfElseState {
    If,
    Else,
}

impl<'c, 'a> ControlFrame<'c, 'a> {
    /// Returns the next `BasicBlock` to execute after the control frame completes.
    ///
    /// # Returns
    /// A reference to the next `BasicBlock` for the control flow.
    pub fn code_after(&self) -> &BasicBlock<'c, 'a> {
        match self {
            ControlFrame::Block { next, .. }
            | ControlFrame::Loop { next, .. }
            | ControlFrame::IfElse { next, .. } => next,
        }
    }

    /// Returns the destination `BasicBlock` for a branch instruction.
    ///
    /// In the case of loops, this will return the loop body. For other blocks and conditionals,
    /// it returns the next block in the flow.
    ///
    /// # Returns
    /// A reference to the branch destination `BasicBlock`.
    pub fn br_dest(&self) -> &BasicBlock<'c, 'a> {
        match self {
            ControlFrame::Block { next, .. } | ControlFrame::IfElse { next, .. } => next,
            ControlFrame::Loop { body, .. } => body,
        }
    }

    /// Returns the list of PHI nodes associated with the current control frame.
    ///
    /// This can be PHI nodes for SSA values in blocks or loops.
    ///
    /// # Returns
    /// A slice of PHI nodes (`[PhiValue<'c>]`) for the current control frame.
    pub fn phis(&self) -> &[PhiValue<'c>] {
        match self {
            ControlFrame::Block { phis, .. } | ControlFrame::Loop { phis, .. } => phis.as_slice(),
            ControlFrame::IfElse { next_phis, .. } => next_phis.as_slice(),
        }
    }

    /// Returns the list of PHI nodes for stack values specific to the loop body.
    ///
    /// This is only applicable in the case of a `Loop` control frame. For other variants,
    /// this returns an empty slice.
    ///
    /// # Returns
    /// A slice of loop body PHI nodes (`[PhiValue<'c>]`).
    pub fn loop_body_phis(&self) -> &[PhiValue<'c>] {
        match self {
            ControlFrame::Block { .. } | ControlFrame::IfElse { .. } => &[],
            ControlFrame::Loop { loop_body_phis, .. } => loop_body_phis.as_slice(),
        }
    }

    /// Checks whether the current control frame represents a loop structure.
    ///
    /// # Returns
    /// `true` if the control frame is a `Loop`, otherwise `false`.
    pub fn is_loop(&self) -> bool {
        matches!(self, ControlFrame::Loop { .. })
    }
}

/// The `ExtraInfo` struct provides additional metadata or state information associated
/// with values in the stack. This can be useful to track state transitions or special
/// attributes during execution.
///
/// # Fields:
/// - `state`: A `u8` field representing the state or additional information related to a stack value.
///   This can be used to encode various states or flags that may be needed for tracking purposes.
///
/// # Trait Implementations:
/// - `Debug`, `Default`, `Eq`, `PartialEq`, `Copy`, `Clone`, `Hash`: Standard trait implementations
///   that allow the struct to be debugged, compared, cloned, and hashed.
///
/// # Example:
/// ```
/// use dora_compiler::state::ExtraInfo;
/// 
/// let info = ExtraInfo { state: 0 };
/// let default_info = ExtraInfo::default();
/// ```
///
/// # Purpose:
/// This struct is intended to hold auxiliary state information alongside other values, particularly
/// in contexts where execution may require tracking state transitions or conditions.
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash)]
pub struct ExtraInfo {
    /// A `u8` field representing the additional state or flag associated with a value.
    pub state: u8,
}

/// The `State` struct manages the state of the execution context, particularly the stack and control flow.
/// It maintains two main stacks: a value stack for holding operands and their associated metadata, and a
/// control stack for managing control flow constructs like function calls, loops, or conditionals.
///
/// # Fields:
/// - `stack`: A vector representing the value stack. Each entry is a tuple of a `BasicValue` and its
///   associated `ExtraInfo` metadata. This stack holds values during the execution of a program or operation.
/// - `control_stack`: A vector representing the control flow stack, which holds `ControlFrame`s. This stack is
///   used to manage control flow constructs such as blocks, loops, or function frames.
/// - `reachable`: A boolean flag indicating whether the current block or instruction is reachable. This is
///   particularly useful in scenarios involving conditional or branching logic, where the control flow may
///   render certain paths unreachable.
///
/// # Example Usage:
/// ```
/// use dora_compiler::state::State;
/// let state = State::new();
/// ```
///
/// # Purpose:
/// The `State` struct is central to the execution model, tracking both operand values and control flow.
/// It is designed to support operations on a stack-based virtual machine, where values are pushed to
/// and popped from the stack, and control flow is managed via the control stack.
///
/// # Notes:
/// - The value stack (`stack`) is used to store operands and intermediate results during execution.
/// - The control stack (`control_stack`) is used for managing higher-level constructs such as function
///   frames, loops, or conditional branches.
/// - The `reachable` flag helps optimize execution by identifying when a block of code can be skipped
///   due to control flow conditions.
#[derive(Debug)]
pub struct State<'c, 'a> {
    /// The stack holds a list of tuples consisting of `BasicValue` and its associated `ExtraInfo`.
    /// This stack represents operand values and their metadata during execution.
    pub stack: Vec<(BasicValue<'c, 'a>, ExtraInfo)>,
    /// The control stack holds `ControlFrame` objects that represent control flow constructs
    /// such as loops, blocks, or function frames.
    control_stack: Vec<ControlFrame<'c, 'a>>,
    /// A flag indicating whether the current code or block is reachable during execution.
    /// This is important for managing execution paths, particularly in conditional or branching
    /// contexts.
    pub reachable: bool,
}

impl Default for State<'_, '_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'c, 'a> State<'c, 'a> {
    /// Creates a new, empty `State` instance with an empty stack and control stack.
    /// The `reachable` field is set to true by default.
    pub fn new() -> Self {
        Self {
            stack: vec![],
            control_stack: vec![],
            reachable: true,
        }
    }

    /// Checks if there are any control frames currently on the control stack.
    ///
    /// # Returns
    /// `true` if the control stack contains frames, otherwise `false`.
    pub fn has_control_frames(&self) -> bool {
        !self.control_stack.is_empty()
    }

    /// Resets the value stack to the snapshot size recorded in the provided control frame.
    ///
    /// This is typically used to truncate the value stack after completing a block or loop.
    pub fn reset_stack(&mut self, frame: &ControlFrame<'c, 'a>) {
        let stack_size_snapshot = match frame {
            ControlFrame::Block {
                stack_size_snapshot,
                ..
            }
            | ControlFrame::Loop {
                stack_size_snapshot,
                ..
            }
            | ControlFrame::IfElse {
                stack_size_snapshot,
                ..
            } => *stack_size_snapshot,
        };
        self.stack.truncate(stack_size_snapshot);
    }

    /// Retrieves the outermost (first) control frame from the control stack.
    ///
    /// # Errors
    /// Returns an error if the control stack is empty.
    pub fn outermost_frame(&self) -> Result<&ControlFrame<'c, 'a>> {
        self.control_stack.first().ok_or_else(|| {
            anyhow::anyhow!(CompileError::Codegen(
                "outermost_frame: invalid control stack depth".to_string()
            ))
        })
    }

    /// Retrieves the control frame at a specific depth in the control stack.
    ///
    /// # Parameters
    /// - `depth`: The depth of the frame, where 0 is the top of the stack.
    ///
    /// # Errors
    /// Returns an error if the requested depth is greater than the number of frames in the stack.
    pub fn frame_at_depth(&self, depth: u32) -> Result<&ControlFrame<'c, 'a>> {
        let index = self
            .control_stack
            .len()
            .checked_sub(1 + (depth as usize))
            .ok_or_else(|| {
                CompileError::Codegen("frame_at_depth: invalid control stack depth".to_string())
            })?;
        Ok(&self.control_stack[index])
    }

    /// Retrieves a mutable reference to the control frame at a specific depth in the control stack.
    ///
    /// # Parameters
    /// - `depth`: The depth of the frame, where 0 is the top of the stack.
    ///
    /// # Errors
    /// Returns an error if the requested depth is greater than the number of frames in the stack.
    pub fn frame_at_depth_mut(&mut self, depth: u32) -> Result<&mut ControlFrame<'c, 'a>> {
        let index = self
            .control_stack
            .len()
            .checked_sub(1 + (depth as usize))
            .ok_or_else(|| {
                CompileError::Codegen("frame_at_depth_mut: invalid control stack depth".to_string())
            })?;
        Ok(&mut self.control_stack[index])
    }

    /// Pops the top control frame off the control stack.
    ///
    /// # Errors
    /// Returns an error if the control stack is empty.
    pub fn pop_frame(&mut self) -> Result<ControlFrame<'c, 'a>> {
        self.control_stack.pop().ok_or_else(|| {
            anyhow::anyhow!(CompileError::Codegen(
                "pop_frame: cannot pop from control stack".to_string()
            ))
        })
    }

    /// Pushes a single value onto the value stack.
    ///
    /// # Parameters
    /// - `value`: The `BasicValue` to push onto the stack.
    pub fn push1(&mut self, value: BasicValue<'c, 'a>) {
        self.push1_extra(value, Default::default());
    }

    /// Pushes a single value along with additional metadata onto the value stack.
    ///
    /// # Parameters
    /// - `value`: The `BasicValue` to push onto the stack.
    /// - `info`: Extra metadata associated with the value.
    pub fn push1_extra(&mut self, value: BasicValue<'c, 'a>, info: ExtraInfo) {
        self.stack.push((value, info));
    }

    /// Pops the top value off the value stack and returns it.
    ///
    /// # Errors
    /// Returns an error if the value stack is empty.
    pub fn pop1(&mut self) -> Result<BasicValue<'c, 'a>> {
        Ok(self.pop1_extra()?.0)
    }

    /// Pops the top value along with additional metadata off the value stack.
    ///
    /// # Errors
    /// Returns an error if the value stack is empty.
    pub fn pop1_extra(&mut self) -> Result<(BasicValue<'c, 'a>, ExtraInfo)> {
        self.stack.pop().ok_or_else(|| {
            anyhow::anyhow!(CompileError::Codegen(
                "pop1_extra: invalid value stack".to_string()
            ))
        })
    }

    /// Pops two values off the value stack and returns them as a tuple.
    ///
    /// # Errors
    /// Returns an error if there are fewer than two values on the stack.
    pub fn pop2(&mut self) -> Result<(BasicValue<'c, 'a>, BasicValue<'c, 'a>)> {
        let v2 = self.pop1()?;
        let v1 = self.pop1()?;
        Ok((v1, v2))
    }

    /// Pops two values with additional metadata off the value stack and returns them as a tuple.
    ///
    /// # Errors
    /// Returns an error if there are fewer than two values on the stack.
    #[allow(clippy::type_complexity)]
    pub fn pop2_extra(
        &mut self,
    ) -> Result<(
        (BasicValue<'c, 'a>, ExtraInfo),
        (BasicValue<'c, 'a>, ExtraInfo),
    )> {
        let v2 = self.pop1_extra()?;
        let v1 = self.pop1_extra()?;
        Ok((v1, v2))
    }

    /// Pops three values from the top of the stack and returns them in reverse order.
    ///
    /// # Errors
    ///
    /// Returns an error if there are not enough values on the stack.
    pub fn pop3(&mut self) -> Result<(BasicValue<'c, 'a>, BasicValue<'c, 'a>, BasicValue<'c, 'a>)> {
        let v3 = self.pop1()?;
        let v2 = self.pop1()?;
        let v1 = self.pop1()?;
        Ok((v1, v2, v3))
    }

    /// Pops three values along with their associated extra information from the top of the stack
    /// and returns them in reverse order.
    ///
    /// # Errors
    ///
    /// Returns an error if there are not enough values on the stack.
    #[allow(clippy::type_complexity)]
    pub fn pop3_extra(
        &mut self,
    ) -> Result<(
        (BasicValue<'c, 'a>, ExtraInfo),
        (BasicValue<'c, 'a>, ExtraInfo),
        (BasicValue<'c, 'a>, ExtraInfo),
    )> {
        let v3 = self.pop1_extra()?;
        let v2 = self.pop1_extra()?;
        let v1 = self.pop1_extra()?;
        Ok((v1, v2, v3))
    }

    /// Peeks at the value at the top of the stack without removing it.
    ///
    /// # Errors
    ///
    /// Returns an error if the stack is empty.
    pub fn peek1(&self) -> Result<BasicValue<'c, 'a>> {
        let index =
            self.stack.len().checked_sub(1).ok_or_else(|| {
                CompileError::Codegen("peek1_extra: invalid value stack".to_string())
            })?;
        Ok(self.stack[index].0)
    }

    /// Peeks at the value and extra information at the top of the stack without removing them.
    ///
    /// # Errors
    ///
    /// Returns an error if the stack is empty.
    pub fn peek1_extra(&self) -> Result<(BasicValue<'c, 'a>, ExtraInfo)> {
        let index =
            self.stack.len().checked_sub(1).ok_or_else(|| {
                CompileError::Codegen("peek1_extra: invalid value stack".to_string())
            })?;
        Ok(self.stack[index])
    }

    /// Peeks at the top `n` values from the stack without removing them.
    ///
    /// # Errors
    ///
    /// Returns an error if there are not enough values on the stack.
    pub fn peekn(&self, n: usize) -> Result<Vec<BasicValue<'c, 'a>>> {
        Ok(self.peekn_extra(n)?.iter().map(|x| x.0).collect())
    }

    /// Peeks at the `n`-th value from the top of the stack without removing it.
    ///
    /// # Errors
    ///
    /// Returns an error if the index is out of bounds.
    pub fn peeknth(&self, n: usize) -> Result<BasicValue<'c, 'a>> {
        let index = self.stack.len().checked_sub(n).ok_or_else(|| {
            CompileError::Codegen(format!("peeknth for size {n}: invalid value stack"))
        })?;
        Ok(self.stack[index].0)
    }

    /// Peeks at the top `n` values from the stack along with their extra information, without removing them.
    ///
    /// # Errors
    ///
    /// Returns an error if there are not enough values on the stack.
    pub fn peekn_extra(&self, n: usize) -> Result<&[(BasicValue<'c, 'a>, ExtraInfo)]> {
        let index = self.stack.len().checked_sub(n).ok_or_else(|| {
            CompileError::Codegen(format!("peekn_extra for size {n}: invalid value stack"))
        })?;
        Ok(&self.stack[index..])
    }

    /// Pops `n` values from the stack and saves their extra information, returning the values and their associated extra information.
    ///
    /// # Errors
    ///
    /// Returns an error if there are not enough values on the stack.
    pub fn popn_save_extra(&mut self, n: usize) -> Result<Vec<(BasicValue<'c, 'a>, ExtraInfo)>> {
        let v = self.peekn_extra(n)?.to_vec();
        self.popn(n)?;
        Ok(v)
    }

    /// Pops `n` values from the stack.
    ///
    /// # Errors
    ///
    /// Returns an error if there are not enough values on the stack.
    pub fn popn(&mut self, n: usize) -> Result<()> {
        let index = self.stack.len().checked_sub(n).ok_or_else(|| {
            CompileError::Codegen(format!("popn for size {n}: invalid value stack"))
        })?;

        self.stack.truncate(index);
        Ok(())
    }

    /// Pushes a block frame onto the control stack.
    ///
    /// # Parameters
    ///
    /// - `next`: The basic block to jump to after the block.
    /// - `phis`: The phi nodes for the block.
    pub fn push_block(&mut self, next: BasicBlock<'c, 'a>, phis: SmallVec<[PhiValue<'c>; 1]>) {
        self.control_stack.push(ControlFrame::Block {
            next,
            phis,
            stack_size_snapshot: self.stack.len(),
        });
    }

    /// Pushes a loop frame onto the control stack.
    ///
    /// # Parameters
    ///
    /// - `body`: The basic block representing the body of the loop.
    /// - `next`: The basic block to jump to after the loop.
    /// - `loop_body_phis`: The phi nodes for the loop body.
    /// - `phis`: The phi nodes for the loop.
    pub fn push_loop(
        &mut self,
        body: BasicBlock<'c, 'a>,
        next: BasicBlock<'c, 'a>,
        loop_body_phis: SmallVec<[PhiValue<'c>; 1]>,
        phis: SmallVec<[PhiValue<'c>; 1]>,
    ) {
        self.control_stack.push(ControlFrame::Loop {
            body,
            next,
            loop_body_phis,
            phis,
            stack_size_snapshot: self.stack.len(),
        });
    }

    /// Pushes an if-else frame onto the control stack.
    ///
    /// # Parameters
    ///
    /// - `if_then`: The basic block for the "then" branch.
    /// - `if_else`: The basic block for the "else" branch.
    /// - `next`: The basic block to jump to after the if-else.
    /// - `then_phis`: The phi nodes for the "then" branch.
    /// - `else_phis`: The phi nodes for the "else" branch.
    /// - `next_phis`: The phi nodes for the block after the if-else.
    pub fn push_if(
        &mut self,
        if_then: BasicBlock<'c, 'a>,
        if_else: BasicBlock<'c, 'a>,
        next: BasicBlock<'c, 'a>,
        then_phis: SmallVec<[PhiValue<'c>; 1]>,
        else_phis: SmallVec<[PhiValue<'c>; 1]>,
        next_phis: SmallVec<[PhiValue<'c>; 1]>,
    ) {
        self.control_stack.push(ControlFrame::IfElse {
            if_then,
            if_else,
            next,
            then_phis,
            else_phis,
            next_phis,
            stack_size_snapshot: self.stack.len(),
            if_else_state: IfElseState::If,
        });
    }

    /// Swaps two elements on the stack by their indices.
    ///
    /// # Parameters
    ///
    /// - `n`: The index of the first element.
    /// - `m`: The index of the second element.
    ///
    /// # Errors
    ///
    /// Returns an error if the indices are out of bounds.
    pub fn exchange(&mut self, n: usize, m: usize) -> Result<()> {
        let len = self.stack.len();

        if n >= len || m >= len {
            bail!(CompileError::Codegen(
                "exchange: invalid indices for value stack".to_string(),
            ));
        }

        self.stack.swap(n, m);

        Ok(())
    }
}
