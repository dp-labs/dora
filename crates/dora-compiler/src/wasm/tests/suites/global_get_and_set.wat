(module
  (global $a i32 (i32.const -2))
  (global $b i64 (i64.const -5))
  (global $x (mut i32) (i32.const -12))
  (global $y (mut i64) (i64.const -15))

  (func (export "get-a") (result i32) (global.get $a))
  (func (export "get-b") (result i64) (global.get $b))
  (func (export "set-x") (param i32) (global.set $x (local.get 0)))
  (func (export "set-y") (param i64) (global.set $y (local.get 0)))
)
