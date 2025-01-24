(module
  (memory (export "memory") 1 1)
  (global $var (mut i32) (i32.const 0x800))

  (func $func1
    i32.const 0
    i32.const 0
    i32.eq
    (if
      (then
        global.get $var
        i32.const 0
        i32.store8
        return
      )
    )

    ;; write the value
    global.get $var
    return
  )
)
