use crate::{
    conversion::{
        builder::OpBuilder,
        rewriter::{DeferredRewriter, move_all_ops_before_op},
        walker::walk_operation,
    },
    errors::{CompileError, Result},
    value::{IntoContextOperation, ToContextValue},
};
use melior::{
    Context,
    dialect::{func, scf},
    ir::{Block, Module, OperationRef, Region, Type, attribute::FlatSymbolRefAttribute},
};

pub(crate) const STORAGE_MEMORY_MAP_CODE: &str = include_str!("mlir/storage.mlir");
pub(crate) const DEFAULT_MEMORY_MAP_CAP: usize = 16;
pub(crate) mod symbols {
    pub(crate) const U256_MAP_NEW: &str = "dora_u256_map_new";
    pub(crate) const U256_MAP_INSERT: &str = "dora_u256_map_insert";
    pub(crate) const U256_MAP_GET: &str = "dora_u256_map_get";
}

#[derive(Clone, Debug)]
pub struct StoragePass<'c> {
    /// A reference to the MLIR context, which manages global state and resources required for MLIR operations.
    pub ctx: &'c Context,
    /// Storage memory cache capacity.
    pub capacity: usize,
}

impl<'c> StoragePass<'c> {
    pub fn new(ctx: &'c Context) -> Self {
        Self {
            ctx,
            capacity: DEFAULT_MEMORY_MAP_CAP,
        }
    }
    pub fn run(&mut self, operation: OperationRef<'_, '_>) -> Result<()> {
        let mut storage_ops = vec![];
        let mut has_sstore = false;
        let mut has_sload = false;
        walk_operation(
            operation,
            Box::new(|op| {
                let name = op.name().as_string_ref().as_str().unwrap().to_string();
                if let Ok(dora_op) = dora_ir::Operation::try_from(name.as_str()) {
                    if matches!(dora_op, dora_ir::Operation::SStore) {
                        has_sstore = true;
                        storage_ops.push(op.to_ctx_operation_ref());
                    }
                    if matches!(dora_op, dora_ir::Operation::SLoad) {
                        has_sload = true;
                        storage_ops.push(op.to_ctx_operation_ref());
                    }
                }
                Ok(())
            }),
        )?;
        let use_memory_cache = has_sstore && has_sload;
        if use_memory_cache {
            let is_created = self.create_storage_memory_cache_function_in_module(operation)?;
            if is_created {
                let op = storage_ops
                    .first()
                    .ok_or(anyhow::anyhow!(CompileError::Codegen(
                        "generate storage memory cache failed: get the first dora ops".to_string()
                    )))?;
                let block = op.block().ok_or(anyhow::anyhow!(CompileError::Codegen(
                    "generate storage memory cache failed: get block from dora ops".to_string()
                )))?;
                let region =
                    block
                        .parent_region()
                        .ok_or(anyhow::anyhow!(CompileError::Codegen(
                            "generate storage memory cache failed: get region from dora ops"
                                .to_string()
                        )))?;
                let setup_block =
                    region
                        .first_block()
                        .ok_or(anyhow::anyhow!(CompileError::Codegen(
                            "generate storage memory cache failed: get setup block from dora ops"
                                .to_string()
                        )))?;
                let op =
                    setup_block
                        .first_operation()
                        .ok_or(anyhow::anyhow!(CompileError::Codegen(
                    "generate storage memory cache failed: get setup block first op from dora ops"
                        .to_string()
                )))?;
                let builder = OpBuilder::new_with_op(self.ctx, op);
                let cap = builder.make(builder.index(self.capacity))?;
                let map_ty = Type::parse(self.ctx, "memref<?x3xi256>").unwrap();
                let mut map = builder.make(func::call(
                    self.ctx,
                    FlatSymbolRefAttribute::new(self.ctx, symbols::U256_MAP_NEW),
                    &[cap],
                    &[map_ty],
                    builder.get_insert_location(),
                ))?;
                for op in &storage_ops {
                    let name = op.name().as_string_ref().as_str().unwrap().to_string();
                    if let Ok(dora_op) = dora_ir::Operation::try_from(name.as_str()) {
                        let key = op.operand(0)?;
                        match dora_op {
                            dora_ir::Operation::SStore => {
                                let builder = OpBuilder::new_with_op(self.ctx, *op);
                                let value = op.operand(1)?;
                                map = builder
                                    .make(func::call(
                                        self.ctx,
                                        FlatSymbolRefAttribute::new(
                                            self.ctx,
                                            symbols::U256_MAP_INSERT,
                                        ),
                                        &[map, key, value],
                                        &[map_ty],
                                        builder.get_insert_location(),
                                    ))?
                                    .to_ctx_value();
                            }
                            dora_ir::Operation::SLoad => {
                                let rewriter = DeferredRewriter::new_with_op(self.ctx, *op);
                                let [map_value, found] = rewriter.make_n(func::call(
                                    self.ctx,
                                    FlatSymbolRefAttribute::new(self.ctx, symbols::U256_MAP_GET),
                                    &[map, key],
                                    &[rewriter.intrinsics.i256_ty, rewriter.intrinsics.i1_ty],
                                    rewriter.get_insert_location(),
                                ))?;
                                rewriter.create(scf::r#if(
                                    found,
                                    &[rewriter.intrinsics.i256_ty],
                                    {
                                        let region = Region::new();
                                        let builder = OpBuilder::new_with_block(
                                            self.ctx,
                                            region.append_block(Block::new(&[])),
                                        );
                                        builder.create(scf::r#yield(
                                            &[map_value],
                                            builder.get_insert_location(),
                                        ));
                                        region
                                    },
                                    {
                                        let region = Region::new();
                                        let builder = OpBuilder::new_with_block(
                                            self.ctx,
                                            region.append_block(Block::new(&[])),
                                        );
                                        let storage_value = builder.make(
                                            dora_ir::dora::sload(
                                                self.ctx,
                                                rewriter.intrinsics.i256_ty,
                                                key,
                                                builder.get_insert_location(),
                                            )
                                            .into(),
                                        )?;
                                        builder.create(scf::r#yield(
                                            &[storage_value],
                                            builder.get_insert_location(),
                                        ));
                                        region
                                    },
                                    rewriter.get_insert_location(),
                                ));
                                rewriter.remove();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn create_storage_memory_cache_function_in_module(
        &mut self,
        operation: OperationRef<'_, '_>,
    ) -> Result<bool> {
        let module = Module::parse(self.ctx, STORAGE_MEMORY_MAP_CODE).ok_or(anyhow::anyhow!(
            CompileError::Codegen(
                "generate storage memory cache failed: parse storage.mlir failed".to_string()
            )
        ))?;
        if let Ok(region) = operation.region(0) {
            if let Some(block) = region.first_block() {
                if let Some(op) = block.first_operation() {
                    move_all_ops_before_op(&module, op);
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}
