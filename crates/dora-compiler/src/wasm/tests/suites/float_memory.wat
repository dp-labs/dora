(module
  (memory (data "\00\00\00\00\00\00\00\f4\7f"))

  (func (export "f64.load") (result f64) (f64.load (i32.const 1)))
  (func (export "i64.load") (result i64) (i64.load (i32.const 1)))
  (func (export "f64.store") (f64.store (i32.const 1) (f64.const nan:0x4000000000000)))
  (func (export "i64.store") (i64.store (i32.const 1) (i64.const 0x7ff4000000000000)))
  (func (export "reset") (i64.store (i32.const 1) (i64.const 0)))
)
