(module
  (memory 1)

  (func (export "fill") (param i32 i32)
    (memory.fill
      (local.get 0)
      (i32.const 0xFF)
      (local.get 1)))

  (func (export "fill_all")
    (memory.fill
      (i32.const 0)
      (i32.const 0)
      (i32.const 0x10000)))

  (func (export "fill_end_of_memory")
    (memory.fill
      (i32.const 0x10000)
      (i32.const 0)
      (i32.const 0)))

  (func (export "load8_u") (param i32) (result i32)
    (i32.load8_u (local.get 0)))
)
