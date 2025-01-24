(module
  ;; Define a function type: takes an i32 parameter and returns an i32 value
  (type $fib-type (func (param i32) (result i32)))

  ;; Recursive Fibonacci function
  (func $fib-recursive (type $fib-type)
    (param $n i32)       ;; Parameter n
    (result i32)         ;; Return type

    ;; Base cases
    (if (i32.le_u (local.get $n) (i32.const 1))
      (then
        (return (local.get $n)) ;; Return n if n <= 1
      )
    )

    ;; Recursive case: fib(n-1) + fib(n-2)
    (i32.add
      (call $fib-recursive (i32.sub (local.get $n) (i32.const 1))) ;; fib(n-1)
      (call $fib-recursive (i32.sub (local.get $n) (i32.const 2))) ;; fib(n-2)
    )
  )

  ;; Iterative Fibonacci function
  (func $fib-iterative (type $fib-type)
    (param $n i32)       ;; Parameter n
    (result i32)         ;; Return type
    (local $a i32)       ;; Local variable a
    (local $b i32)       ;; Local variable b
    (local $i i32)       ;; Local variable i
    (local $tmp i32)     ;; Local variable tmp

    ;; Initialize local variables
    (local.set $a (i32.const 0))  ;; a = 0
    (local.set $b (i32.const 1))  ;; b = 1
    (local.set $i (i32.const 0))  ;; i = 0

    ;; If n == 0, return 0 directly
    (if (i32.eq (local.get $n) (i32.const 0))
      (then
        (return (i32.const 0))
      )
    )

    ;; If n == 1, return 1 directly
    (if (i32.eq (local.get $n) (i32.const 1))
      (then
        (return (i32.const 1))
      )
    )

    ;; Loop to compute the Fibonacci sequence
    (loop $loop
      ;; Compute the next Fibonacci number
      (local.set $tmp (local.get $b))          ;; tmp = b
      (local.set $b (i32.add (local.get $a) (local.get $b))) ;; b = a + b
      (local.set $a (local.get $tmp))          ;; a = tmp

      ;; Increment the counter
      (local.set $i (i32.add (local.get $i) (i32.const 1))) ;; i++

      ;; If i < n - 1, continue the loop
      (br_if $loop (i32.lt_u (local.get $i) (i32.sub (local.get $n) (i32.const 1))))
    )

    ;; Return the result
    (local.get $b)
  )

  ;; Export the recursive Fibonacci function as "fib_recursive"
  (export "fib_recursive" (func $fib-recursive))

  ;; Export the iterative Fibonacci function as "fib_iterative"
  (export "fib_iterative" (func $fib-iterative))
)
