(module
  (global $a i32 (i32.const 255))
  (global $b i32 (i32.const 255))

  (func $main (export "user_entrypoint") (param $c i32) (result i32)
    global.get $a
    global.get $b
    i32.add
    local.get $c
    i32.add
  )
)
