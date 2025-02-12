;; extern crate alloc;
;; 
;; #[no_mangle]
;; pub extern "C" fn vec_sum(data: i32) -> i32 {
;;     let data = alloc::vec![data, 1, 2, 3];
;;     let mut sum = 0;
;;     for i in data {
;;         sum += i;
;;     }
;;     sum
;; }
;; 
;; #[no_mangle]
;; pub extern "C" fn vec_product(data: i32) -> i32 {
;;     let data = alloc::vec![data, 1, 2, 3];
;;     let mut product = 1;
;;     for i in data {
;;         product *= i;
;;     }
;;     product
;; }
;; 
;; #[no_mangle]
;; pub extern "C" fn vec_max(data: i32) -> i32 {
;;     let data = alloc::vec![data, 1, 2, 3];
;;     let mut max = i32::MIN;
;;     for i in data {
;;         if i > max {
;;             max = i;
;;         }
;;     }
;;     max
;; }
;; 
;; #[no_mangle]
;; pub extern "C" fn vec_min(data: i32) -> i32 {
;;     let data = alloc::vec![data, 1, 2, 3];
;;     let mut min = i32::MAX;
;;     for i in data {
;;         if i < min {
;;             min = i;
;;         }
;;     }
;;     min
;; }
;; 
;; #[no_mangle]
;; pub extern "C" fn vec_average(data: i32) -> f32 {
;;     let data = alloc::vec![data, 1, 2, 3];
;;     let sum: i32 = data.iter().sum();
;;     sum as f32 / data.len() as f32
;; }
;; 
;; #[no_mangle]
;; pub extern "C" fn vec_reverse(data: i32) -> i32 {
;;     let mut data = alloc::vec![data, 1, 2, 3];
;;     data.reverse();
;;     data[0]
;; }
;; 
;; #[no_mangle]
;; pub extern "C" fn vec_clone(data: i32) -> i32 {
;;     let data = alloc::vec![data, 1, 2, 3];
;;     let cloned_data = data.clone();
;;     cloned_data[0]
;; }
;; 
;; #[no_mangle]
;; pub extern "C" fn vec_drain(data: i32) -> i32 {
;;     let mut data = alloc::vec![data, 1, 2, 3];
;;     let drained_data: Vec<i32> = data.drain(..2).collect();
;;     drained_data[0]
;; }
;; 
;; #[no_mangle]
;; pub extern "C" fn vec_sort(data: i32) -> i32 {
;;     let mut data = alloc::vec![data, 1, 2, 3];
;;     data.sort();
;;     data[0]
;; }
;; 
;; #[no_mangle]
;; pub extern "C" fn string_format() -> i32 {
;;     let count = 3;
;;     let data = alloc::format!("{}", count);
;;     data.len() as i32
;; }
(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (param i32 i32 i32) (result i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func (param i32 i32 i32)))
  (type (;4;) (func (param i32)))
  (type (;5;) (func (param i32 i32 i32 i32 i32)))
  (type (;6;) (func (param i32 i32 i32 i32)))
  (type (;7;) (func (param i32) (result i32)))
  (type (;8;) (func (param i32) (result f32)))
  (type (;9;) (func (result i32)))
  (type (;10;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;11;) (func))
  (type (;12;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;13;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (type (;14;) (func (param i64 i32 i32) (result i32)))
  (table (;0;) 26 26 funcref)
  (memory (;0;) 17)
  (global (;0;) (mut i32) i32.const 1048576)
  (global (;1;) i32 i32.const 1050017)
  (global (;2;) i32 i32.const 1050032)
  (export "memory" (memory 0))
  (export "vec_sum" (func 7))
  (export "vec_product" (func 8))
  (export "vec_max" (func 9))
  (export "vec_min" (func 10))
  (export "vec_average" (func 11))
  (export "vec_reverse" (func 12))
  (export "vec_clone" (func 13))
  (export "vec_drain" (func 14))
  (export "vec_sort" (func 15))
  (export "string_format" (func 16))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) func 84 74 40 25 30 28 24 21 22 51 48 49 50 26 47 45 46 27 58 68 69 57 59 78 77)
  (func (;0;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    i32.load offset=4
    local.tee 3
    local.get 1
    i32.load
    local.tee 4
    i32.sub
    local.set 5
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 3
          local.get 4
          i32.ne
          br_if 0 (;@3;)
          i32.const 4
          local.set 3
          br 1 (;@2;)
        end
        block ;; label = @3
          local.get 5
          i32.const 2147483644
          i32.le_u
          br_if 0 (;@3;)
          i32.const 0
          local.set 4
          br 2 (;@1;)
        end
        i32.const 0
        i32.load8_u offset=1049537
        drop
        i32.const 4
        local.set 4
        local.get 5
        i32.const 4
        call 17
        local.tee 3
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 2
      i32.const 4
      i32.add
      i32.const 8
      i32.add
      local.tee 4
      i32.const 0
      i32.store
      local.get 2
      local.get 3
      i32.store offset=8
      local.get 2
      local.get 5
      i32.const 2
      i32.shr_u
      i32.store offset=4
      local.get 2
      i32.const 4
      i32.add
      local.get 1
      call 1
      local.get 0
      i32.const 8
      i32.add
      local.get 4
      i32.load
      i32.store
      local.get 0
      local.get 2
      i64.load offset=4 align=4
      i64.store align=4
      local.get 2
      i32.const 16
      i32.add
      global.set 0
      return
    end
    local.get 4
    local.get 5
    call 62
    unreachable
  )
  (func (;1;) (type 0) (param i32 i32)
    (local i32 i32 i32 i64 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    block ;; label = @1
      local.get 1
      i32.load offset=4
      local.get 1
      i32.load
      i32.sub
      i32.const 2
      i32.shr_u
      local.tee 3
      local.get 0
      i32.load
      local.get 0
      i32.load offset=8
      local.tee 4
      i32.sub
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      local.get 4
      local.get 3
      call 2
      local.get 0
      i32.load offset=8
      local.set 4
    end
    local.get 0
    i32.load offset=4
    local.set 3
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i32.load
    i32.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    local.tee 5
    i64.store offset=8
    block ;; label = @1
      local.get 5
      i32.wrap_i64
      local.tee 1
      local.get 2
      i32.load offset=12
      local.tee 6
      i32.eq
      br_if 0 (;@1;)
      local.get 3
      local.get 4
      i32.const 2
      i32.shl
      i32.add
      local.set 3
      loop ;; label = @2
        local.get 3
        local.get 1
        i32.load
        i32.store
        local.get 3
        i32.const 4
        i32.add
        local.set 3
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        local.get 1
        i32.const 4
        i32.add
        local.tee 1
        local.get 6
        i32.ne
        br_if 0 (;@2;)
      end
      local.get 2
      local.get 6
      i32.store offset=8
    end
    local.get 0
    local.get 4
    i32.store offset=8
    local.get 2
    i32.const 8
    i32.add
    call 3
    local.get 2
    i32.const 32
    i32.add
    global.set 0
  )
  (func (;2;) (type 3) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 8
    i32.add
    local.get 0
    local.get 1
    local.get 2
    call 5
    block ;; label = @1
      local.get 3
      i32.load offset=8
      local.tee 2
      i32.const -2147483647
      i32.eq
      br_if 0 (;@1;)
      local.get 2
      local.get 3
      i32.load offset=12
      call 62
      unreachable
    end
    local.get 3
    i32.const 16
    i32.add
    global.set 0
  )
  (func (;3;) (type 4) (param i32)
    (local i32 i32 i32 i32)
    local.get 0
    i32.load offset=4
    local.set 1
    local.get 0
    i32.load
    local.set 2
    local.get 0
    i64.const 17179869188
    i64.store align=4
    local.get 0
    i32.load offset=16
    local.set 3
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          local.get 2
          i32.eq
          br_if 0 (;@3;)
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          i32.load offset=8
          local.tee 2
          i32.const 8
          i32.add
          local.set 1
          local.get 0
          i32.load offset=12
          local.tee 4
          local.get 2
          i32.load offset=8
          local.tee 0
          i32.eq
          br_if 1 (;@2;)
          local.get 2
          i32.load offset=4
          local.tee 2
          local.get 0
          i32.const 2
          i32.shl
          i32.add
          local.get 2
          local.get 4
          i32.const 2
          i32.shl
          i32.add
          local.get 3
          i32.const 2
          i32.shl
          call 87
          drop
          br 1 (;@2;)
        end
        local.get 3
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=8
        local.tee 2
        i32.const 8
        i32.add
        local.set 1
        local.get 0
        i32.load offset=12
        local.tee 4
        local.get 2
        i32.load offset=8
        local.tee 0
        i32.eq
        br_if 0 (;@2;)
        local.get 2
        i32.load offset=4
        local.tee 2
        local.get 0
        i32.const 2
        i32.shl
        i32.add
        local.get 2
        local.get 4
        i32.const 2
        i32.shl
        i32.add
        local.get 3
        i32.const 2
        i32.shl
        call 87
        drop
      end
      local.get 1
      local.get 0
      local.get 3
      i32.add
      i32.store
    end
  )
  (func (;4;) (type 5) (param i32 i32 i32 i32 i32)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 1
                i32.eqz
                br_if 0 (;@6;)
                local.get 2
                i32.const -1
                i32.le_s
                br_if 1 (;@5;)
                block ;; label = @7
                  block ;; label = @8
                    local.get 3
                    i32.load offset=4
                    i32.eqz
                    br_if 0 (;@8;)
                    block ;; label = @9
                      local.get 3
                      i32.load offset=8
                      local.tee 5
                      br_if 0 (;@9;)
                      local.get 2
                      i32.eqz
                      br_if 5 (;@4;)
                      i32.const 0
                      i32.load8_u offset=1049537
                      drop
                      br 2 (;@7;)
                    end
                    local.get 3
                    i32.load
                    local.get 5
                    local.get 1
                    local.get 2
                    call 19
                    local.set 3
                    br 5 (;@3;)
                  end
                  local.get 2
                  i32.eqz
                  br_if 3 (;@4;)
                  i32.const 0
                  i32.load8_u offset=1049537
                  drop
                end
                local.get 2
                local.get 1
                call 17
                local.set 3
                br 3 (;@3;)
              end
              local.get 0
              i32.const 0
              i32.store offset=4
              br 3 (;@2;)
            end
            local.get 0
            i32.const 0
            i32.store offset=4
            br 2 (;@2;)
          end
          local.get 1
          local.set 3
        end
        block ;; label = @3
          local.get 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          local.get 2
          i32.store offset=8
          local.get 0
          local.get 3
          i32.store offset=4
          i32.const 0
          local.set 2
          br 2 (;@1;)
        end
        local.get 0
        local.get 2
        i32.store offset=8
        local.get 0
        local.get 1
        i32.store offset=4
      end
      i32.const 1
      local.set 2
    end
    local.get 0
    local.get 2
    i32.store
  )
  (func (;5;) (type 6) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 4
    global.set 0
    block ;; label = @1
      block ;; label = @2
        local.get 2
        local.get 3
        i32.add
        local.tee 3
        local.get 2
        i32.ge_u
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      i32.const 4
      local.set 5
      local.get 1
      i32.load
      local.tee 6
      i32.const 1
      i32.shl
      local.tee 2
      local.get 3
      local.get 2
      local.get 3
      i32.gt_u
      select
      local.tee 2
      i32.const 4
      local.get 2
      i32.const 4
      i32.gt_u
      select
      local.tee 7
      i32.const 2
      i32.shl
      local.set 3
      local.get 2
      i32.const 536870912
      i32.lt_u
      i32.const 2
      i32.shl
      local.set 2
      block ;; label = @2
        block ;; label = @3
          local.get 6
          br_if 0 (;@3;)
          i32.const 0
          local.set 5
          br 1 (;@2;)
        end
        local.get 4
        local.get 6
        i32.const 2
        i32.shl
        i32.store offset=28
        local.get 4
        local.get 1
        i32.load offset=4
        i32.store offset=20
      end
      local.get 4
      local.get 5
      i32.store offset=24
      local.get 4
      i32.const 8
      i32.add
      local.get 2
      local.get 3
      local.get 4
      i32.const 20
      i32.add
      local.get 4
      call 4
      block ;; label = @2
        local.get 4
        i32.load offset=8
        br_if 0 (;@2;)
        local.get 4
        i32.load offset=12
        local.set 2
        local.get 1
        local.get 7
        i32.store
        local.get 1
        local.get 2
        i32.store offset=4
        i32.const -2147483647
        local.set 2
        br 1 (;@1;)
      end
      local.get 4
      i32.load offset=16
      local.set 1
      local.get 4
      i32.load offset=12
      local.set 2
    end
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 0
    local.get 2
    i32.store
    local.get 4
    i32.const 32
    i32.add
    global.set 0
  )
  (func (;6;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32)
    local.get 1
    i32.load offset=4
    local.set 2
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.load offset=8
        local.tee 1
        br_if 0 (;@2;)
        i32.const 4
        local.set 3
        i32.const 0
        local.set 4
        br 1 (;@1;)
      end
      local.get 1
      i32.const 2
      i32.shl
      local.set 4
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.const 536870911
          i32.le_u
          br_if 0 (;@3;)
          i32.const 0
          local.set 5
          br 1 (;@2;)
        end
        i32.const 0
        i32.load8_u offset=1049537
        drop
        i32.const 4
        local.set 5
        local.get 4
        i32.const 4
        call 17
        local.tee 3
        br_if 1 (;@1;)
      end
      local.get 5
      local.get 4
      call 62
      unreachable
    end
    local.get 3
    local.get 2
    local.get 4
    call 88
    local.set 4
    local.get 0
    local.get 1
    i32.store offset=8
    local.get 0
    local.get 4
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
  )
  (func (;7;) (type 7) (param i32) (result i32)
    (local i32 i32)
    i32.const 0
    local.set 1
    i32.const 0
    i32.load8_u offset=1049537
    drop
    block ;; label = @1
      i32.const 16
      i32.const 4
      call 17
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.const 3
      i32.store offset=12
      local.get 2
      i64.const 8589934593
      i64.store offset=4 align=4
      local.get 2
      local.get 0
      i32.store
      i32.const 0
      local.set 0
      loop ;; label = @2
        local.get 2
        local.get 0
        i32.add
        i32.load
        local.get 1
        i32.add
        local.set 1
        local.get 0
        i32.const 4
        i32.add
        local.tee 0
        i32.const 16
        i32.ne
        br_if 0 (;@2;)
      end
      local.get 2
      i32.const 16
      i32.const 4
      call 18
      local.get 1
      return
    end
    i32.const 4
    i32.const 16
    call 65
    unreachable
  )
  (func (;8;) (type 7) (param i32) (result i32)
    (local i32 i32)
    i32.const 0
    local.set 1
    i32.const 0
    i32.load8_u offset=1049537
    drop
    block ;; label = @1
      i32.const 16
      i32.const 4
      call 17
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.const 3
      i32.store offset=12
      local.get 2
      i64.const 8589934593
      i64.store offset=4 align=4
      local.get 2
      local.get 0
      i32.store
      i32.const 1
      local.set 0
      loop ;; label = @2
        local.get 2
        local.get 1
        i32.add
        i32.load
        local.get 0
        i32.mul
        local.set 0
        local.get 1
        i32.const 4
        i32.add
        local.tee 1
        i32.const 16
        i32.ne
        br_if 0 (;@2;)
      end
      local.get 2
      i32.const 16
      i32.const 4
      call 18
      local.get 0
      return
    end
    i32.const 4
    i32.const 16
    call 65
    unreachable
  )
  (func (;9;) (type 7) (param i32) (result i32)
    (local i32 i32 i32)
    i32.const 0
    local.set 1
    i32.const 0
    i32.load8_u offset=1049537
    drop
    block ;; label = @1
      i32.const 16
      i32.const 4
      call 17
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.const 3
      i32.store offset=12
      local.get 2
      i64.const 8589934593
      i64.store offset=4 align=4
      local.get 2
      local.get 0
      i32.store
      i32.const -2147483648
      local.set 0
      loop ;; label = @2
        local.get 2
        local.get 1
        i32.add
        i32.load
        local.tee 3
        local.get 0
        local.get 3
        local.get 0
        i32.gt_s
        select
        local.set 0
        local.get 1
        i32.const 4
        i32.add
        local.tee 1
        i32.const 16
        i32.ne
        br_if 0 (;@2;)
      end
      local.get 2
      i32.const 16
      i32.const 4
      call 18
      local.get 0
      return
    end
    i32.const 4
    i32.const 16
    call 65
    unreachable
  )
  (func (;10;) (type 7) (param i32) (result i32)
    (local i32 i32 i32)
    i32.const 0
    local.set 1
    i32.const 0
    i32.load8_u offset=1049537
    drop
    block ;; label = @1
      i32.const 16
      i32.const 4
      call 17
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.const 3
      i32.store offset=12
      local.get 2
      i64.const 8589934593
      i64.store offset=4 align=4
      local.get 2
      local.get 0
      i32.store
      i32.const 2147483647
      local.set 0
      loop ;; label = @2
        local.get 2
        local.get 1
        i32.add
        i32.load
        local.tee 3
        local.get 0
        local.get 3
        local.get 0
        i32.lt_s
        select
        local.set 0
        local.get 1
        i32.const 4
        i32.add
        local.tee 1
        i32.const 16
        i32.ne
        br_if 0 (;@2;)
      end
      local.get 2
      i32.const 16
      i32.const 4
      call 18
      local.get 0
      return
    end
    i32.const 4
    i32.const 16
    call 65
    unreachable
  )
  (func (;11;) (type 8) (param i32) (result f32)
    (local i32 i32)
    i32.const 0
    local.set 1
    i32.const 0
    i32.load8_u offset=1049537
    drop
    block ;; label = @1
      i32.const 16
      i32.const 4
      call 17
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.const 3
      i32.store offset=12
      local.get 2
      i64.const 8589934593
      i64.store offset=4 align=4
      local.get 2
      local.get 0
      i32.store
      i32.const 0
      local.set 0
      loop ;; label = @2
        local.get 2
        local.get 1
        i32.add
        i32.load
        local.get 0
        i32.add
        local.set 0
        local.get 1
        i32.const 4
        i32.add
        local.tee 1
        i32.const 16
        i32.ne
        br_if 0 (;@2;)
      end
      local.get 2
      i32.const 16
      i32.const 4
      call 18
      local.get 0
      f32.convert_i32_s
      f32.const 0x1p-2 (;=0.25;)
      f32.mul
      return
    end
    i32.const 4
    i32.const 16
    call 65
    unreachable
  )
  (func (;12;) (type 7) (param i32) (result i32)
    (local i32 i32 i32 i32)
    i32.const 0
    local.set 1
    i32.const 0
    i32.load8_u offset=1049537
    drop
    block ;; label = @1
      i32.const 16
      i32.const 4
      call 17
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i64.const 8589934593
      i64.store offset=4 align=4
      local.get 2
      local.get 0
      i32.store
      local.get 2
      i32.const 12
      i32.add
      local.tee 0
      i32.const 3
      i32.store
      loop ;; label = @2
        local.get 2
        local.get 1
        i32.add
        local.tee 3
        i32.load
        local.set 4
        local.get 3
        local.get 0
        i32.load
        i32.store
        local.get 0
        local.get 4
        i32.store
        local.get 0
        i32.const -4
        i32.add
        local.set 0
        local.get 1
        i32.const 4
        i32.add
        local.tee 1
        i32.const 8
        i32.ne
        br_if 0 (;@2;)
      end
      local.get 2
      i32.load
      local.set 0
      local.get 2
      i32.const 16
      i32.const 4
      call 18
      local.get 0
      return
    end
    i32.const 4
    i32.const 16
    call 65
    unreachable
  )
  (func (;13;) (type 7) (param i32) (result i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    i32.const 0
    i32.load8_u offset=1049537
    drop
    block ;; label = @1
      block ;; label = @2
        i32.const 16
        i32.const 4
        call 17
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        i32.const 3
        i32.store offset=12
        local.get 2
        i64.const 8589934593
        i64.store offset=4 align=4
        local.get 2
        local.get 0
        i32.store
        local.get 1
        i32.const 4
        i32.store offset=16
        local.get 1
        local.get 2
        i32.store offset=12
        local.get 1
        i32.const 4
        i32.store offset=8
        local.get 1
        i32.const 20
        i32.add
        local.get 1
        i32.const 8
        i32.add
        call 6
        local.get 1
        i32.load offset=28
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        i32.load offset=24
        local.tee 3
        i32.load
        local.set 0
        block ;; label = @3
          local.get 1
          i32.load offset=20
          local.tee 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          local.get 4
          i32.const 2
          i32.shl
          i32.const 4
          call 18
        end
        local.get 2
        i32.const 16
        i32.const 4
        call 18
        local.get 1
        i32.const 32
        i32.add
        global.set 0
        local.get 0
        return
      end
      i32.const 4
      i32.const 16
      call 65
      unreachable
    end
    i32.const 0
    i32.const 0
    i32.const 1048588
    call 71
    unreachable
  )
  (func (;14;) (type 7) (param i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 1
    global.set 0
    i32.const 0
    i32.load8_u offset=1049537
    drop
    block ;; label = @1
      block ;; label = @2
        i32.const 16
        i32.const 4
        call 17
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        i32.const 3
        i32.store offset=12
        local.get 2
        i64.const 8589934593
        i64.store offset=4 align=4
        local.get 2
        local.get 0
        i32.store
        local.get 1
        local.get 2
        i32.store offset=8
        local.get 1
        i32.const 4
        i32.store offset=4
        local.get 1
        i32.const 0
        i32.store offset=12
        local.get 1
        i64.const 8589934594
        i64.store offset=40 align=4
        local.get 1
        local.get 2
        i32.const 8
        i32.add
        i32.store offset=32
        local.get 1
        local.get 2
        i32.store offset=28
        local.get 1
        local.get 1
        i32.const 4
        i32.add
        i32.store offset=36
        local.get 1
        i32.const 16
        i32.add
        local.get 1
        i32.const 28
        i32.add
        call 0
        local.get 1
        i32.load offset=24
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        i32.load offset=20
        local.tee 0
        i32.load
        local.set 2
        block ;; label = @3
          local.get 1
          i32.load offset=16
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          local.get 3
          i32.const 2
          i32.shl
          i32.const 4
          call 18
        end
        block ;; label = @3
          local.get 1
          i32.load offset=4
          local.tee 0
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          i32.load offset=8
          local.get 0
          i32.const 2
          i32.shl
          i32.const 4
          call 18
        end
        local.get 1
        i32.const 48
        i32.add
        global.set 0
        local.get 2
        return
      end
      i32.const 4
      i32.const 16
      call 65
      unreachable
    end
    i32.const 0
    i32.const 0
    i32.const 1048604
    call 71
    unreachable
  )
  (func (;15;) (type 7) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    i32.const 0
    local.set 1
    i32.const 0
    i32.load8_u offset=1049537
    drop
    block ;; label = @1
      i32.const 16
      i32.const 4
      call 17
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.const 3
      i32.store offset=12
      local.get 2
      i64.const 8589934593
      i64.store offset=4 align=4
      local.get 2
      local.get 0
      i32.store
      i32.const 4
      local.set 3
      loop ;; label = @2
        block ;; label = @3
          local.get 2
          local.get 3
          i32.add
          local.tee 0
          i32.load
          local.tee 4
          local.get 0
          i32.const -4
          i32.add
          i32.load
          local.tee 5
          i32.ge_s
          br_if 0 (;@3;)
          local.get 1
          local.set 0
          block ;; label = @4
            loop ;; label = @5
              local.get 2
              local.get 0
              i32.add
              i32.const 4
              i32.add
              local.get 5
              i32.store
              block ;; label = @6
                local.get 0
                br_if 0 (;@6;)
                local.get 2
                local.set 0
                br 2 (;@4;)
              end
              local.get 4
              local.get 0
              i32.const -4
              i32.add
              local.tee 0
              local.get 2
              i32.add
              local.tee 6
              i32.load
              local.tee 5
              i32.lt_s
              br_if 0 (;@5;)
            end
            local.get 6
            i32.const 4
            i32.add
            local.set 0
          end
          local.get 0
          local.get 4
          i32.store
        end
        local.get 1
        i32.const 4
        i32.add
        local.set 1
        local.get 3
        i32.const 4
        i32.add
        local.tee 3
        i32.const 16
        i32.ne
        br_if 0 (;@2;)
      end
      local.get 2
      i32.load
      local.set 0
      local.get 2
      i32.const 16
      i32.const 4
      call 18
      local.get 0
      return
    end
    i32.const 4
    i32.const 16
    call 65
    unreachable
  )
  (func (;16;) (type 9) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    i32.const 3
    i32.store
    local.get 0
    i32.const 1
    i32.store offset=20
    local.get 0
    i32.const 1048620
    i32.store offset=16
    local.get 0
    i64.const 1
    i64.store offset=28 align=4
    local.get 0
    i32.const 1
    i32.store offset=44
    local.get 0
    local.get 0
    i32.const 40
    i32.add
    i32.store offset=24
    local.get 0
    local.get 0
    i32.store offset=40
    local.get 0
    i32.const 4
    i32.add
    local.get 0
    i32.const 16
    i32.add
    call 66
    local.get 0
    i32.load offset=12
    local.set 1
    block ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=8
      local.get 2
      i32.const 1
      call 18
    end
    local.get 0
    i32.const 48
    i32.add
    global.set 0
    local.get 1
  )
  (func (;17;) (type 2) (param i32 i32) (result i32)
    (local i32)
    local.get 0
    local.get 1
    call 41
    local.set 2
    local.get 2
    return
  )
  (func (;18;) (type 3) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    call 42
    return
  )
  (func (;19;) (type 10) (param i32 i32 i32 i32) (result i32)
    (local i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call 43
    local.set 4
    local.get 4
    return
  )
  (func (;20;) (type 0) (param i32 i32)
    local.get 0
    local.get 1
    call 54
    return
  )
  (func (;21;) (type 0) (param i32 i32)
    local.get 0
    i64.const 4854689474455388916
    i64.store offset=8
    local.get 0
    i64.const -1846477596472271460
    i64.store
  )
  (func (;22;) (type 0) (param i32 i32)
    local.get 0
    i64.const 7199936582794304877
    i64.store offset=8
    local.get 0
    i64.const -5076933981314334344
    i64.store
  )
  (func (;23;) (type 3) (param i32 i32 i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    block ;; label = @1
      local.get 1
      local.get 2
      i32.add
      local.tee 2
      local.get 1
      i32.ge_u
      br_if 0 (;@1;)
      i32.const 0
      i32.const 0
      call 62
      unreachable
    end
    i32.const 1
    local.set 4
    local.get 0
    i32.load
    local.tee 5
    i32.const 1
    i32.shl
    local.tee 1
    local.get 2
    local.get 1
    local.get 2
    i32.gt_u
    select
    local.tee 1
    i32.const 8
    local.get 1
    i32.const 8
    i32.gt_u
    select
    local.tee 1
    i32.const -1
    i32.xor
    i32.const 31
    i32.shr_u
    local.set 2
    block ;; label = @1
      block ;; label = @2
        local.get 5
        br_if 0 (;@2;)
        i32.const 0
        local.set 4
        br 1 (;@1;)
      end
      local.get 3
      local.get 5
      i32.store offset=28
      local.get 3
      local.get 0
      i32.load offset=4
      i32.store offset=20
    end
    local.get 3
    local.get 4
    i32.store offset=24
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    local.get 1
    local.get 3
    i32.const 20
    i32.add
    call 31
    block ;; label = @1
      local.get 3
      i32.load offset=8
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=12
      local.get 3
      i32.load offset=16
      call 62
      unreachable
    end
    local.get 3
    i32.load offset=12
    local.set 2
    local.get 0
    local.get 1
    i32.store
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 3
    i32.const 32
    i32.add
    global.set 0
  )
  (func (;24;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.const 1048628
    local.get 1
    call 75
  )
  (func (;25;) (type 4) (param i32)
    (local i32)
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 1
      i32.const 1
      call 18
    end
  )
  (func (;26;) (type 4) (param i32)
    (local i32)
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.const -2147483648
      i32.or
      i32.const -2147483648
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 1
      i32.const 1
      call 18
    end
  )
  (func (;27;) (type 0) (param i32 i32)
    local.get 0
    i32.const 0
    i32.store
  )
  (func (;28;) (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.const 128
            i32.lt_u
            br_if 0 (;@4;)
            local.get 2
            i32.const 0
            i32.store offset=12
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            block ;; label = @5
              local.get 1
              i32.const 65536
              i32.ge_u
              br_if 0 (;@5;)
              local.get 2
              local.get 1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 224
              i32.or
              i32.store8 offset=12
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 3
              local.set 1
              br 3 (;@2;)
            end
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=15
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            local.get 2
            local.get 1
            i32.const 18
            i32.shr_u
            i32.const 7
            i32.and
            i32.const 240
            i32.or
            i32.store8 offset=12
            i32.const 4
            local.set 1
            br 2 (;@2;)
          end
          block ;; label = @4
            local.get 0
            i32.load offset=8
            local.tee 3
            local.get 0
            i32.load
            i32.ne
            br_if 0 (;@4;)
            local.get 0
            call 29
          end
          local.get 0
          local.get 3
          i32.const 1
          i32.add
          i32.store offset=8
          local.get 0
          i32.load offset=4
          local.get 3
          i32.add
          local.get 1
          i32.store8
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        local.set 1
      end
      block ;; label = @2
        local.get 0
        i32.load
        local.get 0
        i32.load offset=8
        local.tee 3
        i32.sub
        local.get 1
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.get 3
        local.get 1
        call 23
        local.get 0
        i32.load offset=8
        local.set 3
      end
      local.get 0
      i32.load offset=4
      local.get 3
      i32.add
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      call 88
      drop
      local.get 0
      local.get 3
      local.get 1
      i32.add
      i32.store offset=8
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    i32.const 0
  )
  (func (;29;) (type 4) (param i32)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 2
      i32.const -1
      i32.ne
      br_if 0 (;@1;)
      i32.const 0
      i32.const 0
      call 62
      unreachable
    end
    i32.const 1
    local.set 3
    local.get 2
    i32.const 1
    i32.shl
    local.tee 4
    local.get 2
    i32.const 1
    i32.add
    local.tee 5
    local.get 4
    local.get 5
    i32.gt_u
    select
    local.tee 4
    i32.const 8
    local.get 4
    i32.const 8
    i32.gt_u
    select
    local.tee 4
    i32.const -1
    i32.xor
    i32.const 31
    i32.shr_u
    local.set 5
    block ;; label = @1
      block ;; label = @2
        local.get 2
        br_if 0 (;@2;)
        i32.const 0
        local.set 3
        br 1 (;@1;)
      end
      local.get 1
      local.get 2
      i32.store offset=28
      local.get 1
      local.get 0
      i32.load offset=4
      i32.store offset=20
    end
    local.get 1
    local.get 3
    i32.store offset=24
    local.get 1
    i32.const 8
    i32.add
    local.get 5
    local.get 4
    local.get 1
    i32.const 20
    i32.add
    call 31
    block ;; label = @1
      local.get 1
      i32.load offset=8
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=12
      local.get 1
      i32.load offset=16
      call 62
      unreachable
    end
    local.get 1
    i32.load offset=12
    local.set 2
    local.get 0
    local.get 4
    i32.store
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 1
    i32.const 32
    i32.add
    global.set 0
  )
  (func (;30;) (type 1) (param i32 i32 i32) (result i32)
    (local i32)
    block ;; label = @1
      local.get 0
      i32.load
      local.get 0
      i32.load offset=8
      local.tee 3
      i32.sub
      local.get 2
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      local.get 3
      local.get 2
      call 23
      local.get 0
      i32.load offset=8
      local.set 3
    end
    local.get 0
    i32.load offset=4
    local.get 3
    i32.add
    local.get 1
    local.get 2
    call 88
    drop
    local.get 0
    local.get 3
    local.get 2
    i32.add
    i32.store offset=8
    i32.const 0
  )
  (func (;31;) (type 6) (param i32 i32 i32 i32)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          i32.const -1
          i32.le_s
          br_if 1 (;@2;)
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 3
                i32.load offset=4
                i32.eqz
                br_if 0 (;@6;)
                block ;; label = @7
                  local.get 3
                  i32.load offset=8
                  local.tee 4
                  br_if 0 (;@7;)
                  block ;; label = @8
                    local.get 2
                    br_if 0 (;@8;)
                    local.get 1
                    local.set 3
                    br 4 (;@4;)
                  end
                  i32.const 0
                  i32.load8_u offset=1049537
                  drop
                  br 2 (;@5;)
                end
                local.get 3
                i32.load
                local.get 4
                local.get 1
                local.get 2
                call 19
                local.set 3
                br 2 (;@4;)
              end
              block ;; label = @6
                local.get 2
                br_if 0 (;@6;)
                local.get 1
                local.set 3
                br 2 (;@4;)
              end
              i32.const 0
              i32.load8_u offset=1049537
              drop
            end
            local.get 2
            local.get 1
            call 17
            local.set 3
          end
          block ;; label = @4
            local.get 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            local.get 2
            i32.store offset=8
            local.get 0
            local.get 3
            i32.store offset=4
            local.get 0
            i32.const 0
            i32.store
            return
          end
          local.get 0
          local.get 2
          i32.store offset=8
          local.get 0
          local.get 1
          i32.store offset=4
          br 2 (;@1;)
        end
        local.get 0
        i32.const 0
        i32.store offset=4
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      i32.store offset=4
    end
    local.get 0
    i32.const 1
    i32.store
  )
  (func (;32;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32)
    local.get 0
    i32.load offset=12
    local.set 2
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.const 256
          i32.lt_u
          br_if 0 (;@3;)
          local.get 0
          i32.load offset=24
          local.set 3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 2
                local.get 0
                i32.ne
                br_if 0 (;@6;)
                local.get 0
                i32.const 20
                i32.const 16
                local.get 0
                i32.load offset=20
                local.tee 2
                select
                i32.add
                i32.load
                local.tee 1
                br_if 1 (;@5;)
                i32.const 0
                local.set 2
                br 2 (;@4;)
              end
              local.get 0
              i32.load offset=8
              local.tee 1
              local.get 2
              i32.store offset=12
              local.get 2
              local.get 1
              i32.store offset=8
              br 1 (;@4;)
            end
            local.get 0
            i32.const 20
            i32.add
            local.get 0
            i32.const 16
            i32.add
            local.get 2
            select
            local.set 4
            loop ;; label = @5
              local.get 4
              local.set 5
              local.get 1
              local.tee 2
              i32.const 20
              i32.add
              local.get 2
              i32.const 16
              i32.add
              local.get 2
              i32.load offset=20
              local.tee 1
              select
              local.set 4
              local.get 2
              i32.const 20
              i32.const 16
              local.get 1
              select
              i32.add
              i32.load
              local.tee 1
              br_if 0 (;@5;)
            end
            local.get 5
            i32.const 0
            i32.store
          end
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          block ;; label = @4
            local.get 0
            i32.load offset=28
            i32.const 2
            i32.shl
            i32.const 1049560
            i32.add
            local.tee 1
            i32.load
            local.get 0
            i32.eq
            br_if 0 (;@4;)
            local.get 3
            i32.const 16
            i32.const 20
            local.get 3
            i32.load offset=16
            local.get 0
            i32.eq
            select
            i32.add
            local.get 2
            i32.store
            local.get 2
            i32.eqz
            br_if 3 (;@1;)
            br 2 (;@2;)
          end
          local.get 1
          local.get 2
          i32.store
          local.get 2
          br_if 1 (;@2;)
          i32.const 0
          i32.const 0
          i32.load offset=1049972
          i32.const -2
          local.get 0
          i32.load offset=28
          i32.rotl
          i32.and
          i32.store offset=1049972
          br 2 (;@1;)
        end
        block ;; label = @3
          local.get 2
          local.get 0
          i32.load offset=8
          local.tee 4
          i32.eq
          br_if 0 (;@3;)
          local.get 4
          local.get 2
          i32.store offset=12
          local.get 2
          local.get 4
          i32.store offset=8
          return
        end
        i32.const 0
        i32.const 0
        i32.load offset=1049968
        i32.const -2
        local.get 1
        i32.const 3
        i32.shr_u
        i32.rotl
        i32.and
        i32.store offset=1049968
        return
      end
      local.get 2
      local.get 3
      i32.store offset=24
      block ;; label = @2
        local.get 0
        i32.load offset=16
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        i32.store offset=16
        local.get 1
        local.get 2
        i32.store offset=24
      end
      local.get 0
      i32.load offset=20
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 1
      i32.store offset=20
      local.get 1
      local.get 2
      i32.store offset=24
      return
    end
  )
  (func (;33;) (type 0) (param i32 i32)
    (local i32 i32)
    local.get 0
    local.get 1
    i32.add
    local.set 2
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 3
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 3
        i32.const 2
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.load
        local.tee 3
        local.get 1
        i32.add
        local.set 1
        block ;; label = @3
          local.get 0
          local.get 3
          i32.sub
          local.tee 0
          i32.const 0
          i32.load offset=1049984
          i32.ne
          br_if 0 (;@3;)
          local.get 2
          i32.load offset=4
          i32.const 3
          i32.and
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
          i32.const 0
          local.get 1
          i32.store offset=1049976
          local.get 2
          local.get 2
          i32.load offset=4
          i32.const -2
          i32.and
          i32.store offset=4
          local.get 0
          local.get 1
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 2
          local.get 1
          i32.store
          br 2 (;@1;)
        end
        local.get 0
        local.get 3
        call 32
      end
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 2
              i32.load offset=4
              local.tee 3
              i32.const 2
              i32.and
              br_if 0 (;@5;)
              local.get 2
              i32.const 0
              i32.load offset=1049988
              i32.eq
              br_if 2 (;@3;)
              local.get 2
              i32.const 0
              i32.load offset=1049984
              i32.eq
              br_if 3 (;@2;)
              local.get 2
              local.get 3
              i32.const -8
              i32.and
              local.tee 3
              call 32
              local.get 0
              local.get 3
              local.get 1
              i32.add
              local.tee 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 1
              i32.add
              local.get 1
              i32.store
              local.get 0
              i32.const 0
              i32.load offset=1049984
              i32.ne
              br_if 1 (;@4;)
              i32.const 0
              local.get 1
              i32.store offset=1049976
              return
            end
            local.get 2
            local.get 3
            i32.const -2
            i32.and
            i32.store offset=4
            local.get 0
            local.get 1
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            local.get 1
            i32.add
            local.get 1
            i32.store
          end
          block ;; label = @4
            local.get 1
            i32.const 256
            i32.lt_u
            br_if 0 (;@4;)
            local.get 0
            local.get 1
            call 34
            return
          end
          local.get 1
          i32.const -8
          i32.and
          i32.const 1049704
          i32.add
          local.set 2
          block ;; label = @4
            block ;; label = @5
              i32.const 0
              i32.load offset=1049968
              local.tee 3
              i32.const 1
              local.get 1
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 1
              i32.and
              br_if 0 (;@5;)
              i32.const 0
              local.get 3
              local.get 1
              i32.or
              i32.store offset=1049968
              local.get 2
              local.set 1
              br 1 (;@4;)
            end
            local.get 2
            i32.load offset=8
            local.set 1
          end
          local.get 2
          local.get 0
          i32.store offset=8
          local.get 1
          local.get 0
          i32.store offset=12
          local.get 0
          local.get 2
          i32.store offset=12
          local.get 0
          local.get 1
          i32.store offset=8
          return
        end
        i32.const 0
        local.get 0
        i32.store offset=1049988
        i32.const 0
        i32.const 0
        i32.load offset=1049980
        local.get 1
        i32.add
        local.tee 1
        i32.store offset=1049980
        local.get 0
        local.get 1
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 0
        i32.const 0
        i32.load offset=1049984
        i32.ne
        br_if 1 (;@1;)
        i32.const 0
        i32.const 0
        i32.store offset=1049976
        i32.const 0
        i32.const 0
        i32.store offset=1049984
        return
      end
      i32.const 0
      local.get 0
      i32.store offset=1049984
      i32.const 0
      i32.const 0
      i32.load offset=1049976
      local.get 1
      i32.add
      local.tee 1
      i32.store offset=1049976
      local.get 0
      local.get 1
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 0
      local.get 1
      i32.add
      local.get 1
      i32.store
      return
    end
  )
  (func (;34;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32)
    i32.const 0
    local.set 2
    block ;; label = @1
      local.get 1
      i32.const 256
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 31
      local.set 2
      local.get 1
      i32.const 16777215
      i32.gt_u
      br_if 0 (;@1;)
      local.get 1
      i32.const 6
      local.get 1
      i32.const 8
      i32.shr_u
      i32.clz
      local.tee 2
      i32.sub
      i32.shr_u
      i32.const 1
      i32.and
      local.get 2
      i32.const 1
      i32.shl
      i32.sub
      i32.const 62
      i32.add
      local.set 2
    end
    local.get 0
    i64.const 0
    i64.store offset=16 align=4
    local.get 0
    local.get 2
    i32.store offset=28
    local.get 2
    i32.const 2
    i32.shl
    i32.const 1049560
    i32.add
    local.set 3
    block ;; label = @1
      i32.const 0
      i32.load offset=1049972
      i32.const 1
      local.get 2
      i32.shl
      local.tee 4
      i32.and
      br_if 0 (;@1;)
      local.get 3
      local.get 0
      i32.store
      local.get 0
      local.get 3
      i32.store offset=24
      local.get 0
      local.get 0
      i32.store offset=12
      local.get 0
      local.get 0
      i32.store offset=8
      i32.const 0
      i32.const 0
      i32.load offset=1049972
      local.get 4
      i32.or
      i32.store offset=1049972
      return
    end
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.load
          local.tee 4
          i32.load offset=4
          i32.const -8
          i32.and
          local.get 1
          i32.ne
          br_if 0 (;@3;)
          local.get 4
          local.set 2
          br 1 (;@2;)
        end
        local.get 1
        i32.const 0
        i32.const 25
        local.get 2
        i32.const 1
        i32.shr_u
        i32.sub
        local.get 2
        i32.const 31
        i32.eq
        select
        i32.shl
        local.set 3
        loop ;; label = @3
          local.get 4
          local.get 3
          i32.const 29
          i32.shr_u
          i32.const 4
          i32.and
          i32.add
          i32.const 16
          i32.add
          local.tee 5
          i32.load
          local.tee 2
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          i32.const 1
          i32.shl
          local.set 3
          local.get 2
          local.set 4
          local.get 2
          i32.load offset=4
          i32.const -8
          i32.and
          local.get 1
          i32.ne
          br_if 0 (;@3;)
        end
      end
      local.get 2
      i32.load offset=8
      local.tee 3
      local.get 0
      i32.store offset=12
      local.get 2
      local.get 0
      i32.store offset=8
      local.get 0
      i32.const 0
      i32.store offset=24
      local.get 0
      local.get 2
      i32.store offset=12
      local.get 0
      local.get 3
      i32.store offset=8
      return
    end
    local.get 5
    local.get 0
    i32.store
    local.get 0
    local.get 4
    i32.store offset=24
    local.get 0
    local.get 0
    i32.store offset=12
    local.get 0
    local.get 0
    i32.store offset=8
  )
  (func (;35;) (type 4) (param i32)
    (local i32 i32 i32 i32 i32)
    local.get 0
    i32.const -8
    i32.add
    local.tee 1
    local.get 0
    i32.const -4
    i32.add
    i32.load
    local.tee 2
    i32.const -8
    i32.and
    local.tee 0
    i32.add
    local.set 3
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 2
        i32.const 2
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        i32.load
        local.tee 2
        local.get 0
        i32.add
        local.set 0
        block ;; label = @3
          local.get 1
          local.get 2
          i32.sub
          local.tee 1
          i32.const 0
          i32.load offset=1049984
          i32.ne
          br_if 0 (;@3;)
          local.get 3
          i32.load offset=4
          i32.const 3
          i32.and
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
          i32.const 0
          local.get 0
          i32.store offset=1049976
          local.get 3
          local.get 3
          i32.load offset=4
          i32.const -2
          i32.and
          i32.store offset=4
          local.get 1
          local.get 0
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 3
          local.get 0
          i32.store
          return
        end
        local.get 1
        local.get 2
        call 32
      end
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 3
                  i32.load offset=4
                  local.tee 2
                  i32.const 2
                  i32.and
                  br_if 0 (;@7;)
                  local.get 3
                  i32.const 0
                  i32.load offset=1049988
                  i32.eq
                  br_if 2 (;@5;)
                  local.get 3
                  i32.const 0
                  i32.load offset=1049984
                  i32.eq
                  br_if 3 (;@4;)
                  local.get 3
                  local.get 2
                  i32.const -8
                  i32.and
                  local.tee 2
                  call 32
                  local.get 1
                  local.get 2
                  local.get 0
                  i32.add
                  local.tee 0
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 1
                  local.get 0
                  i32.add
                  local.get 0
                  i32.store
                  local.get 1
                  i32.const 0
                  i32.load offset=1049984
                  i32.ne
                  br_if 1 (;@6;)
                  i32.const 0
                  local.get 0
                  i32.store offset=1049976
                  return
                end
                local.get 3
                local.get 2
                i32.const -2
                i32.and
                i32.store offset=4
                local.get 1
                local.get 0
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 1
                local.get 0
                i32.add
                local.get 0
                i32.store
              end
              local.get 0
              i32.const 256
              i32.lt_u
              br_if 2 (;@3;)
              local.get 1
              local.get 0
              call 34
              i32.const 0
              local.set 1
              i32.const 0
              i32.const 0
              i32.load offset=1050008
              i32.const -1
              i32.add
              local.tee 0
              i32.store offset=1050008
              local.get 0
              br_if 4 (;@1;)
              block ;; label = @6
                i32.const 0
                i32.load offset=1049696
                local.tee 0
                i32.eqz
                br_if 0 (;@6;)
                i32.const 0
                local.set 1
                loop ;; label = @7
                  local.get 1
                  i32.const 1
                  i32.add
                  local.set 1
                  local.get 0
                  i32.load offset=8
                  local.tee 0
                  br_if 0 (;@7;)
                end
              end
              i32.const 0
              local.get 1
              i32.const 4095
              local.get 1
              i32.const 4095
              i32.gt_u
              select
              i32.store offset=1050008
              return
            end
            i32.const 0
            local.get 1
            i32.store offset=1049988
            i32.const 0
            i32.const 0
            i32.load offset=1049980
            local.get 0
            i32.add
            local.tee 0
            i32.store offset=1049980
            local.get 1
            local.get 0
            i32.const 1
            i32.or
            i32.store offset=4
            block ;; label = @5
              local.get 1
              i32.const 0
              i32.load offset=1049984
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.store offset=1049976
              i32.const 0
              i32.const 0
              i32.store offset=1049984
            end
            local.get 0
            i32.const 0
            i32.load offset=1050000
            local.tee 4
            i32.le_u
            br_if 3 (;@1;)
            i32.const 0
            i32.load offset=1049988
            local.tee 0
            i32.eqz
            br_if 3 (;@1;)
            i32.const 0
            local.set 2
            i32.const 0
            i32.load offset=1049980
            local.tee 5
            i32.const 41
            i32.lt_u
            br_if 2 (;@2;)
            i32.const 1049688
            local.set 1
            loop ;; label = @5
              block ;; label = @6
                local.get 1
                i32.load
                local.tee 3
                local.get 0
                i32.gt_u
                br_if 0 (;@6;)
                local.get 0
                local.get 3
                local.get 1
                i32.load offset=4
                i32.add
                i32.lt_u
                br_if 4 (;@2;)
              end
              local.get 1
              i32.load offset=8
              local.set 1
              br 0 (;@5;)
            end
          end
          i32.const 0
          local.get 1
          i32.store offset=1049984
          i32.const 0
          i32.const 0
          i32.load offset=1049976
          local.get 0
          i32.add
          local.tee 0
          i32.store offset=1049976
          local.get 1
          local.get 0
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 1
          local.get 0
          i32.add
          local.get 0
          i32.store
          return
        end
        local.get 0
        i32.const -8
        i32.and
        i32.const 1049704
        i32.add
        local.set 3
        block ;; label = @3
          block ;; label = @4
            i32.const 0
            i32.load offset=1049968
            local.tee 2
            i32.const 1
            local.get 0
            i32.const 3
            i32.shr_u
            i32.shl
            local.tee 0
            i32.and
            br_if 0 (;@4;)
            i32.const 0
            local.get 2
            local.get 0
            i32.or
            i32.store offset=1049968
            local.get 3
            local.set 0
            br 1 (;@3;)
          end
          local.get 3
          i32.load offset=8
          local.set 0
        end
        local.get 3
        local.get 1
        i32.store offset=8
        local.get 0
        local.get 1
        i32.store offset=12
        local.get 1
        local.get 3
        i32.store offset=12
        local.get 1
        local.get 0
        i32.store offset=8
        return
      end
      block ;; label = @2
        i32.const 0
        i32.load offset=1049696
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        loop ;; label = @3
          local.get 2
          i32.const 1
          i32.add
          local.set 2
          local.get 1
          i32.load offset=8
          local.tee 1
          br_if 0 (;@3;)
        end
      end
      i32.const 0
      local.get 2
      i32.const 4095
      local.get 2
      i32.const 4095
      i32.gt_u
      select
      i32.store offset=1050008
      local.get 5
      local.get 4
      i32.le_u
      br_if 0 (;@1;)
      i32.const 0
      i32.const -1
      i32.store offset=1050000
    end
  )
  (func (;36;) (type 7) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 0
                    i32.const 245
                    i32.lt_u
                    br_if 0 (;@8;)
                    i32.const 0
                    local.set 2
                    local.get 0
                    i32.const -65587
                    i32.ge_u
                    br_if 7 (;@1;)
                    local.get 0
                    i32.const 11
                    i32.add
                    local.tee 0
                    i32.const -8
                    i32.and
                    local.set 3
                    i32.const 0
                    i32.load offset=1049972
                    local.tee 4
                    i32.eqz
                    br_if 4 (;@4;)
                    i32.const 0
                    local.set 5
                    block ;; label = @9
                      local.get 3
                      i32.const 256
                      i32.lt_u
                      br_if 0 (;@9;)
                      i32.const 31
                      local.set 5
                      local.get 3
                      i32.const 16777215
                      i32.gt_u
                      br_if 0 (;@9;)
                      local.get 3
                      i32.const 6
                      local.get 0
                      i32.const 8
                      i32.shr_u
                      i32.clz
                      local.tee 0
                      i32.sub
                      i32.shr_u
                      i32.const 1
                      i32.and
                      local.get 0
                      i32.const 1
                      i32.shl
                      i32.sub
                      i32.const 62
                      i32.add
                      local.set 5
                    end
                    i32.const 0
                    local.get 3
                    i32.sub
                    local.set 2
                    block ;; label = @9
                      local.get 5
                      i32.const 2
                      i32.shl
                      i32.const 1049560
                      i32.add
                      i32.load
                      local.tee 6
                      br_if 0 (;@9;)
                      i32.const 0
                      local.set 0
                      i32.const 0
                      local.set 7
                      br 2 (;@7;)
                    end
                    i32.const 0
                    local.set 0
                    local.get 3
                    i32.const 0
                    i32.const 25
                    local.get 5
                    i32.const 1
                    i32.shr_u
                    i32.sub
                    local.get 5
                    i32.const 31
                    i32.eq
                    select
                    i32.shl
                    local.set 8
                    i32.const 0
                    local.set 7
                    loop ;; label = @9
                      block ;; label = @10
                        local.get 6
                        local.tee 6
                        i32.load offset=4
                        i32.const -8
                        i32.and
                        local.tee 9
                        local.get 3
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 9
                        local.get 3
                        i32.sub
                        local.tee 9
                        local.get 2
                        i32.ge_u
                        br_if 0 (;@10;)
                        local.get 9
                        local.set 2
                        local.get 6
                        local.set 7
                        local.get 9
                        br_if 0 (;@10;)
                        i32.const 0
                        local.set 2
                        local.get 6
                        local.set 7
                        local.get 6
                        local.set 0
                        br 4 (;@6;)
                      end
                      local.get 6
                      i32.load offset=20
                      local.tee 9
                      local.get 0
                      local.get 9
                      local.get 6
                      local.get 8
                      i32.const 29
                      i32.shr_u
                      i32.const 4
                      i32.and
                      i32.add
                      i32.const 16
                      i32.add
                      i32.load
                      local.tee 6
                      i32.ne
                      select
                      local.get 0
                      local.get 9
                      select
                      local.set 0
                      local.get 8
                      i32.const 1
                      i32.shl
                      local.set 8
                      local.get 6
                      i32.eqz
                      br_if 2 (;@7;)
                      br 0 (;@9;)
                    end
                  end
                  block ;; label = @8
                    i32.const 0
                    i32.load offset=1049968
                    local.tee 6
                    i32.const 16
                    local.get 0
                    i32.const 11
                    i32.add
                    i32.const 504
                    i32.and
                    local.get 0
                    i32.const 11
                    i32.lt_u
                    select
                    local.tee 3
                    i32.const 3
                    i32.shr_u
                    local.tee 2
                    i32.shr_u
                    local.tee 0
                    i32.const 3
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                    block ;; label = @9
                      block ;; label = @10
                        local.get 0
                        i32.const -1
                        i32.xor
                        i32.const 1
                        i32.and
                        local.get 2
                        i32.add
                        local.tee 3
                        i32.const 3
                        i32.shl
                        local.tee 0
                        i32.const 1049704
                        i32.add
                        local.tee 2
                        local.get 0
                        i32.const 1049712
                        i32.add
                        i32.load
                        local.tee 0
                        i32.load offset=8
                        local.tee 7
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 7
                        local.get 2
                        i32.store offset=12
                        local.get 2
                        local.get 7
                        i32.store offset=8
                        br 1 (;@9;)
                      end
                      i32.const 0
                      local.get 6
                      i32.const -2
                      local.get 3
                      i32.rotl
                      i32.and
                      i32.store offset=1049968
                    end
                    local.get 0
                    i32.const 8
                    i32.add
                    local.set 2
                    local.get 0
                    local.get 3
                    i32.const 3
                    i32.shl
                    local.tee 3
                    i32.const 3
                    i32.or
                    i32.store offset=4
                    local.get 0
                    local.get 3
                    i32.add
                    local.tee 0
                    local.get 0
                    i32.load offset=4
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    br 7 (;@1;)
                  end
                  local.get 3
                  i32.const 0
                  i32.load offset=1049976
                  i32.le_u
                  br_if 3 (;@4;)
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        local.get 0
                        br_if 0 (;@10;)
                        i32.const 0
                        i32.load offset=1049972
                        local.tee 0
                        i32.eqz
                        br_if 6 (;@4;)
                        local.get 0
                        i32.ctz
                        i32.const 2
                        i32.shl
                        i32.const 1049560
                        i32.add
                        i32.load
                        local.tee 7
                        i32.load offset=4
                        i32.const -8
                        i32.and
                        local.get 3
                        i32.sub
                        local.set 2
                        local.get 7
                        local.set 6
                        loop ;; label = @11
                          block ;; label = @12
                            local.get 7
                            i32.load offset=16
                            local.tee 0
                            br_if 0 (;@12;)
                            local.get 7
                            i32.load offset=20
                            local.tee 0
                            br_if 0 (;@12;)
                            local.get 6
                            i32.load offset=24
                            local.set 5
                            block ;; label = @13
                              block ;; label = @14
                                block ;; label = @15
                                  local.get 6
                                  i32.load offset=12
                                  local.tee 0
                                  local.get 6
                                  i32.ne
                                  br_if 0 (;@15;)
                                  local.get 6
                                  i32.const 20
                                  i32.const 16
                                  local.get 6
                                  i32.load offset=20
                                  local.tee 0
                                  select
                                  i32.add
                                  i32.load
                                  local.tee 7
                                  br_if 1 (;@14;)
                                  i32.const 0
                                  local.set 0
                                  br 2 (;@13;)
                                end
                                local.get 6
                                i32.load offset=8
                                local.tee 7
                                local.get 0
                                i32.store offset=12
                                local.get 0
                                local.get 7
                                i32.store offset=8
                                br 1 (;@13;)
                              end
                              local.get 6
                              i32.const 20
                              i32.add
                              local.get 6
                              i32.const 16
                              i32.add
                              local.get 0
                              select
                              local.set 8
                              loop ;; label = @14
                                local.get 8
                                local.set 9
                                local.get 7
                                local.tee 0
                                i32.const 20
                                i32.add
                                local.get 0
                                i32.const 16
                                i32.add
                                local.get 0
                                i32.load offset=20
                                local.tee 7
                                select
                                local.set 8
                                local.get 0
                                i32.const 20
                                i32.const 16
                                local.get 7
                                select
                                i32.add
                                i32.load
                                local.tee 7
                                br_if 0 (;@14;)
                              end
                              local.get 9
                              i32.const 0
                              i32.store
                            end
                            local.get 5
                            i32.eqz
                            br_if 4 (;@8;)
                            block ;; label = @13
                              local.get 6
                              i32.load offset=28
                              i32.const 2
                              i32.shl
                              i32.const 1049560
                              i32.add
                              local.tee 7
                              i32.load
                              local.get 6
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 5
                              i32.const 16
                              i32.const 20
                              local.get 5
                              i32.load offset=16
                              local.get 6
                              i32.eq
                              select
                              i32.add
                              local.get 0
                              i32.store
                              local.get 0
                              i32.eqz
                              br_if 5 (;@8;)
                              br 4 (;@9;)
                            end
                            local.get 7
                            local.get 0
                            i32.store
                            local.get 0
                            br_if 3 (;@9;)
                            i32.const 0
                            i32.const 0
                            i32.load offset=1049972
                            i32.const -2
                            local.get 6
                            i32.load offset=28
                            i32.rotl
                            i32.and
                            i32.store offset=1049972
                            br 4 (;@8;)
                          end
                          local.get 0
                          i32.load offset=4
                          i32.const -8
                          i32.and
                          local.get 3
                          i32.sub
                          local.tee 7
                          local.get 2
                          local.get 7
                          local.get 2
                          i32.lt_u
                          local.tee 7
                          select
                          local.set 2
                          local.get 0
                          local.get 6
                          local.get 7
                          select
                          local.set 6
                          local.get 0
                          local.set 7
                          br 0 (;@11;)
                        end
                      end
                      block ;; label = @10
                        block ;; label = @11
                          local.get 0
                          local.get 2
                          i32.shl
                          i32.const 2
                          local.get 2
                          i32.shl
                          local.tee 0
                          i32.const 0
                          local.get 0
                          i32.sub
                          i32.or
                          i32.and
                          i32.ctz
                          local.tee 2
                          i32.const 3
                          i32.shl
                          local.tee 0
                          i32.const 1049704
                          i32.add
                          local.tee 7
                          local.get 0
                          i32.const 1049712
                          i32.add
                          i32.load
                          local.tee 0
                          i32.load offset=8
                          local.tee 8
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 8
                          local.get 7
                          i32.store offset=12
                          local.get 7
                          local.get 8
                          i32.store offset=8
                          br 1 (;@10;)
                        end
                        i32.const 0
                        local.get 6
                        i32.const -2
                        local.get 2
                        i32.rotl
                        i32.and
                        i32.store offset=1049968
                      end
                      local.get 0
                      local.get 3
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 3
                      i32.add
                      local.tee 8
                      local.get 2
                      i32.const 3
                      i32.shl
                      local.tee 2
                      local.get 3
                      i32.sub
                      local.tee 7
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 2
                      i32.add
                      local.get 7
                      i32.store
                      block ;; label = @10
                        i32.const 0
                        i32.load offset=1049976
                        local.tee 6
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 6
                        i32.const -8
                        i32.and
                        i32.const 1049704
                        i32.add
                        local.set 2
                        i32.const 0
                        i32.load offset=1049984
                        local.set 3
                        block ;; label = @11
                          block ;; label = @12
                            i32.const 0
                            i32.load offset=1049968
                            local.tee 9
                            i32.const 1
                            local.get 6
                            i32.const 3
                            i32.shr_u
                            i32.shl
                            local.tee 6
                            i32.and
                            br_if 0 (;@12;)
                            i32.const 0
                            local.get 9
                            local.get 6
                            i32.or
                            i32.store offset=1049968
                            local.get 2
                            local.set 6
                            br 1 (;@11;)
                          end
                          local.get 2
                          i32.load offset=8
                          local.set 6
                        end
                        local.get 2
                        local.get 3
                        i32.store offset=8
                        local.get 6
                        local.get 3
                        i32.store offset=12
                        local.get 3
                        local.get 2
                        i32.store offset=12
                        local.get 3
                        local.get 6
                        i32.store offset=8
                      end
                      local.get 0
                      i32.const 8
                      i32.add
                      local.set 2
                      i32.const 0
                      local.get 8
                      i32.store offset=1049984
                      i32.const 0
                      local.get 7
                      i32.store offset=1049976
                      br 8 (;@1;)
                    end
                    local.get 0
                    local.get 5
                    i32.store offset=24
                    block ;; label = @9
                      local.get 6
                      i32.load offset=16
                      local.tee 7
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 0
                      local.get 7
                      i32.store offset=16
                      local.get 7
                      local.get 0
                      i32.store offset=24
                    end
                    local.get 6
                    i32.load offset=20
                    local.tee 7
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 7
                    i32.store offset=20
                    local.get 7
                    local.get 0
                    i32.store offset=24
                  end
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        local.get 2
                        i32.const 16
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 6
                        local.get 3
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 6
                        local.get 3
                        i32.add
                        local.tee 3
                        local.get 2
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 3
                        local.get 2
                        i32.add
                        local.get 2
                        i32.store
                        i32.const 0
                        i32.load offset=1049976
                        local.tee 8
                        i32.eqz
                        br_if 1 (;@9;)
                        local.get 8
                        i32.const -8
                        i32.and
                        i32.const 1049704
                        i32.add
                        local.set 7
                        i32.const 0
                        i32.load offset=1049984
                        local.set 0
                        block ;; label = @11
                          block ;; label = @12
                            i32.const 0
                            i32.load offset=1049968
                            local.tee 9
                            i32.const 1
                            local.get 8
                            i32.const 3
                            i32.shr_u
                            i32.shl
                            local.tee 8
                            i32.and
                            br_if 0 (;@12;)
                            i32.const 0
                            local.get 9
                            local.get 8
                            i32.or
                            i32.store offset=1049968
                            local.get 7
                            local.set 8
                            br 1 (;@11;)
                          end
                          local.get 7
                          i32.load offset=8
                          local.set 8
                        end
                        local.get 7
                        local.get 0
                        i32.store offset=8
                        local.get 8
                        local.get 0
                        i32.store offset=12
                        local.get 0
                        local.get 7
                        i32.store offset=12
                        local.get 0
                        local.get 8
                        i32.store offset=8
                        br 1 (;@9;)
                      end
                      local.get 6
                      local.get 2
                      local.get 3
                      i32.add
                      local.tee 0
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 6
                      local.get 0
                      i32.add
                      local.tee 0
                      local.get 0
                      i32.load offset=4
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      br 1 (;@8;)
                    end
                    i32.const 0
                    local.get 3
                    i32.store offset=1049984
                    i32.const 0
                    local.get 2
                    i32.store offset=1049976
                  end
                  local.get 6
                  i32.const 8
                  i32.add
                  local.set 2
                  br 6 (;@1;)
                end
                block ;; label = @7
                  local.get 0
                  local.get 7
                  i32.or
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 7
                  i32.const 2
                  local.get 5
                  i32.shl
                  local.tee 0
                  i32.const 0
                  local.get 0
                  i32.sub
                  i32.or
                  local.get 4
                  i32.and
                  local.tee 0
                  i32.eqz
                  br_if 3 (;@4;)
                  local.get 0
                  i32.ctz
                  i32.const 2
                  i32.shl
                  i32.const 1049560
                  i32.add
                  i32.load
                  local.set 0
                end
                local.get 0
                i32.eqz
                br_if 1 (;@5;)
              end
              loop ;; label = @6
                local.get 0
                local.get 7
                local.get 0
                i32.load offset=4
                i32.const -8
                i32.and
                local.tee 6
                local.get 3
                i32.sub
                local.tee 9
                local.get 2
                i32.lt_u
                local.tee 5
                select
                local.set 4
                local.get 6
                local.get 3
                i32.lt_u
                local.set 8
                local.get 9
                local.get 2
                local.get 5
                select
                local.set 9
                block ;; label = @7
                  local.get 0
                  i32.load offset=16
                  local.tee 6
                  br_if 0 (;@7;)
                  local.get 0
                  i32.load offset=20
                  local.set 6
                end
                local.get 7
                local.get 4
                local.get 8
                select
                local.set 7
                local.get 2
                local.get 9
                local.get 8
                select
                local.set 2
                local.get 6
                local.set 0
                local.get 6
                br_if 0 (;@6;)
              end
            end
            local.get 7
            i32.eqz
            br_if 0 (;@4;)
            block ;; label = @5
              i32.const 0
              i32.load offset=1049976
              local.tee 0
              local.get 3
              i32.lt_u
              br_if 0 (;@5;)
              local.get 2
              local.get 0
              local.get 3
              i32.sub
              i32.ge_u
              br_if 1 (;@4;)
            end
            local.get 7
            i32.load offset=24
            local.set 5
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 7
                  i32.load offset=12
                  local.tee 0
                  local.get 7
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 7
                  i32.const 20
                  i32.const 16
                  local.get 7
                  i32.load offset=20
                  local.tee 0
                  select
                  i32.add
                  i32.load
                  local.tee 6
                  br_if 1 (;@6;)
                  i32.const 0
                  local.set 0
                  br 2 (;@5;)
                end
                local.get 7
                i32.load offset=8
                local.tee 6
                local.get 0
                i32.store offset=12
                local.get 0
                local.get 6
                i32.store offset=8
                br 1 (;@5;)
              end
              local.get 7
              i32.const 20
              i32.add
              local.get 7
              i32.const 16
              i32.add
              local.get 0
              select
              local.set 8
              loop ;; label = @6
                local.get 8
                local.set 9
                local.get 6
                local.tee 0
                i32.const 20
                i32.add
                local.get 0
                i32.const 16
                i32.add
                local.get 0
                i32.load offset=20
                local.tee 6
                select
                local.set 8
                local.get 0
                i32.const 20
                i32.const 16
                local.get 6
                select
                i32.add
                i32.load
                local.tee 6
                br_if 0 (;@6;)
              end
              local.get 9
              i32.const 0
              i32.store
            end
            local.get 5
            i32.eqz
            br_if 2 (;@2;)
            block ;; label = @5
              local.get 7
              i32.load offset=28
              i32.const 2
              i32.shl
              i32.const 1049560
              i32.add
              local.tee 6
              i32.load
              local.get 7
              i32.eq
              br_if 0 (;@5;)
              local.get 5
              i32.const 16
              i32.const 20
              local.get 5
              i32.load offset=16
              local.get 7
              i32.eq
              select
              i32.add
              local.get 0
              i32.store
              local.get 0
              i32.eqz
              br_if 3 (;@2;)
              br 2 (;@3;)
            end
            local.get 6
            local.get 0
            i32.store
            local.get 0
            br_if 1 (;@3;)
            i32.const 0
            i32.const 0
            i32.load offset=1049972
            i32.const -2
            local.get 7
            i32.load offset=28
            i32.rotl
            i32.and
            i32.store offset=1049972
            br 2 (;@2;)
          end
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      i32.const 0
                      i32.load offset=1049976
                      local.tee 0
                      local.get 3
                      i32.ge_u
                      br_if 0 (;@9;)
                      block ;; label = @10
                        i32.const 0
                        i32.load offset=1049980
                        local.tee 0
                        local.get 3
                        i32.gt_u
                        br_if 0 (;@10;)
                        local.get 1
                        i32.const 4
                        i32.add
                        i32.const 1050012
                        local.get 3
                        i32.const 65583
                        i32.add
                        i32.const -65536
                        i32.and
                        call 56
                        block ;; label = @11
                          local.get 1
                          i32.load offset=4
                          local.tee 6
                          br_if 0 (;@11;)
                          i32.const 0
                          local.set 2
                          br 10 (;@1;)
                        end
                        local.get 1
                        i32.load offset=12
                        local.set 5
                        i32.const 0
                        i32.const 0
                        i32.load offset=1049992
                        local.get 1
                        i32.load offset=8
                        local.tee 9
                        i32.add
                        local.tee 0
                        i32.store offset=1049992
                        i32.const 0
                        i32.const 0
                        i32.load offset=1049996
                        local.tee 2
                        local.get 0
                        local.get 2
                        local.get 0
                        i32.gt_u
                        select
                        i32.store offset=1049996
                        block ;; label = @11
                          block ;; label = @12
                            block ;; label = @13
                              i32.const 0
                              i32.load offset=1049988
                              local.tee 2
                              i32.eqz
                              br_if 0 (;@13;)
                              i32.const 1049688
                              local.set 0
                              loop ;; label = @14
                                local.get 6
                                local.get 0
                                i32.load
                                local.tee 7
                                local.get 0
                                i32.load offset=4
                                local.tee 8
                                i32.add
                                i32.eq
                                br_if 2 (;@12;)
                                local.get 0
                                i32.load offset=8
                                local.tee 0
                                br_if 0 (;@14;)
                                br 3 (;@11;)
                              end
                            end
                            block ;; label = @13
                              block ;; label = @14
                                i32.const 0
                                i32.load offset=1050004
                                local.tee 0
                                i32.eqz
                                br_if 0 (;@14;)
                                local.get 6
                                local.get 0
                                i32.ge_u
                                br_if 1 (;@13;)
                              end
                              i32.const 0
                              local.get 6
                              i32.store offset=1050004
                            end
                            i32.const 0
                            i32.const 4095
                            i32.store offset=1050008
                            i32.const 0
                            local.get 5
                            i32.store offset=1049700
                            i32.const 0
                            local.get 9
                            i32.store offset=1049692
                            i32.const 0
                            local.get 6
                            i32.store offset=1049688
                            i32.const 0
                            i32.const 1049704
                            i32.store offset=1049716
                            i32.const 0
                            i32.const 1049712
                            i32.store offset=1049724
                            i32.const 0
                            i32.const 1049704
                            i32.store offset=1049712
                            i32.const 0
                            i32.const 1049720
                            i32.store offset=1049732
                            i32.const 0
                            i32.const 1049712
                            i32.store offset=1049720
                            i32.const 0
                            i32.const 1049728
                            i32.store offset=1049740
                            i32.const 0
                            i32.const 1049720
                            i32.store offset=1049728
                            i32.const 0
                            i32.const 1049736
                            i32.store offset=1049748
                            i32.const 0
                            i32.const 1049728
                            i32.store offset=1049736
                            i32.const 0
                            i32.const 1049744
                            i32.store offset=1049756
                            i32.const 0
                            i32.const 1049736
                            i32.store offset=1049744
                            i32.const 0
                            i32.const 1049752
                            i32.store offset=1049764
                            i32.const 0
                            i32.const 1049744
                            i32.store offset=1049752
                            i32.const 0
                            i32.const 1049760
                            i32.store offset=1049772
                            i32.const 0
                            i32.const 1049752
                            i32.store offset=1049760
                            i32.const 0
                            i32.const 1049768
                            i32.store offset=1049780
                            i32.const 0
                            i32.const 1049760
                            i32.store offset=1049768
                            i32.const 0
                            i32.const 1049768
                            i32.store offset=1049776
                            i32.const 0
                            i32.const 1049776
                            i32.store offset=1049788
                            i32.const 0
                            i32.const 1049776
                            i32.store offset=1049784
                            i32.const 0
                            i32.const 1049784
                            i32.store offset=1049796
                            i32.const 0
                            i32.const 1049784
                            i32.store offset=1049792
                            i32.const 0
                            i32.const 1049792
                            i32.store offset=1049804
                            i32.const 0
                            i32.const 1049792
                            i32.store offset=1049800
                            i32.const 0
                            i32.const 1049800
                            i32.store offset=1049812
                            i32.const 0
                            i32.const 1049800
                            i32.store offset=1049808
                            i32.const 0
                            i32.const 1049808
                            i32.store offset=1049820
                            i32.const 0
                            i32.const 1049808
                            i32.store offset=1049816
                            i32.const 0
                            i32.const 1049816
                            i32.store offset=1049828
                            i32.const 0
                            i32.const 1049816
                            i32.store offset=1049824
                            i32.const 0
                            i32.const 1049824
                            i32.store offset=1049836
                            i32.const 0
                            i32.const 1049824
                            i32.store offset=1049832
                            i32.const 0
                            i32.const 1049832
                            i32.store offset=1049844
                            i32.const 0
                            i32.const 1049840
                            i32.store offset=1049852
                            i32.const 0
                            i32.const 1049832
                            i32.store offset=1049840
                            i32.const 0
                            i32.const 1049848
                            i32.store offset=1049860
                            i32.const 0
                            i32.const 1049840
                            i32.store offset=1049848
                            i32.const 0
                            i32.const 1049856
                            i32.store offset=1049868
                            i32.const 0
                            i32.const 1049848
                            i32.store offset=1049856
                            i32.const 0
                            i32.const 1049864
                            i32.store offset=1049876
                            i32.const 0
                            i32.const 1049856
                            i32.store offset=1049864
                            i32.const 0
                            i32.const 1049872
                            i32.store offset=1049884
                            i32.const 0
                            i32.const 1049864
                            i32.store offset=1049872
                            i32.const 0
                            i32.const 1049880
                            i32.store offset=1049892
                            i32.const 0
                            i32.const 1049872
                            i32.store offset=1049880
                            i32.const 0
                            i32.const 1049888
                            i32.store offset=1049900
                            i32.const 0
                            i32.const 1049880
                            i32.store offset=1049888
                            i32.const 0
                            i32.const 1049896
                            i32.store offset=1049908
                            i32.const 0
                            i32.const 1049888
                            i32.store offset=1049896
                            i32.const 0
                            i32.const 1049904
                            i32.store offset=1049916
                            i32.const 0
                            i32.const 1049896
                            i32.store offset=1049904
                            i32.const 0
                            i32.const 1049912
                            i32.store offset=1049924
                            i32.const 0
                            i32.const 1049904
                            i32.store offset=1049912
                            i32.const 0
                            i32.const 1049920
                            i32.store offset=1049932
                            i32.const 0
                            i32.const 1049912
                            i32.store offset=1049920
                            i32.const 0
                            i32.const 1049928
                            i32.store offset=1049940
                            i32.const 0
                            i32.const 1049920
                            i32.store offset=1049928
                            i32.const 0
                            i32.const 1049936
                            i32.store offset=1049948
                            i32.const 0
                            i32.const 1049928
                            i32.store offset=1049936
                            i32.const 0
                            i32.const 1049944
                            i32.store offset=1049956
                            i32.const 0
                            i32.const 1049936
                            i32.store offset=1049944
                            i32.const 0
                            i32.const 1049952
                            i32.store offset=1049964
                            i32.const 0
                            i32.const 1049944
                            i32.store offset=1049952
                            i32.const 0
                            local.get 6
                            i32.const 15
                            i32.add
                            i32.const -8
                            i32.and
                            local.tee 0
                            i32.const -8
                            i32.add
                            local.tee 2
                            i32.store offset=1049988
                            i32.const 0
                            i32.const 1049952
                            i32.store offset=1049960
                            i32.const 0
                            local.get 6
                            local.get 0
                            i32.sub
                            local.get 9
                            i32.const -40
                            i32.add
                            local.tee 0
                            i32.add
                            i32.const 8
                            i32.add
                            local.tee 7
                            i32.store offset=1049980
                            local.get 2
                            local.get 7
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            local.get 6
                            local.get 0
                            i32.add
                            i32.const 40
                            i32.store offset=4
                            i32.const 0
                            i32.const 2097152
                            i32.store offset=1050000
                            br 8 (;@4;)
                          end
                          local.get 2
                          local.get 6
                          i32.ge_u
                          br_if 0 (;@11;)
                          local.get 7
                          local.get 2
                          i32.gt_u
                          br_if 0 (;@11;)
                          local.get 0
                          i32.load offset=12
                          local.tee 7
                          i32.const 1
                          i32.and
                          br_if 0 (;@11;)
                          local.get 7
                          i32.const 1
                          i32.shr_u
                          local.get 5
                          i32.eq
                          br_if 3 (;@8;)
                        end
                        i32.const 0
                        i32.const 0
                        i32.load offset=1050004
                        local.tee 0
                        local.get 6
                        local.get 6
                        local.get 0
                        i32.gt_u
                        select
                        i32.store offset=1050004
                        local.get 6
                        local.get 9
                        i32.add
                        local.set 7
                        i32.const 1049688
                        local.set 0
                        block ;; label = @11
                          block ;; label = @12
                            block ;; label = @13
                              loop ;; label = @14
                                local.get 0
                                i32.load
                                local.get 7
                                i32.eq
                                br_if 1 (;@13;)
                                local.get 0
                                i32.load offset=8
                                local.tee 0
                                br_if 0 (;@14;)
                                br 2 (;@12;)
                              end
                            end
                            local.get 0
                            i32.load offset=12
                            local.tee 8
                            i32.const 1
                            i32.and
                            br_if 0 (;@12;)
                            local.get 8
                            i32.const 1
                            i32.shr_u
                            local.get 5
                            i32.eq
                            br_if 1 (;@11;)
                          end
                          i32.const 1049688
                          local.set 0
                          block ;; label = @12
                            loop ;; label = @13
                              block ;; label = @14
                                local.get 0
                                i32.load
                                local.tee 7
                                local.get 2
                                i32.gt_u
                                br_if 0 (;@14;)
                                local.get 2
                                local.get 7
                                local.get 0
                                i32.load offset=4
                                i32.add
                                local.tee 7
                                i32.lt_u
                                br_if 2 (;@12;)
                              end
                              local.get 0
                              i32.load offset=8
                              local.set 0
                              br 0 (;@13;)
                            end
                          end
                          i32.const 0
                          local.get 6
                          i32.const 15
                          i32.add
                          i32.const -8
                          i32.and
                          local.tee 0
                          i32.const -8
                          i32.add
                          local.tee 8
                          i32.store offset=1049988
                          i32.const 0
                          local.get 6
                          local.get 0
                          i32.sub
                          local.get 9
                          i32.const -40
                          i32.add
                          local.tee 0
                          i32.add
                          i32.const 8
                          i32.add
                          local.tee 4
                          i32.store offset=1049980
                          local.get 8
                          local.get 4
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 6
                          local.get 0
                          i32.add
                          i32.const 40
                          i32.store offset=4
                          i32.const 0
                          i32.const 2097152
                          i32.store offset=1050000
                          local.get 2
                          local.get 7
                          i32.const -32
                          i32.add
                          i32.const -8
                          i32.and
                          i32.const -8
                          i32.add
                          local.tee 0
                          local.get 0
                          local.get 2
                          i32.const 16
                          i32.add
                          i32.lt_u
                          select
                          local.tee 8
                          i32.const 27
                          i32.store offset=4
                          i32.const 0
                          i64.load offset=1049688 align=4
                          local.set 10
                          local.get 8
                          i32.const 16
                          i32.add
                          i32.const 0
                          i64.load offset=1049696 align=4
                          i64.store align=4
                          local.get 8
                          local.get 10
                          i64.store offset=8 align=4
                          i32.const 0
                          local.get 5
                          i32.store offset=1049700
                          i32.const 0
                          local.get 9
                          i32.store offset=1049692
                          i32.const 0
                          local.get 6
                          i32.store offset=1049688
                          i32.const 0
                          local.get 8
                          i32.const 8
                          i32.add
                          i32.store offset=1049696
                          local.get 8
                          i32.const 28
                          i32.add
                          local.set 0
                          loop ;; label = @12
                            local.get 0
                            i32.const 7
                            i32.store
                            local.get 0
                            i32.const 4
                            i32.add
                            local.tee 0
                            local.get 7
                            i32.lt_u
                            br_if 0 (;@12;)
                          end
                          local.get 8
                          local.get 2
                          i32.eq
                          br_if 7 (;@4;)
                          local.get 8
                          local.get 8
                          i32.load offset=4
                          i32.const -2
                          i32.and
                          i32.store offset=4
                          local.get 2
                          local.get 8
                          local.get 2
                          i32.sub
                          local.tee 0
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 8
                          local.get 0
                          i32.store
                          block ;; label = @12
                            local.get 0
                            i32.const 256
                            i32.lt_u
                            br_if 0 (;@12;)
                            local.get 2
                            local.get 0
                            call 34
                            br 8 (;@4;)
                          end
                          local.get 0
                          i32.const -8
                          i32.and
                          i32.const 1049704
                          i32.add
                          local.set 7
                          block ;; label = @12
                            block ;; label = @13
                              i32.const 0
                              i32.load offset=1049968
                              local.tee 6
                              i32.const 1
                              local.get 0
                              i32.const 3
                              i32.shr_u
                              i32.shl
                              local.tee 0
                              i32.and
                              br_if 0 (;@13;)
                              i32.const 0
                              local.get 6
                              local.get 0
                              i32.or
                              i32.store offset=1049968
                              local.get 7
                              local.set 0
                              br 1 (;@12;)
                            end
                            local.get 7
                            i32.load offset=8
                            local.set 0
                          end
                          local.get 7
                          local.get 2
                          i32.store offset=8
                          local.get 0
                          local.get 2
                          i32.store offset=12
                          local.get 2
                          local.get 7
                          i32.store offset=12
                          local.get 2
                          local.get 0
                          i32.store offset=8
                          br 7 (;@4;)
                        end
                        local.get 0
                        local.get 6
                        i32.store
                        local.get 0
                        local.get 0
                        i32.load offset=4
                        local.get 9
                        i32.add
                        i32.store offset=4
                        local.get 6
                        i32.const 15
                        i32.add
                        i32.const -8
                        i32.and
                        i32.const -8
                        i32.add
                        local.tee 6
                        local.get 3
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 7
                        i32.const 15
                        i32.add
                        i32.const -8
                        i32.and
                        i32.const -8
                        i32.add
                        local.tee 2
                        local.get 6
                        local.get 3
                        i32.add
                        local.tee 0
                        i32.sub
                        local.set 3
                        local.get 2
                        i32.const 0
                        i32.load offset=1049988
                        i32.eq
                        br_if 3 (;@7;)
                        local.get 2
                        i32.const 0
                        i32.load offset=1049984
                        i32.eq
                        br_if 4 (;@6;)
                        block ;; label = @11
                          local.get 2
                          i32.load offset=4
                          local.tee 7
                          i32.const 3
                          i32.and
                          i32.const 1
                          i32.ne
                          br_if 0 (;@11;)
                          local.get 2
                          local.get 7
                          i32.const -8
                          i32.and
                          local.tee 7
                          call 32
                          local.get 7
                          local.get 3
                          i32.add
                          local.set 3
                          local.get 2
                          local.get 7
                          i32.add
                          local.tee 2
                          i32.load offset=4
                          local.set 7
                        end
                        local.get 2
                        local.get 7
                        i32.const -2
                        i32.and
                        i32.store offset=4
                        local.get 0
                        local.get 3
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        local.get 0
                        local.get 3
                        i32.add
                        local.get 3
                        i32.store
                        block ;; label = @11
                          local.get 3
                          i32.const 256
                          i32.lt_u
                          br_if 0 (;@11;)
                          local.get 0
                          local.get 3
                          call 34
                          br 6 (;@5;)
                        end
                        local.get 3
                        i32.const -8
                        i32.and
                        i32.const 1049704
                        i32.add
                        local.set 2
                        block ;; label = @11
                          block ;; label = @12
                            i32.const 0
                            i32.load offset=1049968
                            local.tee 7
                            i32.const 1
                            local.get 3
                            i32.const 3
                            i32.shr_u
                            i32.shl
                            local.tee 3
                            i32.and
                            br_if 0 (;@12;)
                            i32.const 0
                            local.get 7
                            local.get 3
                            i32.or
                            i32.store offset=1049968
                            local.get 2
                            local.set 3
                            br 1 (;@11;)
                          end
                          local.get 2
                          i32.load offset=8
                          local.set 3
                        end
                        local.get 2
                        local.get 0
                        i32.store offset=8
                        local.get 3
                        local.get 0
                        i32.store offset=12
                        local.get 0
                        local.get 2
                        i32.store offset=12
                        local.get 0
                        local.get 3
                        i32.store offset=8
                        br 5 (;@5;)
                      end
                      i32.const 0
                      local.get 0
                      local.get 3
                      i32.sub
                      local.tee 2
                      i32.store offset=1049980
                      i32.const 0
                      i32.const 0
                      i32.load offset=1049988
                      local.tee 0
                      local.get 3
                      i32.add
                      local.tee 7
                      i32.store offset=1049988
                      local.get 7
                      local.get 2
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 0
                      local.get 3
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 0
                      i32.const 8
                      i32.add
                      local.set 2
                      br 8 (;@1;)
                    end
                    i32.const 0
                    i32.load offset=1049984
                    local.set 2
                    block ;; label = @9
                      block ;; label = @10
                        local.get 0
                        local.get 3
                        i32.sub
                        local.tee 7
                        i32.const 15
                        i32.gt_u
                        br_if 0 (;@10;)
                        i32.const 0
                        i32.const 0
                        i32.store offset=1049984
                        i32.const 0
                        i32.const 0
                        i32.store offset=1049976
                        local.get 2
                        local.get 0
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 2
                        local.get 0
                        i32.add
                        local.tee 0
                        local.get 0
                        i32.load offset=4
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        br 1 (;@9;)
                      end
                      i32.const 0
                      local.get 7
                      i32.store offset=1049976
                      i32.const 0
                      local.get 2
                      local.get 3
                      i32.add
                      local.tee 6
                      i32.store offset=1049984
                      local.get 6
                      local.get 7
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 2
                      local.get 0
                      i32.add
                      local.get 7
                      i32.store
                      local.get 2
                      local.get 3
                      i32.const 3
                      i32.or
                      i32.store offset=4
                    end
                    local.get 2
                    i32.const 8
                    i32.add
                    local.set 2
                    br 7 (;@1;)
                  end
                  local.get 0
                  local.get 8
                  local.get 9
                  i32.add
                  i32.store offset=4
                  i32.const 0
                  i32.const 0
                  i32.load offset=1049988
                  local.tee 0
                  i32.const 15
                  i32.add
                  i32.const -8
                  i32.and
                  local.tee 2
                  i32.const -8
                  i32.add
                  local.tee 7
                  i32.store offset=1049988
                  i32.const 0
                  local.get 0
                  local.get 2
                  i32.sub
                  i32.const 0
                  i32.load offset=1049980
                  local.get 9
                  i32.add
                  local.tee 2
                  i32.add
                  i32.const 8
                  i32.add
                  local.tee 6
                  i32.store offset=1049980
                  local.get 7
                  local.get 6
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 0
                  local.get 2
                  i32.add
                  i32.const 40
                  i32.store offset=4
                  i32.const 0
                  i32.const 2097152
                  i32.store offset=1050000
                  br 3 (;@4;)
                end
                i32.const 0
                local.get 0
                i32.store offset=1049988
                i32.const 0
                i32.const 0
                i32.load offset=1049980
                local.get 3
                i32.add
                local.tee 3
                i32.store offset=1049980
                local.get 0
                local.get 3
                i32.const 1
                i32.or
                i32.store offset=4
                br 1 (;@5;)
              end
              i32.const 0
              local.get 0
              i32.store offset=1049984
              i32.const 0
              i32.const 0
              i32.load offset=1049976
              local.get 3
              i32.add
              local.tee 3
              i32.store offset=1049976
              local.get 0
              local.get 3
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 3
              i32.add
              local.get 3
              i32.store
            end
            local.get 6
            i32.const 8
            i32.add
            local.set 2
            br 3 (;@1;)
          end
          i32.const 0
          local.set 2
          i32.const 0
          i32.load offset=1049980
          local.tee 0
          local.get 3
          i32.le_u
          br_if 2 (;@1;)
          i32.const 0
          local.get 0
          local.get 3
          i32.sub
          local.tee 2
          i32.store offset=1049980
          i32.const 0
          i32.const 0
          i32.load offset=1049988
          local.tee 0
          local.get 3
          i32.add
          local.tee 7
          i32.store offset=1049988
          local.get 7
          local.get 2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          local.get 3
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 0
          i32.const 8
          i32.add
          local.set 2
          br 2 (;@1;)
        end
        local.get 0
        local.get 5
        i32.store offset=24
        block ;; label = @3
          local.get 7
          i32.load offset=16
          local.tee 6
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          local.get 6
          i32.store offset=16
          local.get 6
          local.get 0
          i32.store offset=24
        end
        local.get 7
        i32.load offset=20
        local.tee 6
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.get 6
        i32.store offset=20
        local.get 6
        local.get 0
        i32.store offset=24
      end
      block ;; label = @2
        block ;; label = @3
          local.get 2
          i32.const 16
          i32.lt_u
          br_if 0 (;@3;)
          local.get 7
          local.get 3
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 7
          local.get 3
          i32.add
          local.tee 0
          local.get 2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          local.get 2
          i32.add
          local.get 2
          i32.store
          block ;; label = @4
            local.get 2
            i32.const 256
            i32.lt_u
            br_if 0 (;@4;)
            local.get 0
            local.get 2
            call 34
            br 2 (;@2;)
          end
          local.get 2
          i32.const -8
          i32.and
          i32.const 1049704
          i32.add
          local.set 3
          block ;; label = @4
            block ;; label = @5
              i32.const 0
              i32.load offset=1049968
              local.tee 6
              i32.const 1
              local.get 2
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 2
              i32.and
              br_if 0 (;@5;)
              i32.const 0
              local.get 6
              local.get 2
              i32.or
              i32.store offset=1049968
              local.get 3
              local.set 2
              br 1 (;@4;)
            end
            local.get 3
            i32.load offset=8
            local.set 2
          end
          local.get 3
          local.get 0
          i32.store offset=8
          local.get 2
          local.get 0
          i32.store offset=12
          local.get 0
          local.get 3
          i32.store offset=12
          local.get 0
          local.get 2
          i32.store offset=8
          br 1 (;@2;)
        end
        local.get 7
        local.get 2
        local.get 3
        i32.add
        local.tee 0
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 7
        local.get 0
        i32.add
        local.tee 0
        local.get 0
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
      end
      local.get 7
      i32.const 8
      i32.add
      local.set 2
    end
    local.get 1
    i32.const 16
    i32.add
    global.set 0
    local.get 2
  )
  (func (;37;) (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    i32.const 0
    local.set 2
    block ;; label = @1
      i32.const -65587
      local.get 0
      i32.const 16
      local.get 0
      i32.const 16
      i32.gt_u
      select
      local.tee 0
      i32.sub
      local.get 1
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 16
      local.get 1
      i32.const 11
      i32.add
      i32.const -8
      i32.and
      local.get 1
      i32.const 11
      i32.lt_u
      select
      local.tee 3
      i32.add
      i32.const 12
      i32.add
      call 36
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const -8
      i32.add
      local.set 2
      block ;; label = @2
        block ;; label = @3
          local.get 0
          i32.const -1
          i32.add
          local.tee 4
          local.get 1
          i32.and
          br_if 0 (;@3;)
          local.get 2
          local.set 0
          br 1 (;@2;)
        end
        local.get 1
        i32.const -4
        i32.add
        local.tee 5
        i32.load
        local.tee 6
        i32.const -8
        i32.and
        local.get 4
        local.get 1
        i32.add
        i32.const 0
        local.get 0
        i32.sub
        i32.and
        i32.const -8
        i32.add
        local.tee 1
        i32.const 0
        local.get 0
        local.get 1
        local.get 2
        i32.sub
        i32.const 16
        i32.gt_u
        select
        i32.add
        local.tee 0
        local.get 2
        i32.sub
        local.tee 1
        i32.sub
        local.set 4
        block ;; label = @3
          local.get 6
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          local.get 4
          local.get 0
          i32.load offset=4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store offset=4
          local.get 0
          local.get 4
          i32.add
          local.tee 4
          local.get 4
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 5
          local.get 1
          local.get 5
          i32.load
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 2
          local.get 1
          i32.add
          local.tee 4
          local.get 4
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 2
          local.get 1
          call 33
          br 1 (;@2;)
        end
        local.get 2
        i32.load
        local.set 2
        local.get 0
        local.get 4
        i32.store offset=4
        local.get 0
        local.get 2
        local.get 1
        i32.add
        i32.store
      end
      block ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.const -8
        i32.and
        local.tee 2
        local.get 3
        i32.const 16
        i32.add
        i32.le_u
        br_if 0 (;@2;)
        local.get 0
        local.get 3
        local.get 1
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store offset=4
        local.get 0
        local.get 3
        i32.add
        local.tee 1
        local.get 2
        local.get 3
        i32.sub
        local.tee 3
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 0
        local.get 2
        i32.add
        local.tee 2
        local.get 2
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 1
        local.get 3
        call 33
      end
      local.get 0
      i32.const 8
      i32.add
      local.set 2
    end
    local.get 2
  )
  (func (;38;) (type 4) (param i32)
    local.get 0
    call 39
    unreachable
  )
  (func (;39;) (type 4) (param i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 0
    i32.load offset=12
    local.set 2
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 0
            i32.load offset=4
            br_table 0 (;@4;) 1 (;@3;) 2 (;@2;)
          end
          local.get 2
          br_if 1 (;@2;)
          i32.const 1
          local.set 2
          i32.const 0
          local.set 3
          br 2 (;@1;)
        end
        local.get 2
        br_if 0 (;@2;)
        local.get 0
        i32.load
        local.tee 2
        i32.load offset=4
        local.set 3
        local.get 2
        i32.load
        local.set 2
        br 1 (;@1;)
      end
      local.get 1
      i32.const -2147483648
      i32.store
      local.get 1
      local.get 0
      i32.store offset=12
      local.get 1
      i32.const 1048972
      local.get 0
      i32.load offset=24
      local.get 0
      i32.load offset=28
      local.tee 0
      i32.load8_u offset=28
      local.get 0
      i32.load8_u offset=29
      call 52
      unreachable
    end
    local.get 1
    local.get 3
    i32.store offset=4
    local.get 1
    local.get 2
    i32.store
    local.get 1
    i32.const 1048944
    local.get 0
    i32.load offset=24
    local.get 0
    i32.load offset=28
    local.tee 0
    i32.load8_u offset=28
    local.get 0
    i32.load8_u offset=29
    call 52
    unreachable
  )
  (func (;40;) (type 0) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    global.set 0
    block ;; label = @1
      i32.const 0
      i32.load8_u offset=1049536
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.const 2
      i32.store offset=12
      local.get 2
      i32.const 1048856
      i32.store offset=8
      local.get 2
      i64.const 1
      i64.store offset=20 align=4
      local.get 2
      local.get 1
      i32.store offset=44
      local.get 2
      i32.const 2
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get 2
      i32.const 44
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=32
      local.get 2
      local.get 2
      i32.const 32
      i32.add
      i32.store offset=16
      local.get 2
      i32.const 8
      i32.add
      i32.const 1048896
      call 70
      unreachable
    end
    local.get 2
    i32.const 48
    i32.add
    global.set 0
  )
  (func (;41;) (type 2) (param i32 i32) (result i32)
    block ;; label = @1
      local.get 1
      i32.const 9
      i32.lt_u
      br_if 0 (;@1;)
      local.get 1
      local.get 0
      call 37
      return
    end
    local.get 0
    call 36
  )
  (func (;42;) (type 3) (param i32 i32 i32)
    (local i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.const -4
        i32.add
        i32.load
        local.tee 3
        i32.const -8
        i32.and
        local.tee 4
        i32.const 4
        i32.const 8
        local.get 3
        i32.const 3
        i32.and
        local.tee 3
        select
        local.get 1
        i32.add
        i32.lt_u
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 1
          i32.const 39
          i32.add
          i32.gt_u
          br_if 2 (;@1;)
        end
        local.get 0
        call 35
        return
      end
      i32.const 1048693
      i32.const 46
      i32.const 1048740
      call 73
      unreachable
    end
    i32.const 1048756
    i32.const 46
    i32.const 1048804
    call 73
    unreachable
  )
  (func (;43;) (type 10) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 0
              i32.const -4
              i32.add
              local.tee 4
              i32.load
              local.tee 5
              i32.const -8
              i32.and
              local.tee 6
              i32.const 4
              i32.const 8
              local.get 5
              i32.const 3
              i32.and
              local.tee 7
              select
              local.get 1
              i32.add
              i32.lt_u
              br_if 0 (;@5;)
              local.get 1
              i32.const 39
              i32.add
              local.set 8
              block ;; label = @6
                local.get 7
                i32.eqz
                br_if 0 (;@6;)
                local.get 6
                local.get 8
                i32.gt_u
                br_if 2 (;@4;)
              end
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 2
                    i32.const 9
                    i32.lt_u
                    br_if 0 (;@8;)
                    local.get 2
                    local.get 3
                    call 37
                    local.tee 2
                    br_if 1 (;@7;)
                    i32.const 0
                    return
                  end
                  i32.const 0
                  local.set 2
                  local.get 3
                  i32.const -65588
                  i32.gt_u
                  br_if 1 (;@6;)
                  i32.const 16
                  local.get 3
                  i32.const 11
                  i32.add
                  i32.const -8
                  i32.and
                  local.get 3
                  i32.const 11
                  i32.lt_u
                  select
                  local.set 1
                  block ;; label = @8
                    block ;; label = @9
                      local.get 7
                      br_if 0 (;@9;)
                      local.get 1
                      i32.const 256
                      i32.lt_u
                      br_if 1 (;@8;)
                      local.get 6
                      local.get 1
                      i32.const 4
                      i32.or
                      i32.lt_u
                      br_if 1 (;@8;)
                      local.get 6
                      local.get 1
                      i32.sub
                      i32.const 131073
                      i32.ge_u
                      br_if 1 (;@8;)
                      local.get 0
                      return
                    end
                    local.get 0
                    i32.const -8
                    i32.add
                    local.tee 8
                    local.get 6
                    i32.add
                    local.set 7
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            block ;; label = @13
                              local.get 6
                              local.get 1
                              i32.ge_u
                              br_if 0 (;@13;)
                              local.get 7
                              i32.const 0
                              i32.load offset=1049988
                              i32.eq
                              br_if 4 (;@9;)
                              local.get 7
                              i32.const 0
                              i32.load offset=1049984
                              i32.eq
                              br_if 2 (;@11;)
                              local.get 7
                              i32.load offset=4
                              local.tee 5
                              i32.const 2
                              i32.and
                              br_if 5 (;@8;)
                              local.get 5
                              i32.const -8
                              i32.and
                              local.tee 9
                              local.get 6
                              i32.add
                              local.tee 5
                              local.get 1
                              i32.lt_u
                              br_if 5 (;@8;)
                              local.get 7
                              local.get 9
                              call 32
                              local.get 5
                              local.get 1
                              i32.sub
                              local.tee 3
                              i32.const 16
                              i32.lt_u
                              br_if 1 (;@12;)
                              local.get 4
                              local.get 1
                              local.get 4
                              i32.load
                              i32.const 1
                              i32.and
                              i32.or
                              i32.const 2
                              i32.or
                              i32.store
                              local.get 8
                              local.get 1
                              i32.add
                              local.tee 1
                              local.get 3
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              local.get 8
                              local.get 5
                              i32.add
                              local.tee 2
                              local.get 2
                              i32.load offset=4
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              local.get 1
                              local.get 3
                              call 33
                              local.get 0
                              return
                            end
                            local.get 6
                            local.get 1
                            i32.sub
                            local.tee 3
                            i32.const 15
                            i32.gt_u
                            br_if 2 (;@10;)
                            local.get 0
                            return
                          end
                          local.get 4
                          local.get 5
                          local.get 4
                          i32.load
                          i32.const 1
                          i32.and
                          i32.or
                          i32.const 2
                          i32.or
                          i32.store
                          local.get 8
                          local.get 5
                          i32.add
                          local.tee 1
                          local.get 1
                          i32.load offset=4
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 0
                          return
                        end
                        i32.const 0
                        i32.load offset=1049976
                        local.get 6
                        i32.add
                        local.tee 7
                        local.get 1
                        i32.lt_u
                        br_if 2 (;@8;)
                        block ;; label = @11
                          block ;; label = @12
                            local.get 7
                            local.get 1
                            i32.sub
                            local.tee 3
                            i32.const 15
                            i32.gt_u
                            br_if 0 (;@12;)
                            local.get 4
                            local.get 5
                            i32.const 1
                            i32.and
                            local.get 7
                            i32.or
                            i32.const 2
                            i32.or
                            i32.store
                            local.get 8
                            local.get 7
                            i32.add
                            local.tee 1
                            local.get 1
                            i32.load offset=4
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            i32.const 0
                            local.set 3
                            i32.const 0
                            local.set 1
                            br 1 (;@11;)
                          end
                          local.get 4
                          local.get 1
                          local.get 5
                          i32.const 1
                          i32.and
                          i32.or
                          i32.const 2
                          i32.or
                          i32.store
                          local.get 8
                          local.get 1
                          i32.add
                          local.tee 1
                          local.get 3
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          local.get 8
                          local.get 7
                          i32.add
                          local.tee 2
                          local.get 3
                          i32.store
                          local.get 2
                          local.get 2
                          i32.load offset=4
                          i32.const -2
                          i32.and
                          i32.store offset=4
                        end
                        i32.const 0
                        local.get 1
                        i32.store offset=1049984
                        i32.const 0
                        local.get 3
                        i32.store offset=1049976
                        local.get 0
                        return
                      end
                      local.get 4
                      local.get 1
                      local.get 5
                      i32.const 1
                      i32.and
                      i32.or
                      i32.const 2
                      i32.or
                      i32.store
                      local.get 8
                      local.get 1
                      i32.add
                      local.tee 1
                      local.get 3
                      i32.const 3
                      i32.or
                      i32.store offset=4
                      local.get 7
                      local.get 7
                      i32.load offset=4
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 1
                      local.get 3
                      call 33
                      local.get 0
                      return
                    end
                    i32.const 0
                    i32.load offset=1049980
                    local.get 6
                    i32.add
                    local.tee 7
                    local.get 1
                    i32.gt_u
                    br_if 7 (;@1;)
                  end
                  local.get 3
                  call 36
                  local.tee 1
                  i32.eqz
                  br_if 1 (;@6;)
                  local.get 1
                  local.get 0
                  i32.const -4
                  i32.const -8
                  local.get 4
                  i32.load
                  local.tee 2
                  i32.const 3
                  i32.and
                  select
                  local.get 2
                  i32.const -8
                  i32.and
                  i32.add
                  local.tee 2
                  local.get 3
                  local.get 2
                  local.get 3
                  i32.lt_u
                  select
                  call 88
                  local.set 1
                  local.get 0
                  call 35
                  local.get 1
                  return
                end
                local.get 2
                local.get 0
                local.get 1
                local.get 3
                local.get 1
                local.get 3
                i32.lt_u
                select
                call 88
                drop
                local.get 4
                i32.load
                local.tee 3
                i32.const -8
                i32.and
                local.tee 7
                i32.const 4
                i32.const 8
                local.get 3
                i32.const 3
                i32.and
                local.tee 3
                select
                local.get 1
                i32.add
                i32.lt_u
                br_if 3 (;@3;)
                block ;; label = @7
                  local.get 3
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 7
                  local.get 8
                  i32.gt_u
                  br_if 5 (;@2;)
                end
                local.get 0
                call 35
              end
              local.get 2
              return
            end
            i32.const 1048693
            i32.const 46
            i32.const 1048740
            call 73
            unreachable
          end
          i32.const 1048756
          i32.const 46
          i32.const 1048804
          call 73
          unreachable
        end
        i32.const 1048693
        i32.const 46
        i32.const 1048740
        call 73
        unreachable
      end
      i32.const 1048756
      i32.const 46
      i32.const 1048804
      call 73
      unreachable
    end
    local.get 4
    local.get 1
    local.get 5
    i32.const 1
    i32.and
    i32.or
    i32.const 2
    i32.or
    i32.store
    local.get 8
    local.get 1
    i32.add
    local.tee 3
    local.get 7
    local.get 1
    i32.sub
    local.tee 1
    i32.const 1
    i32.or
    i32.store offset=4
    i32.const 0
    local.get 1
    i32.store offset=1049980
    i32.const 0
    local.get 3
    i32.store offset=1049988
    local.get 0
  )
  (func (;44;) (type 4) (param i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    local.get 0
    i32.load offset=24
    local.set 2
    local.get 1
    i32.const 16
    i32.add
    local.get 0
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 1
    i32.const 8
    i32.add
    local.get 0
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 1
    local.get 0
    i32.store offset=28
    local.get 1
    local.get 2
    i32.store offset=24
    local.get 1
    local.get 0
    i64.load align=4
    i64.store
    local.get 1
    call 38
    unreachable
  )
  (func (;45;) (type 0) (param i32 i32)
    (local i32 i32 i32 i64)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 2
    global.set 0
    block ;; label = @1
      local.get 1
      i32.load
      i32.const -2147483648
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=12
      local.set 3
      local.get 2
      i32.const 28
      i32.add
      i32.const 8
      i32.add
      local.tee 4
      i32.const 0
      i32.store
      local.get 2
      i64.const 4294967296
      i64.store offset=28 align=4
      local.get 2
      i32.const 40
      i32.add
      i32.const 16
      i32.add
      local.get 3
      i32.const 16
      i32.add
      i64.load align=4
      i64.store
      local.get 2
      i32.const 40
      i32.add
      i32.const 8
      i32.add
      local.get 3
      i32.const 8
      i32.add
      i64.load align=4
      i64.store
      local.get 2
      local.get 3
      i64.load align=4
      i64.store offset=40
      local.get 2
      i32.const 28
      i32.add
      i32.const 1048628
      local.get 2
      i32.const 40
      i32.add
      call 75
      drop
      local.get 2
      i32.const 16
      i32.add
      i32.const 8
      i32.add
      local.get 4
      i32.load
      local.tee 3
      i32.store
      local.get 2
      local.get 2
      i64.load offset=28 align=4
      local.tee 5
      i64.store offset=16
      local.get 1
      i32.const 8
      i32.add
      local.get 3
      i32.store
      local.get 1
      local.get 5
      i64.store align=4
    end
    local.get 1
    i64.load align=4
    local.set 5
    local.get 1
    i64.const 4294967296
    i64.store align=4
    local.get 2
    i32.const 8
    i32.add
    local.tee 3
    local.get 1
    i32.const 8
    i32.add
    local.tee 1
    i32.load
    i32.store
    local.get 1
    i32.const 0
    i32.store
    i32.const 0
    i32.load8_u offset=1049537
    drop
    local.get 2
    local.get 5
    i64.store
    block ;; label = @1
      i32.const 12
      i32.const 4
      call 17
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      local.get 2
      i64.load
      i64.store align=4
      local.get 1
      i32.const 8
      i32.add
      local.get 3
      i32.load
      i32.store
      local.get 0
      i32.const 1048912
      i32.store offset=4
      local.get 0
      local.get 1
      i32.store
      local.get 2
      i32.const 64
      i32.add
      global.set 0
      return
    end
    i32.const 4
    i32.const 12
    call 65
    unreachable
  )
  (func (;46;) (type 0) (param i32 i32)
    (local i32 i32 i32 i64)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    global.set 0
    block ;; label = @1
      local.get 1
      i32.load
      i32.const -2147483648
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=12
      local.set 3
      local.get 2
      i32.const 12
      i32.add
      i32.const 8
      i32.add
      local.tee 4
      i32.const 0
      i32.store
      local.get 2
      i64.const 4294967296
      i64.store offset=12 align=4
      local.get 2
      i32.const 24
      i32.add
      i32.const 16
      i32.add
      local.get 3
      i32.const 16
      i32.add
      i64.load align=4
      i64.store
      local.get 2
      i32.const 24
      i32.add
      i32.const 8
      i32.add
      local.get 3
      i32.const 8
      i32.add
      i64.load align=4
      i64.store
      local.get 2
      local.get 3
      i64.load align=4
      i64.store offset=24
      local.get 2
      i32.const 12
      i32.add
      i32.const 1048628
      local.get 2
      i32.const 24
      i32.add
      call 75
      drop
      local.get 2
      i32.const 8
      i32.add
      local.get 4
      i32.load
      local.tee 3
      i32.store
      local.get 2
      local.get 2
      i64.load offset=12 align=4
      local.tee 5
      i64.store
      local.get 1
      i32.const 8
      i32.add
      local.get 3
      i32.store
      local.get 1
      local.get 5
      i64.store align=4
    end
    local.get 0
    i32.const 1048912
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 48
    i32.add
    global.set 0
  )
  (func (;47;) (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load
        i32.const -2147483648
        i32.eq
        br_if 0 (;@2;)
        local.get 1
        local.get 0
        i32.load offset=4
        local.get 0
        i32.load offset=8
        call 82
        local.set 0
        br 1 (;@1;)
      end
      local.get 2
      i32.const 8
      i32.add
      i32.const 16
      i32.add
      local.get 0
      i32.load offset=12
      local.tee 0
      i32.const 16
      i32.add
      i64.load align=4
      i64.store
      local.get 2
      i32.const 8
      i32.add
      i32.const 8
      i32.add
      local.get 0
      i32.const 8
      i32.add
      i64.load align=4
      i64.store
      local.get 2
      local.get 0
      i64.load align=4
      i64.store offset=8
      local.get 1
      i32.load offset=20
      local.get 1
      i32.load offset=24
      local.get 2
      i32.const 8
      i32.add
      call 75
      local.set 0
    end
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 0
  )
  (func (;48;) (type 0) (param i32 i32)
    (local i32 i32)
    i32.const 0
    i32.load8_u offset=1049537
    drop
    local.get 1
    i32.load offset=4
    local.set 2
    local.get 1
    i32.load
    local.set 3
    block ;; label = @1
      i32.const 8
      i32.const 4
      call 17
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      local.get 2
      i32.store offset=4
      local.get 1
      local.get 3
      i32.store
      local.get 0
      i32.const 1048928
      i32.store offset=4
      local.get 0
      local.get 1
      i32.store
      return
    end
    i32.const 4
    i32.const 8
    call 65
    unreachable
  )
  (func (;49;) (type 0) (param i32 i32)
    local.get 0
    i32.const 1048928
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
  )
  (func (;50;) (type 0) (param i32 i32)
    local.get 0
    local.get 1
    i64.load align=4
    i64.store
  )
  (func (;51;) (type 2) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call 82
  )
  (func (;52;) (type 5) (param i32 i32 i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 5
    global.set 0
    i32.const 0
    i32.const 0
    i32.load offset=1049556
    local.tee 6
    i32.const 1
    i32.add
    i32.store offset=1049556
    block ;; label = @1
      local.get 6
      i32.const 0
      i32.lt_s
      br_if 0 (;@1;)
      block ;; label = @2
        i32.const 0
        i32.load8_u offset=1050016
        br_if 0 (;@2;)
        i32.const 0
        i32.const 1
        i32.store8 offset=1050016
        i32.const 0
        i32.const 0
        i32.load offset=1050012
        i32.const 1
        i32.add
        i32.store offset=1050012
        i32.const 0
        i32.load offset=1049544
        local.tee 6
        i32.const -1
        i32.le_s
        br_if 1 (;@1;)
        i32.const 0
        local.get 6
        i32.const 1
        i32.add
        i32.store offset=1049544
        block ;; label = @3
          i32.const 0
          i32.load offset=1049548
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          local.get 0
          local.get 1
          i32.load offset=20
          call_indirect (type 0)
          local.get 5
          local.get 4
          i32.store8 offset=29
          local.get 5
          local.get 3
          i32.store8 offset=28
          local.get 5
          local.get 2
          i32.store offset=24
          local.get 5
          local.get 5
          i64.load
          i64.store offset=16 align=4
          i32.const 0
          i32.load offset=1049548
          local.get 5
          i32.const 16
          i32.add
          i32.const 0
          i32.load offset=1049552
          i32.load offset=20
          call_indirect (type 0)
          i32.const 0
          i32.load offset=1049544
          i32.const -1
          i32.add
          local.set 6
        end
        i32.const 0
        local.get 6
        i32.store offset=1049544
        i32.const 0
        i32.const 0
        i32.store8 offset=1050016
        local.get 3
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        local.get 1
        call 53
        unreachable
      end
      local.get 5
      i32.const 8
      i32.add
      local.get 0
      local.get 1
      i32.load offset=24
      call_indirect (type 0)
    end
    unreachable
    unreachable
  )
  (func (;53;) (type 0) (param i32 i32)
    local.get 0
    local.get 1
    call 55
    drop
    unreachable
    unreachable
  )
  (func (;54;) (type 0) (param i32 i32)
    (local i32)
    local.get 1
    local.get 0
    i32.const 0
    i32.load offset=1049540
    local.tee 2
    i32.const 3
    local.get 2
    select
    call_indirect (type 0)
    unreachable
    unreachable
  )
  (func (;55;) (type 2) (param i32 i32) (result i32)
    unreachable
    unreachable
  )
  (func (;56;) (type 3) (param i32 i32 i32)
    (local i32)
    local.get 2
    i32.const 16
    i32.shr_u
    memory.grow
    local.set 3
    local.get 0
    i32.const 0
    i32.store offset=8
    local.get 0
    i32.const 0
    local.get 2
    i32.const -65536
    i32.and
    local.get 3
    i32.const -1
    i32.eq
    local.tee 2
    select
    i32.store offset=4
    local.get 0
    i32.const 0
    local.get 3
    i32.const 16
    i32.shl
    local.get 2
    select
    i32.store
  )
  (func (;57;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.const 1049008
    local.get 1
    call 75
  )
  (func (;58;) (type 4) (param i32)
    (local i32)
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 1
      i32.const 1
      call 18
    end
  )
  (func (;59;) (type 2) (param i32 i32) (result i32)
    local.get 1
    i32.const 1049000
    i32.const 5
    call 82
  )
  (func (;60;) (type 11)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    i32.const 0
    i32.store offset=24
    local.get 0
    i32.const 1
    i32.store offset=12
    local.get 0
    i32.const 1049052
    i32.store offset=8
    local.get 0
    i64.const 4
    i64.store offset=16 align=4
    local.get 0
    i32.const 8
    i32.add
    i32.const 1049088
    call 70
    unreachable
  )
  (func (;61;) (type 3) (param i32 i32 i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    block ;; label = @1
      local.get 1
      local.get 2
      i32.add
      local.tee 2
      local.get 1
      i32.ge_u
      br_if 0 (;@1;)
      i32.const 0
      i32.const 0
      call 62
      unreachable
    end
    i32.const 1
    local.set 4
    local.get 0
    i32.load
    local.tee 5
    i32.const 1
    i32.shl
    local.tee 1
    local.get 2
    local.get 1
    local.get 2
    i32.gt_u
    select
    local.tee 1
    i32.const 8
    local.get 1
    i32.const 8
    i32.gt_u
    select
    local.tee 1
    i32.const -1
    i32.xor
    i32.const 31
    i32.shr_u
    local.set 2
    block ;; label = @1
      block ;; label = @2
        local.get 5
        br_if 0 (;@2;)
        i32.const 0
        local.set 4
        br 1 (;@1;)
      end
      local.get 3
      local.get 5
      i32.store offset=28
      local.get 3
      local.get 0
      i32.load offset=4
      i32.store offset=20
    end
    local.get 3
    local.get 4
    i32.store offset=24
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    local.get 1
    local.get 3
    i32.const 20
    i32.add
    call 63
    block ;; label = @1
      local.get 3
      i32.load offset=8
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=12
      local.get 3
      i32.load offset=16
      call 62
      unreachable
    end
    local.get 3
    i32.load offset=12
    local.set 2
    local.get 0
    local.get 1
    i32.store
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 3
    i32.const 32
    i32.add
    global.set 0
  )
  (func (;62;) (type 0) (param i32 i32)
    block ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      call 60
      unreachable
    end
    local.get 0
    local.get 1
    call 65
    unreachable
  )
  (func (;63;) (type 6) (param i32 i32 i32 i32)
    (local i32 i32 i32)
    i32.const 1
    local.set 4
    i32.const 0
    local.set 5
    i32.const 4
    local.set 6
    block ;; label = @1
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.const 0
      i32.lt_s
      br_if 0 (;@1;)
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 3
                i32.load offset=4
                i32.eqz
                br_if 0 (;@6;)
                block ;; label = @7
                  local.get 3
                  i32.load offset=8
                  local.tee 4
                  br_if 0 (;@7;)
                  block ;; label = @8
                    local.get 2
                    br_if 0 (;@8;)
                    i32.const 1
                    local.set 4
                    br 4 (;@4;)
                  end
                  i32.const 0
                  i32.load8_u offset=1049537
                  drop
                  local.get 2
                  i32.const 1
                  call 17
                  local.set 4
                  br 2 (;@5;)
                end
                local.get 3
                i32.load
                local.get 4
                i32.const 1
                local.get 2
                call 19
                local.set 4
                br 1 (;@5;)
              end
              block ;; label = @6
                local.get 2
                br_if 0 (;@6;)
                i32.const 1
                local.set 4
                br 2 (;@4;)
              end
              i32.const 0
              i32.load8_u offset=1049537
              drop
              local.get 2
              i32.const 1
              call 17
              local.set 4
            end
            local.get 4
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 0
          local.get 4
          i32.store offset=4
          i32.const 0
          local.set 4
          br 1 (;@2;)
        end
        i32.const 1
        local.set 4
        local.get 0
        i32.const 1
        i32.store offset=4
      end
      i32.const 8
      local.set 6
      local.get 2
      local.set 5
    end
    local.get 0
    local.get 6
    i32.add
    local.get 5
    i32.store
    local.get 0
    local.get 4
    i32.store
  )
  (func (;64;) (type 4) (param i32)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 2
      i32.const -1
      i32.ne
      br_if 0 (;@1;)
      i32.const 0
      i32.const 0
      call 62
      unreachable
    end
    i32.const 1
    local.set 3
    local.get 2
    i32.const 1
    i32.shl
    local.tee 4
    local.get 2
    i32.const 1
    i32.add
    local.tee 5
    local.get 4
    local.get 5
    i32.gt_u
    select
    local.tee 4
    i32.const 8
    local.get 4
    i32.const 8
    i32.gt_u
    select
    local.tee 4
    i32.const -1
    i32.xor
    i32.const 31
    i32.shr_u
    local.set 5
    block ;; label = @1
      block ;; label = @2
        local.get 2
        br_if 0 (;@2;)
        i32.const 0
        local.set 3
        br 1 (;@1;)
      end
      local.get 1
      local.get 2
      i32.store offset=28
      local.get 1
      local.get 0
      i32.load offset=4
      i32.store offset=20
    end
    local.get 1
    local.get 3
    i32.store offset=24
    local.get 1
    i32.const 8
    i32.add
    local.get 5
    local.get 4
    local.get 1
    i32.const 20
    i32.add
    call 63
    block ;; label = @1
      local.get 1
      i32.load offset=8
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=12
      local.get 1
      i32.load offset=16
      call 62
      unreachable
    end
    local.get 1
    i32.load offset=12
    local.set 2
    local.get 0
    local.get 4
    i32.store
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 1
    i32.const 32
    i32.add
    global.set 0
  )
  (func (;65;) (type 0) (param i32 i32)
    local.get 1
    local.get 0
    call 20
    unreachable
  )
  (func (;66;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 1
              i32.load offset=4
              local.tee 3
              i32.eqz
              br_if 0 (;@5;)
              local.get 1
              i32.load
              local.set 4
              local.get 3
              i32.const 3
              i32.and
              local.set 5
              block ;; label = @6
                block ;; label = @7
                  local.get 3
                  i32.const 4
                  i32.ge_u
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 3
                  i32.const 0
                  local.set 6
                  br 1 (;@6;)
                end
                local.get 4
                i32.const 28
                i32.add
                local.set 7
                local.get 3
                i32.const -4
                i32.and
                local.set 8
                i32.const 0
                local.set 3
                i32.const 0
                local.set 6
                loop ;; label = @7
                  local.get 7
                  i32.load
                  local.get 7
                  i32.const -8
                  i32.add
                  i32.load
                  local.get 7
                  i32.const -16
                  i32.add
                  i32.load
                  local.get 7
                  i32.const -24
                  i32.add
                  i32.load
                  local.get 3
                  i32.add
                  i32.add
                  i32.add
                  i32.add
                  local.set 3
                  local.get 7
                  i32.const 32
                  i32.add
                  local.set 7
                  local.get 8
                  local.get 6
                  i32.const 4
                  i32.add
                  local.tee 6
                  i32.ne
                  br_if 0 (;@7;)
                end
              end
              block ;; label = @6
                local.get 5
                i32.eqz
                br_if 0 (;@6;)
                local.get 6
                i32.const 3
                i32.shl
                local.get 4
                i32.add
                i32.const 4
                i32.add
                local.set 7
                loop ;; label = @7
                  local.get 7
                  i32.load
                  local.get 3
                  i32.add
                  local.set 3
                  local.get 7
                  i32.const 8
                  i32.add
                  local.set 7
                  local.get 5
                  i32.const -1
                  i32.add
                  local.tee 5
                  br_if 0 (;@7;)
                end
              end
              block ;; label = @6
                local.get 1
                i32.load offset=12
                i32.eqz
                br_if 0 (;@6;)
                local.get 3
                i32.const 0
                i32.lt_s
                br_if 1 (;@5;)
                local.get 3
                i32.const 16
                i32.lt_u
                local.get 4
                i32.load offset=4
                i32.eqz
                i32.and
                br_if 1 (;@5;)
                local.get 3
                i32.const 1
                i32.shl
                local.set 3
              end
              local.get 3
              br_if 1 (;@4;)
            end
            i32.const 1
            local.set 7
            i32.const 0
            local.set 3
            br 1 (;@3;)
          end
          i32.const 0
          local.set 5
          local.get 3
          i32.const 0
          i32.lt_s
          br_if 1 (;@2;)
          i32.const 0
          i32.load8_u offset=1049537
          drop
          i32.const 1
          local.set 5
          local.get 3
          i32.const 1
          call 17
          local.tee 7
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 2
        i32.const 0
        i32.store offset=8
        local.get 2
        local.get 7
        i32.store offset=4
        local.get 2
        local.get 3
        i32.store
        local.get 2
        i32.const 1049008
        local.get 1
        call 75
        i32.eqz
        br_if 1 (;@1;)
        i32.const 1049120
        i32.const 86
        local.get 2
        i32.const 15
        i32.add
        i32.const 1049104
        i32.const 1049232
        call 76
        unreachable
      end
      local.get 5
      local.get 3
      call 62
      unreachable
    end
    local.get 0
    local.get 2
    i64.load align=4
    i64.store align=4
    local.get 0
    i32.const 8
    i32.add
    local.get 2
    i32.const 8
    i32.add
    i32.load
    i32.store
    local.get 2
    i32.const 16
    i32.add
    global.set 0
  )
  (func (;67;) (type 0) (param i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.const 128
            i32.lt_u
            br_if 0 (;@4;)
            local.get 2
            i32.const 0
            i32.store offset=12
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            block ;; label = @5
              local.get 1
              i32.const 65536
              i32.ge_u
              br_if 0 (;@5;)
              local.get 2
              local.get 1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 224
              i32.or
              i32.store8 offset=12
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 3
              local.set 1
              br 3 (;@2;)
            end
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=15
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            local.get 2
            local.get 1
            i32.const 18
            i32.shr_u
            i32.const 7
            i32.and
            i32.const 240
            i32.or
            i32.store8 offset=12
            i32.const 4
            local.set 1
            br 2 (;@2;)
          end
          block ;; label = @4
            local.get 0
            i32.load offset=8
            local.tee 3
            local.get 0
            i32.load
            i32.ne
            br_if 0 (;@4;)
            local.get 0
            call 64
          end
          local.get 0
          local.get 3
          i32.const 1
          i32.add
          i32.store offset=8
          local.get 0
          i32.load offset=4
          local.get 3
          i32.add
          local.get 1
          i32.store8
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        local.set 1
      end
      block ;; label = @2
        local.get 0
        i32.load
        local.get 0
        i32.load offset=8
        local.tee 3
        i32.sub
        local.get 1
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.get 3
        local.get 1
        call 61
        local.get 0
        i32.load offset=8
        local.set 3
      end
      local.get 0
      i32.load offset=4
      local.get 3
      i32.add
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      call 88
      drop
      local.get 0
      local.get 3
      local.get 1
      i32.add
      i32.store offset=8
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
  )
  (func (;68;) (type 1) (param i32 i32 i32) (result i32)
    (local i32)
    block ;; label = @1
      local.get 0
      i32.load
      local.get 0
      i32.load offset=8
      local.tee 3
      i32.sub
      local.get 2
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      local.get 3
      local.get 2
      call 61
      local.get 0
      i32.load offset=8
      local.set 3
    end
    local.get 0
    i32.load offset=4
    local.get 3
    i32.add
    local.get 1
    local.get 2
    call 88
    drop
    local.get 0
    local.get 3
    local.get 2
    i32.add
    i32.store offset=8
    i32.const 0
  )
  (func (;69;) (type 2) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call 67
    i32.const 0
  )
  (func (;70;) (type 0) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 16
    i32.add
    local.get 0
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    local.get 0
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 1
    i32.store16 offset=28
    local.get 2
    local.get 1
    i32.store offset=24
    local.get 2
    local.get 0
    i64.load align=4
    i64.store
    local.get 2
    call 44
    unreachable
  )
  (func (;71;) (type 3) (param i32 i32 i32)
    (local i32 i64)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 1
    i32.store offset=4
    local.get 3
    local.get 0
    i32.store
    local.get 3
    i32.const 2
    i32.store offset=12
    local.get 3
    i32.const 1049300
    i32.store offset=8
    local.get 3
    i64.const 2
    i64.store offset=20 align=4
    local.get 3
    i32.const 2
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.tee 4
    local.get 3
    i64.extend_i32_u
    i64.or
    i64.store offset=40
    local.get 3
    local.get 4
    local.get 3
    i32.const 4
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store offset=32
    local.get 3
    local.get 3
    i32.const 32
    i32.add
    i32.store offset=16
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call 70
    unreachable
  )
  (func (;72;) (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 3
      local.get 0
      i32.load offset=8
      local.tee 4
      i32.or
      i32.eqz
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 2
        i32.add
        local.set 5
        block ;; label = @3
          block ;; label = @4
            local.get 0
            i32.load offset=12
            local.tee 6
            br_if 0 (;@4;)
            i32.const 0
            local.set 7
            local.get 1
            local.set 8
            br 1 (;@3;)
          end
          i32.const 0
          local.set 7
          local.get 1
          local.set 8
          loop ;; label = @4
            local.get 8
            local.tee 4
            local.get 5
            i32.eq
            br_if 2 (;@2;)
            block ;; label = @5
              block ;; label = @6
                local.get 4
                i32.load8_s
                local.tee 8
                i32.const -1
                i32.le_s
                br_if 0 (;@6;)
                local.get 4
                i32.const 1
                i32.add
                local.set 8
                br 1 (;@5;)
              end
              block ;; label = @6
                local.get 8
                i32.const -32
                i32.ge_u
                br_if 0 (;@6;)
                local.get 4
                i32.const 2
                i32.add
                local.set 8
                br 1 (;@5;)
              end
              block ;; label = @6
                local.get 8
                i32.const -16
                i32.ge_u
                br_if 0 (;@6;)
                local.get 4
                i32.const 3
                i32.add
                local.set 8
                br 1 (;@5;)
              end
              local.get 4
              i32.const 4
              i32.add
              local.set 8
            end
            local.get 8
            local.get 4
            i32.sub
            local.get 7
            i32.add
            local.set 7
            local.get 6
            i32.const -1
            i32.add
            local.tee 6
            br_if 0 (;@4;)
          end
        end
        local.get 8
        local.get 5
        i32.eq
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 8
          i32.load8_s
          local.tee 4
          i32.const -1
          i32.gt_s
          br_if 0 (;@3;)
          local.get 4
          i32.const -32
          i32.lt_u
          drop
        end
        block ;; label = @3
          block ;; label = @4
            local.get 7
            i32.eqz
            br_if 0 (;@4;)
            block ;; label = @5
              local.get 7
              local.get 2
              i32.ge_u
              br_if 0 (;@5;)
              i32.const 0
              local.set 4
              local.get 1
              local.get 7
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              br_if 1 (;@4;)
              br 2 (;@3;)
            end
            i32.const 0
            local.set 4
            local.get 7
            local.get 2
            i32.ne
            br_if 1 (;@3;)
          end
          local.get 1
          local.set 4
        end
        local.get 7
        local.get 2
        local.get 4
        select
        local.set 2
        local.get 4
        local.get 1
        local.get 4
        select
        local.set 1
      end
      block ;; label = @2
        local.get 3
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=20
        local.get 1
        local.get 2
        local.get 0
        i32.load offset=24
        i32.load offset=12
        call_indirect (type 1)
        return
      end
      local.get 0
      i32.load offset=4
      local.set 3
      block ;; label = @2
        block ;; label = @3
          local.get 2
          i32.const 16
          i32.lt_u
          br_if 0 (;@3;)
          local.get 1
          local.get 2
          call 80
          local.set 4
          br 1 (;@2;)
        end
        block ;; label = @3
          local.get 2
          br_if 0 (;@3;)
          i32.const 0
          local.set 4
          br 1 (;@2;)
        end
        local.get 2
        i32.const 3
        i32.and
        local.set 6
        block ;; label = @3
          block ;; label = @4
            local.get 2
            i32.const 4
            i32.ge_u
            br_if 0 (;@4;)
            i32.const 0
            local.set 4
            i32.const 0
            local.set 7
            br 1 (;@3;)
          end
          local.get 2
          i32.const 12
          i32.and
          local.set 5
          i32.const 0
          local.set 4
          i32.const 0
          local.set 7
          loop ;; label = @4
            local.get 4
            local.get 1
            local.get 7
            i32.add
            local.tee 8
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 8
            i32.const 1
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 8
            i32.const 2
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 8
            i32.const 3
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set 4
            local.get 5
            local.get 7
            i32.const 4
            i32.add
            local.tee 7
            i32.ne
            br_if 0 (;@4;)
          end
        end
        local.get 6
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 7
        i32.add
        local.set 8
        loop ;; label = @3
          local.get 4
          local.get 8
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.set 4
          local.get 8
          i32.const 1
          i32.add
          local.set 8
          local.get 6
          i32.const -1
          i32.add
          local.tee 6
          br_if 0 (;@3;)
        end
      end
      block ;; label = @2
        block ;; label = @3
          local.get 3
          local.get 4
          i32.le_u
          br_if 0 (;@3;)
          local.get 3
          local.get 4
          i32.sub
          local.set 5
          i32.const 0
          local.set 4
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 0
                i32.load8_u offset=32
                br_table 2 (;@4;) 0 (;@6;) 1 (;@5;) 2 (;@4;) 2 (;@4;)
              end
              local.get 5
              local.set 4
              i32.const 0
              local.set 5
              br 1 (;@4;)
            end
            local.get 5
            i32.const 1
            i32.shr_u
            local.set 4
            local.get 5
            i32.const 1
            i32.add
            i32.const 1
            i32.shr_u
            local.set 5
          end
          local.get 4
          i32.const 1
          i32.add
          local.set 4
          local.get 0
          i32.load offset=16
          local.set 6
          local.get 0
          i32.load offset=24
          local.set 8
          local.get 0
          i32.load offset=20
          local.set 7
          loop ;; label = @4
            local.get 4
            i32.const -1
            i32.add
            local.tee 4
            i32.eqz
            br_if 2 (;@2;)
            local.get 7
            local.get 6
            local.get 8
            i32.load offset=16
            call_indirect (type 2)
            i32.eqz
            br_if 0 (;@4;)
          end
          i32.const 1
          return
        end
        local.get 0
        i32.load offset=20
        local.get 1
        local.get 2
        local.get 0
        i32.load offset=24
        i32.load offset=12
        call_indirect (type 1)
        return
      end
      i32.const 1
      local.set 4
      block ;; label = @2
        local.get 7
        local.get 1
        local.get 2
        local.get 8
        i32.load offset=12
        call_indirect (type 1)
        br_if 0 (;@2;)
        i32.const 0
        local.set 4
        block ;; label = @3
          loop ;; label = @4
            block ;; label = @5
              local.get 5
              local.get 4
              i32.ne
              br_if 0 (;@5;)
              local.get 5
              local.set 4
              br 2 (;@3;)
            end
            local.get 4
            i32.const 1
            i32.add
            local.set 4
            local.get 7
            local.get 6
            local.get 8
            i32.load offset=16
            call_indirect (type 2)
            i32.eqz
            br_if 0 (;@4;)
          end
          local.get 4
          i32.const -1
          i32.add
          local.set 4
        end
        local.get 4
        local.get 5
        i32.lt_u
        local.set 4
      end
      local.get 4
      return
    end
    local.get 0
    i32.load offset=20
    local.get 1
    local.get 2
    local.get 0
    i32.load offset=24
    i32.load offset=12
    call_indirect (type 1)
  )
  (func (;73;) (type 3) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 0
    i32.store offset=16
    local.get 3
    i32.const 1
    i32.store offset=4
    local.get 3
    i64.const 4
    i64.store offset=8 align=4
    local.get 3
    local.get 1
    i32.store offset=28
    local.get 3
    local.get 0
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 24
    i32.add
    i32.store
    local.get 3
    local.get 2
    call 70
    unreachable
  )
  (func (;74;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i64.load32_u
    i32.const 1
    local.get 1
    call 83
  )
  (func (;75;) (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 3
    i32.store8 offset=44
    local.get 3
    i32.const 32
    i32.store offset=28
    i32.const 0
    local.set 4
    local.get 3
    i32.const 0
    i32.store offset=40
    local.get 3
    local.get 1
    i32.store offset=36
    local.get 3
    local.get 0
    i32.store offset=32
    local.get 3
    i32.const 0
    i32.store offset=20
    local.get 3
    i32.const 0
    i32.store offset=12
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 2
              i32.load offset=16
              local.tee 5
              br_if 0 (;@5;)
              local.get 2
              i32.load offset=12
              local.tee 0
              i32.eqz
              br_if 1 (;@4;)
              local.get 2
              i32.load offset=8
              local.set 1
              local.get 0
              i32.const 3
              i32.shl
              local.set 6
              local.get 0
              i32.const -1
              i32.add
              i32.const 536870911
              i32.and
              i32.const 1
              i32.add
              local.set 4
              local.get 2
              i32.load
              local.set 0
              loop ;; label = @6
                block ;; label = @7
                  local.get 0
                  i32.const 4
                  i32.add
                  i32.load
                  local.tee 7
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 3
                  i32.load offset=32
                  local.get 0
                  i32.load
                  local.get 7
                  local.get 3
                  i32.load offset=36
                  i32.load offset=12
                  call_indirect (type 1)
                  br_if 4 (;@3;)
                end
                local.get 1
                i32.load
                local.get 3
                i32.const 12
                i32.add
                local.get 1
                i32.load offset=4
                call_indirect (type 2)
                br_if 3 (;@3;)
                local.get 1
                i32.const 8
                i32.add
                local.set 1
                local.get 0
                i32.const 8
                i32.add
                local.set 0
                local.get 6
                i32.const -8
                i32.add
                local.tee 6
                br_if 0 (;@6;)
                br 2 (;@4;)
              end
            end
            local.get 2
            i32.load offset=20
            local.tee 1
            i32.eqz
            br_if 0 (;@4;)
            local.get 1
            i32.const 5
            i32.shl
            local.set 8
            local.get 1
            i32.const -1
            i32.add
            i32.const 134217727
            i32.and
            i32.const 1
            i32.add
            local.set 4
            local.get 2
            i32.load offset=8
            local.set 9
            local.get 2
            i32.load
            local.set 0
            i32.const 0
            local.set 6
            loop ;; label = @5
              block ;; label = @6
                local.get 0
                i32.const 4
                i32.add
                i32.load
                local.tee 1
                i32.eqz
                br_if 0 (;@6;)
                local.get 3
                i32.load offset=32
                local.get 0
                i32.load
                local.get 1
                local.get 3
                i32.load offset=36
                i32.load offset=12
                call_indirect (type 1)
                br_if 3 (;@3;)
              end
              local.get 3
              local.get 5
              local.get 6
              i32.add
              local.tee 1
              i32.const 16
              i32.add
              i32.load
              i32.store offset=28
              local.get 3
              local.get 1
              i32.const 28
              i32.add
              i32.load8_u
              i32.store8 offset=44
              local.get 3
              local.get 1
              i32.const 24
              i32.add
              i32.load
              i32.store offset=40
              local.get 1
              i32.const 12
              i32.add
              i32.load
              local.set 7
              i32.const 0
              local.set 10
              i32.const 0
              local.set 11
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 1
                    i32.const 8
                    i32.add
                    i32.load
                    br_table 1 (;@7;) 0 (;@8;) 2 (;@6;) 1 (;@7;)
                  end
                  local.get 7
                  i32.const 3
                  i32.shl
                  local.set 12
                  i32.const 0
                  local.set 11
                  local.get 9
                  local.get 12
                  i32.add
                  local.tee 12
                  i32.load offset=4
                  br_if 1 (;@6;)
                  local.get 12
                  i32.load
                  local.set 7
                end
                i32.const 1
                local.set 11
              end
              local.get 3
              local.get 7
              i32.store offset=16
              local.get 3
              local.get 11
              i32.store offset=12
              local.get 1
              i32.const 4
              i32.add
              i32.load
              local.set 7
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 1
                    i32.load
                    br_table 1 (;@7;) 0 (;@8;) 2 (;@6;) 1 (;@7;)
                  end
                  local.get 7
                  i32.const 3
                  i32.shl
                  local.set 11
                  local.get 9
                  local.get 11
                  i32.add
                  local.tee 11
                  i32.load offset=4
                  br_if 1 (;@6;)
                  local.get 11
                  i32.load
                  local.set 7
                end
                i32.const 1
                local.set 10
              end
              local.get 3
              local.get 7
              i32.store offset=24
              local.get 3
              local.get 10
              i32.store offset=20
              local.get 9
              local.get 1
              i32.const 20
              i32.add
              i32.load
              i32.const 3
              i32.shl
              i32.add
              local.tee 1
              i32.load
              local.get 3
              i32.const 12
              i32.add
              local.get 1
              i32.load offset=4
              call_indirect (type 2)
              br_if 2 (;@3;)
              local.get 0
              i32.const 8
              i32.add
              local.set 0
              local.get 8
              local.get 6
              i32.const 32
              i32.add
              local.tee 6
              i32.ne
              br_if 0 (;@5;)
            end
          end
          local.get 4
          local.get 2
          i32.load offset=4
          i32.ge_u
          br_if 1 (;@2;)
          local.get 3
          i32.load offset=32
          local.get 2
          i32.load
          local.get 4
          i32.const 3
          i32.shl
          i32.add
          local.tee 1
          i32.load
          local.get 1
          i32.load offset=4
          local.get 3
          i32.load offset=36
          i32.load offset=12
          call_indirect (type 1)
          i32.eqz
          br_if 1 (;@2;)
        end
        i32.const 1
        local.set 1
        br 1 (;@1;)
      end
      i32.const 0
      local.set 1
    end
    local.get 3
    i32.const 48
    i32.add
    global.set 0
    local.get 1
  )
  (func (;76;) (type 5) (param i32 i32 i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 5
    global.set 0
    local.get 5
    local.get 1
    i32.store offset=12
    local.get 5
    local.get 0
    i32.store offset=8
    local.get 5
    local.get 3
    i32.store offset=20
    local.get 5
    local.get 2
    i32.store offset=16
    local.get 5
    i32.const 2
    i32.store offset=28
    local.get 5
    i32.const 1049320
    i32.store offset=24
    local.get 5
    i64.const 2
    i64.store offset=36 align=4
    local.get 5
    i32.const 24
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.get 5
    i32.const 16
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store offset=56
    local.get 5
    i32.const 25
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.get 5
    i32.const 8
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store offset=48
    local.get 5
    local.get 5
    i32.const 48
    i32.add
    i32.store offset=32
    local.get 5
    i32.const 24
    i32.add
    local.get 4
    call 70
    unreachable
  )
  (func (;77;) (type 2) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call 72
  )
  (func (;78;) (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 2)
  )
  (func (;79;) (type 12) (param i32 i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 1
        br_if 0 (;@2;)
        local.get 5
        i32.const 1
        i32.add
        local.set 6
        local.get 0
        i32.load offset=28
        local.set 7
        i32.const 45
        local.set 8
        br 1 (;@1;)
      end
      i32.const 43
      i32.const 1114112
      local.get 0
      i32.load offset=28
      local.tee 7
      i32.const 1
      i32.and
      local.tee 1
      select
      local.set 8
      local.get 1
      local.get 5
      i32.add
      local.set 6
    end
    block ;; label = @1
      block ;; label = @2
        local.get 7
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.const 16
          i32.lt_u
          br_if 0 (;@3;)
          local.get 2
          local.get 3
          call 80
          local.set 1
          br 1 (;@2;)
        end
        block ;; label = @3
          local.get 3
          br_if 0 (;@3;)
          i32.const 0
          local.set 1
          br 1 (;@2;)
        end
        local.get 3
        i32.const 3
        i32.and
        local.set 9
        block ;; label = @3
          block ;; label = @4
            local.get 3
            i32.const 4
            i32.ge_u
            br_if 0 (;@4;)
            i32.const 0
            local.set 1
            i32.const 0
            local.set 10
            br 1 (;@3;)
          end
          local.get 3
          i32.const 12
          i32.and
          local.set 11
          i32.const 0
          local.set 1
          i32.const 0
          local.set 10
          loop ;; label = @4
            local.get 1
            local.get 2
            local.get 10
            i32.add
            local.tee 12
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 12
            i32.const 1
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 12
            i32.const 2
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 12
            i32.const 3
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set 1
            local.get 11
            local.get 10
            i32.const 4
            i32.add
            local.tee 10
            i32.ne
            br_if 0 (;@4;)
          end
        end
        local.get 9
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 10
        i32.add
        local.set 12
        loop ;; label = @3
          local.get 1
          local.get 12
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.set 1
          local.get 12
          i32.const 1
          i32.add
          local.set 12
          local.get 9
          i32.const -1
          i32.add
          local.tee 9
          br_if 0 (;@3;)
        end
      end
      local.get 1
      local.get 6
      i32.add
      local.set 6
    end
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load
        br_if 0 (;@2;)
        i32.const 1
        local.set 1
        local.get 0
        i32.load offset=20
        local.tee 12
        local.get 0
        i32.load offset=24
        local.tee 10
        local.get 8
        local.get 2
        local.get 3
        call 81
        br_if 1 (;@1;)
        local.get 12
        local.get 4
        local.get 5
        local.get 10
        i32.load offset=12
        call_indirect (type 1)
        return
      end
      block ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 9
        local.get 6
        i32.gt_u
        br_if 0 (;@2;)
        i32.const 1
        local.set 1
        local.get 0
        i32.load offset=20
        local.tee 12
        local.get 0
        i32.load offset=24
        local.tee 10
        local.get 8
        local.get 2
        local.get 3
        call 81
        br_if 1 (;@1;)
        local.get 12
        local.get 4
        local.get 5
        local.get 10
        i32.load offset=12
        call_indirect (type 1)
        return
      end
      block ;; label = @2
        local.get 7
        i32.const 8
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=16
        local.set 11
        local.get 0
        i32.const 48
        i32.store offset=16
        local.get 0
        i32.load8_u offset=32
        local.set 7
        i32.const 1
        local.set 1
        local.get 0
        i32.const 1
        i32.store8 offset=32
        local.get 0
        i32.load offset=20
        local.tee 12
        local.get 0
        i32.load offset=24
        local.tee 10
        local.get 8
        local.get 2
        local.get 3
        call 81
        br_if 1 (;@1;)
        local.get 9
        local.get 6
        i32.sub
        i32.const 1
        i32.add
        local.set 1
        block ;; label = @3
          loop ;; label = @4
            local.get 1
            i32.const -1
            i32.add
            local.tee 1
            i32.eqz
            br_if 1 (;@3;)
            local.get 12
            i32.const 48
            local.get 10
            i32.load offset=16
            call_indirect (type 2)
            i32.eqz
            br_if 0 (;@4;)
          end
          i32.const 1
          return
        end
        i32.const 1
        local.set 1
        local.get 12
        local.get 4
        local.get 5
        local.get 10
        i32.load offset=12
        call_indirect (type 1)
        br_if 1 (;@1;)
        local.get 0
        local.get 7
        i32.store8 offset=32
        local.get 0
        local.get 11
        i32.store offset=16
        i32.const 0
        local.set 1
        br 1 (;@1;)
      end
      local.get 9
      local.get 6
      i32.sub
      local.set 6
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 0
            i32.load8_u offset=32
            local.tee 1
            br_table 2 (;@2;) 0 (;@4;) 1 (;@3;) 0 (;@4;) 2 (;@2;)
          end
          local.get 6
          local.set 1
          i32.const 0
          local.set 6
          br 1 (;@2;)
        end
        local.get 6
        i32.const 1
        i32.shr_u
        local.set 1
        local.get 6
        i32.const 1
        i32.add
        i32.const 1
        i32.shr_u
        local.set 6
      end
      local.get 1
      i32.const 1
      i32.add
      local.set 1
      local.get 0
      i32.load offset=16
      local.set 9
      local.get 0
      i32.load offset=24
      local.set 12
      local.get 0
      i32.load offset=20
      local.set 10
      block ;; label = @2
        loop ;; label = @3
          local.get 1
          i32.const -1
          i32.add
          local.tee 1
          i32.eqz
          br_if 1 (;@2;)
          local.get 10
          local.get 9
          local.get 12
          i32.load offset=16
          call_indirect (type 2)
          i32.eqz
          br_if 0 (;@3;)
        end
        i32.const 1
        return
      end
      i32.const 1
      local.set 1
      local.get 10
      local.get 12
      local.get 8
      local.get 2
      local.get 3
      call 81
      br_if 0 (;@1;)
      local.get 10
      local.get 4
      local.get 5
      local.get 12
      i32.load offset=12
      call_indirect (type 1)
      br_if 0 (;@1;)
      i32.const 0
      local.set 1
      loop ;; label = @2
        block ;; label = @3
          local.get 6
          local.get 1
          i32.ne
          br_if 0 (;@3;)
          local.get 6
          local.get 6
          i32.lt_u
          return
        end
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 10
        local.get 9
        local.get 12
        i32.load offset=16
        call_indirect (type 2)
        i32.eqz
        br_if 0 (;@2;)
      end
      local.get 1
      i32.const -1
      i32.add
      local.get 6
      i32.lt_u
      return
    end
    local.get 1
  )
  (func (;80;) (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 1
        local.get 0
        i32.const 3
        i32.add
        i32.const -4
        i32.and
        local.tee 2
        local.get 0
        i32.sub
        local.tee 3
        i32.lt_u
        br_if 0 (;@2;)
        local.get 1
        local.get 3
        i32.sub
        local.tee 4
        i32.const 4
        i32.lt_u
        br_if 0 (;@2;)
        local.get 4
        i32.const 3
        i32.and
        local.set 5
        i32.const 0
        local.set 6
        i32.const 0
        local.set 1
        block ;; label = @3
          local.get 2
          local.get 0
          i32.eq
          local.tee 7
          br_if 0 (;@3;)
          i32.const 0
          local.set 1
          block ;; label = @4
            block ;; label = @5
              local.get 0
              local.get 2
              i32.sub
              local.tee 8
              i32.const -4
              i32.le_u
              br_if 0 (;@5;)
              i32.const 0
              local.set 9
              br 1 (;@4;)
            end
            i32.const 0
            local.set 9
            loop ;; label = @5
              local.get 1
              local.get 0
              local.get 9
              i32.add
              local.tee 2
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 2
              i32.const 1
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 2
              i32.const 2
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 2
              i32.const 3
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.set 1
              local.get 9
              i32.const 4
              i32.add
              local.tee 9
              br_if 0 (;@5;)
            end
          end
          local.get 7
          br_if 0 (;@3;)
          local.get 0
          local.get 9
          i32.add
          local.set 2
          loop ;; label = @4
            local.get 1
            local.get 2
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set 1
            local.get 2
            i32.const 1
            i32.add
            local.set 2
            local.get 8
            i32.const 1
            i32.add
            local.tee 8
            br_if 0 (;@4;)
          end
        end
        local.get 0
        local.get 3
        i32.add
        local.set 9
        block ;; label = @3
          local.get 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 9
          local.get 4
          i32.const -4
          i32.and
          i32.add
          local.tee 2
          i32.load8_s
          i32.const -65
          i32.gt_s
          local.set 6
          local.get 5
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          local.get 6
          local.get 2
          i32.load8_s offset=1
          i32.const -65
          i32.gt_s
          i32.add
          local.set 6
          local.get 5
          i32.const 2
          i32.eq
          br_if 0 (;@3;)
          local.get 6
          local.get 2
          i32.load8_s offset=2
          i32.const -65
          i32.gt_s
          i32.add
          local.set 6
        end
        local.get 4
        i32.const 2
        i32.shr_u
        local.set 3
        local.get 6
        local.get 1
        i32.add
        local.set 8
        loop ;; label = @3
          local.get 9
          local.set 4
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          i32.const 192
          local.get 3
          i32.const 192
          i32.lt_u
          select
          local.tee 6
          i32.const 3
          i32.and
          local.set 7
          local.get 6
          i32.const 2
          i32.shl
          local.set 5
          i32.const 0
          local.set 2
          block ;; label = @4
            local.get 3
            i32.const 4
            i32.lt_u
            br_if 0 (;@4;)
            local.get 4
            local.get 5
            i32.const 1008
            i32.and
            i32.add
            local.set 0
            i32.const 0
            local.set 2
            local.get 4
            local.set 1
            loop ;; label = @5
              local.get 1
              i32.load offset=12
              local.tee 9
              i32.const -1
              i32.xor
              i32.const 7
              i32.shr_u
              local.get 9
              i32.const 6
              i32.shr_u
              i32.or
              i32.const 16843009
              i32.and
              local.get 1
              i32.load offset=8
              local.tee 9
              i32.const -1
              i32.xor
              i32.const 7
              i32.shr_u
              local.get 9
              i32.const 6
              i32.shr_u
              i32.or
              i32.const 16843009
              i32.and
              local.get 1
              i32.load offset=4
              local.tee 9
              i32.const -1
              i32.xor
              i32.const 7
              i32.shr_u
              local.get 9
              i32.const 6
              i32.shr_u
              i32.or
              i32.const 16843009
              i32.and
              local.get 1
              i32.load
              local.tee 9
              i32.const -1
              i32.xor
              i32.const 7
              i32.shr_u
              local.get 9
              i32.const 6
              i32.shr_u
              i32.or
              i32.const 16843009
              i32.and
              local.get 2
              i32.add
              i32.add
              i32.add
              i32.add
              local.set 2
              local.get 1
              i32.const 16
              i32.add
              local.tee 1
              local.get 0
              i32.ne
              br_if 0 (;@5;)
            end
          end
          local.get 3
          local.get 6
          i32.sub
          local.set 3
          local.get 4
          local.get 5
          i32.add
          local.set 9
          local.get 2
          i32.const 8
          i32.shr_u
          i32.const 16711935
          i32.and
          local.get 2
          i32.const 16711935
          i32.and
          i32.add
          i32.const 65537
          i32.mul
          i32.const 16
          i32.shr_u
          local.get 8
          i32.add
          local.set 8
          local.get 7
          i32.eqz
          br_if 0 (;@3;)
        end
        local.get 4
        local.get 6
        i32.const 252
        i32.and
        i32.const 2
        i32.shl
        i32.add
        local.tee 2
        i32.load
        local.tee 1
        i32.const -1
        i32.xor
        i32.const 7
        i32.shr_u
        local.get 1
        i32.const 6
        i32.shr_u
        i32.or
        i32.const 16843009
        i32.and
        local.set 1
        block ;; label = @3
          local.get 7
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          local.get 2
          i32.load offset=4
          local.tee 9
          i32.const -1
          i32.xor
          i32.const 7
          i32.shr_u
          local.get 9
          i32.const 6
          i32.shr_u
          i32.or
          i32.const 16843009
          i32.and
          local.get 1
          i32.add
          local.set 1
          local.get 7
          i32.const 2
          i32.eq
          br_if 0 (;@3;)
          local.get 2
          i32.load offset=8
          local.tee 2
          i32.const -1
          i32.xor
          i32.const 7
          i32.shr_u
          local.get 2
          i32.const 6
          i32.shr_u
          i32.or
          i32.const 16843009
          i32.and
          local.get 1
          i32.add
          local.set 1
        end
        local.get 1
        i32.const 8
        i32.shr_u
        i32.const 459007
        i32.and
        local.get 1
        i32.const 16711935
        i32.and
        i32.add
        i32.const 65537
        i32.mul
        i32.const 16
        i32.shr_u
        local.get 8
        i32.add
        return
      end
      block ;; label = @2
        local.get 1
        br_if 0 (;@2;)
        i32.const 0
        return
      end
      local.get 1
      i32.const 3
      i32.and
      local.set 9
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.const 4
          i32.ge_u
          br_if 0 (;@3;)
          i32.const 0
          local.set 8
          i32.const 0
          local.set 2
          br 1 (;@2;)
        end
        local.get 1
        i32.const -4
        i32.and
        local.set 3
        i32.const 0
        local.set 8
        i32.const 0
        local.set 2
        loop ;; label = @3
          local.get 8
          local.get 0
          local.get 2
          i32.add
          local.tee 1
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.get 1
          i32.const 1
          i32.add
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.get 1
          i32.const 2
          i32.add
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.get 1
          i32.const 3
          i32.add
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.set 8
          local.get 3
          local.get 2
          i32.const 4
          i32.add
          local.tee 2
          i32.ne
          br_if 0 (;@3;)
        end
      end
      local.get 9
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      i32.add
      local.set 1
      loop ;; label = @2
        local.get 8
        local.get 1
        i32.load8_s
        i32.const -65
        i32.gt_s
        i32.add
        local.set 8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 9
        i32.const -1
        i32.add
        local.tee 9
        br_if 0 (;@2;)
      end
    end
    local.get 8
  )
  (func (;81;) (type 13) (param i32 i32 i32 i32 i32) (result i32)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 2
          i32.const 1114112
          i32.eq
          br_if 0 (;@3;)
          i32.const 1
          local.set 5
          local.get 0
          local.get 2
          local.get 1
          i32.load offset=16
          call_indirect (type 2)
          br_if 1 (;@2;)
        end
        local.get 3
        br_if 1 (;@1;)
        i32.const 0
        local.set 5
      end
      local.get 5
      return
    end
    local.get 0
    local.get 3
    local.get 4
    local.get 1
    i32.load offset=12
    call_indirect (type 1)
  )
  (func (;82;) (type 1) (param i32 i32 i32) (result i32)
    local.get 0
    i32.load offset=20
    local.get 1
    local.get 2
    local.get 0
    i32.load offset=24
    i32.load offset=12
    call_indirect (type 1)
  )
  (func (;83;) (type 14) (param i64 i32 i32) (result i32)
    (local i32 i32 i64 i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    i32.const 39
    local.set 4
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i64.const 10000
        i64.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.set 5
        br 1 (;@1;)
      end
      i32.const 39
      local.set 4
      loop ;; label = @2
        local.get 3
        i32.const 9
        i32.add
        local.get 4
        i32.add
        local.tee 6
        i32.const -4
        i32.add
        local.get 0
        local.get 0
        i64.const 10000
        i64.div_u
        local.tee 5
        i64.const 10000
        i64.mul
        i64.sub
        i32.wrap_i64
        local.tee 7
        i32.const 65535
        i32.and
        i32.const 100
        i32.div_u
        local.tee 8
        i32.const 1
        i32.shl
        i32.const 1049336
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 6
        i32.const -2
        i32.add
        local.get 7
        local.get 8
        i32.const 100
        i32.mul
        i32.sub
        i32.const 65535
        i32.and
        i32.const 1
        i32.shl
        i32.const 1049336
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 4
        i32.const -4
        i32.add
        local.set 4
        local.get 0
        i64.const 99999999
        i64.gt_u
        local.set 6
        local.get 5
        local.set 0
        local.get 6
        br_if 0 (;@2;)
      end
    end
    block ;; label = @1
      local.get 5
      i32.wrap_i64
      local.tee 6
      i32.const 99
      i32.le_u
      br_if 0 (;@1;)
      local.get 3
      i32.const 9
      i32.add
      local.get 4
      i32.const -2
      i32.add
      local.tee 4
      i32.add
      local.get 5
      i32.wrap_i64
      local.tee 6
      local.get 6
      i32.const 65535
      i32.and
      i32.const 100
      i32.div_u
      local.tee 6
      i32.const 100
      i32.mul
      i32.sub
      i32.const 65535
      i32.and
      i32.const 1
      i32.shl
      i32.const 1049336
      i32.add
      i32.load16_u align=1
      i32.store16 align=1
    end
    block ;; label = @1
      block ;; label = @2
        local.get 6
        i32.const 10
        i32.lt_u
        br_if 0 (;@2;)
        local.get 3
        i32.const 9
        i32.add
        local.get 4
        i32.const -2
        i32.add
        local.tee 4
        i32.add
        local.get 6
        i32.const 1
        i32.shl
        i32.const 1049336
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        br 1 (;@1;)
      end
      local.get 3
      i32.const 9
      i32.add
      local.get 4
      i32.const -1
      i32.add
      local.tee 4
      i32.add
      local.get 6
      i32.const 48
      i32.or
      i32.store8
    end
    local.get 2
    local.get 1
    i32.const 1
    i32.const 0
    local.get 3
    i32.const 9
    i32.add
    local.get 4
    i32.add
    i32.const 39
    local.get 4
    i32.sub
    call 79
    local.set 4
    local.get 3
    i32.const 48
    i32.add
    global.set 0
    local.get 4
  )
  (func (;84;) (type 2) (param i32 i32) (result i32)
    (local i32)
    local.get 0
    i32.load
    local.tee 0
    local.get 0
    i32.const 31
    i32.shr_s
    local.tee 2
    i32.xor
    local.get 2
    i32.sub
    i64.extend_i32_u
    local.get 0
    i32.const -1
    i32.xor
    i32.const 31
    i32.shr_u
    local.get 1
    call 83
  )
  (func (;85;) (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.const 16
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      local.get 0
      i32.sub
      i32.const 3
      i32.and
      local.tee 4
      i32.add
      local.set 5
      block ;; label = @2
        local.get 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        local.get 1
        local.set 6
        loop ;; label = @3
          local.get 3
          local.get 6
          i32.load8_u
          i32.store8
          local.get 6
          i32.const 1
          i32.add
          local.set 6
          local.get 3
          i32.const 1
          i32.add
          local.tee 3
          local.get 5
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 5
      local.get 2
      local.get 4
      i32.sub
      local.tee 7
      i32.const -4
      i32.and
      local.tee 8
      i32.add
      local.set 3
      block ;; label = @2
        block ;; label = @3
          local.get 1
          local.get 4
          i32.add
          local.tee 9
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 8
          i32.const 1
          i32.lt_s
          br_if 1 (;@2;)
          local.get 9
          i32.const 3
          i32.shl
          local.tee 6
          i32.const 24
          i32.and
          local.set 2
          local.get 9
          i32.const -4
          i32.and
          local.tee 10
          i32.const 4
          i32.add
          local.set 1
          i32.const 0
          local.get 6
          i32.sub
          i32.const 24
          i32.and
          local.set 4
          local.get 10
          i32.load
          local.set 6
          loop ;; label = @4
            local.get 5
            local.get 6
            local.get 2
            i32.shr_u
            local.get 1
            i32.load
            local.tee 6
            local.get 4
            i32.shl
            i32.or
            i32.store
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            local.get 5
            i32.const 4
            i32.add
            local.tee 5
            local.get 3
            i32.lt_u
            br_if 0 (;@4;)
            br 2 (;@2;)
          end
        end
        local.get 8
        i32.const 1
        i32.lt_s
        br_if 0 (;@2;)
        local.get 9
        local.set 1
        loop ;; label = @3
          local.get 5
          local.get 1
          i32.load
          i32.store
          local.get 1
          i32.const 4
          i32.add
          local.set 1
          local.get 5
          i32.const 4
          i32.add
          local.tee 5
          local.get 3
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 7
      i32.const 3
      i32.and
      local.set 2
      local.get 9
      local.get 8
      i32.add
      local.set 1
    end
    block ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      local.get 2
      i32.add
      local.set 5
      loop ;; label = @2
        local.get 3
        local.get 1
        i32.load8_u
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 3
        i32.const 1
        i32.add
        local.tee 3
        local.get 5
        i32.lt_u
        br_if 0 (;@2;)
      end
    end
    local.get 0
  )
  (func (;86;) (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 0
            local.get 1
            i32.sub
            local.get 2
            i32.ge_u
            br_if 0 (;@4;)
            local.get 1
            local.get 2
            i32.add
            local.set 3
            local.get 0
            local.get 2
            i32.add
            local.set 4
            block ;; label = @5
              local.get 2
              i32.const 16
              i32.ge_u
              br_if 0 (;@5;)
              local.get 0
              local.set 5
              br 3 (;@2;)
            end
            local.get 4
            i32.const -4
            i32.and
            local.set 5
            i32.const 0
            local.get 4
            i32.const 3
            i32.and
            local.tee 6
            i32.sub
            local.set 7
            block ;; label = @5
              local.get 6
              i32.eqz
              br_if 0 (;@5;)
              local.get 1
              local.get 2
              i32.add
              i32.const -1
              i32.add
              local.set 8
              loop ;; label = @6
                local.get 4
                i32.const -1
                i32.add
                local.tee 4
                local.get 8
                i32.load8_u
                i32.store8
                local.get 8
                i32.const -1
                i32.add
                local.set 8
                local.get 5
                local.get 4
                i32.lt_u
                br_if 0 (;@6;)
              end
            end
            local.get 5
            local.get 2
            local.get 6
            i32.sub
            local.tee 9
            i32.const -4
            i32.and
            local.tee 6
            i32.sub
            local.set 4
            block ;; label = @5
              local.get 3
              local.get 7
              i32.add
              local.tee 7
              i32.const 3
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 6
              i32.const 1
              i32.lt_s
              br_if 2 (;@3;)
              local.get 7
              i32.const 3
              i32.shl
              local.tee 8
              i32.const 24
              i32.and
              local.set 2
              local.get 7
              i32.const -4
              i32.and
              local.tee 10
              i32.const -4
              i32.add
              local.set 1
              i32.const 0
              local.get 8
              i32.sub
              i32.const 24
              i32.and
              local.set 3
              local.get 10
              i32.load
              local.set 8
              loop ;; label = @6
                local.get 5
                i32.const -4
                i32.add
                local.tee 5
                local.get 8
                local.get 3
                i32.shl
                local.get 1
                i32.load
                local.tee 8
                local.get 2
                i32.shr_u
                i32.or
                i32.store
                local.get 1
                i32.const -4
                i32.add
                local.set 1
                local.get 4
                local.get 5
                i32.lt_u
                br_if 0 (;@6;)
                br 3 (;@3;)
              end
            end
            local.get 6
            i32.const 1
            i32.lt_s
            br_if 1 (;@3;)
            local.get 9
            local.get 1
            i32.add
            i32.const -4
            i32.add
            local.set 1
            loop ;; label = @5
              local.get 5
              i32.const -4
              i32.add
              local.tee 5
              local.get 1
              i32.load
              i32.store
              local.get 1
              i32.const -4
              i32.add
              local.set 1
              local.get 4
              local.get 5
              i32.lt_u
              br_if 0 (;@5;)
              br 2 (;@3;)
            end
          end
          block ;; label = @4
            block ;; label = @5
              local.get 2
              i32.const 16
              i32.ge_u
              br_if 0 (;@5;)
              local.get 0
              local.set 4
              br 1 (;@4;)
            end
            local.get 0
            i32.const 0
            local.get 0
            i32.sub
            i32.const 3
            i32.and
            local.tee 3
            i32.add
            local.set 5
            block ;; label = @5
              local.get 3
              i32.eqz
              br_if 0 (;@5;)
              local.get 0
              local.set 4
              local.get 1
              local.set 8
              loop ;; label = @6
                local.get 4
                local.get 8
                i32.load8_u
                i32.store8
                local.get 8
                i32.const 1
                i32.add
                local.set 8
                local.get 4
                i32.const 1
                i32.add
                local.tee 4
                local.get 5
                i32.lt_u
                br_if 0 (;@6;)
              end
            end
            local.get 5
            local.get 2
            local.get 3
            i32.sub
            local.tee 9
            i32.const -4
            i32.and
            local.tee 7
            i32.add
            local.set 4
            block ;; label = @5
              block ;; label = @6
                local.get 1
                local.get 3
                i32.add
                local.tee 6
                i32.const 3
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 7
                i32.const 1
                i32.lt_s
                br_if 1 (;@5;)
                local.get 6
                i32.const 3
                i32.shl
                local.tee 8
                i32.const 24
                i32.and
                local.set 2
                local.get 6
                i32.const -4
                i32.and
                local.tee 10
                i32.const 4
                i32.add
                local.set 1
                i32.const 0
                local.get 8
                i32.sub
                i32.const 24
                i32.and
                local.set 3
                local.get 10
                i32.load
                local.set 8
                loop ;; label = @7
                  local.get 5
                  local.get 8
                  local.get 2
                  i32.shr_u
                  local.get 1
                  i32.load
                  local.tee 8
                  local.get 3
                  i32.shl
                  i32.or
                  i32.store
                  local.get 1
                  i32.const 4
                  i32.add
                  local.set 1
                  local.get 5
                  i32.const 4
                  i32.add
                  local.tee 5
                  local.get 4
                  i32.lt_u
                  br_if 0 (;@7;)
                  br 2 (;@5;)
                end
              end
              local.get 7
              i32.const 1
              i32.lt_s
              br_if 0 (;@5;)
              local.get 6
              local.set 1
              loop ;; label = @6
                local.get 5
                local.get 1
                i32.load
                i32.store
                local.get 1
                i32.const 4
                i32.add
                local.set 1
                local.get 5
                i32.const 4
                i32.add
                local.tee 5
                local.get 4
                i32.lt_u
                br_if 0 (;@6;)
              end
            end
            local.get 9
            i32.const 3
            i32.and
            local.set 2
            local.get 6
            local.get 7
            i32.add
            local.set 1
          end
          local.get 2
          i32.eqz
          br_if 2 (;@1;)
          local.get 4
          local.get 2
          i32.add
          local.set 5
          loop ;; label = @4
            local.get 4
            local.get 1
            i32.load8_u
            i32.store8
            local.get 1
            i32.const 1
            i32.add
            local.set 1
            local.get 4
            i32.const 1
            i32.add
            local.tee 4
            local.get 5
            i32.lt_u
            br_if 0 (;@4;)
            br 3 (;@1;)
          end
        end
        local.get 9
        i32.const 3
        i32.and
        local.tee 1
        i32.eqz
        br_if 1 (;@1;)
        local.get 7
        i32.const 0
        local.get 6
        i32.sub
        i32.add
        local.set 3
        local.get 4
        local.get 1
        i32.sub
        local.set 5
      end
      local.get 3
      i32.const -1
      i32.add
      local.set 1
      loop ;; label = @2
        local.get 4
        i32.const -1
        i32.add
        local.tee 4
        local.get 1
        i32.load8_u
        i32.store8
        local.get 1
        i32.const -1
        i32.add
        local.set 1
        local.get 5
        local.get 4
        i32.lt_u
        br_if 0 (;@2;)
      end
    end
    local.get 0
  )
  (func (;87;) (type 1) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call 86
  )
  (func (;88;) (type 1) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call 85
  )
  (data (;0;) (i32.const 1048576) "src/lib.rs\00\00\00\00\10\00\0a\00\00\00B\00\00\00\10\00\00\00\00\00\10\00\0a\00\00\00I\00\00\00\11\00\00\00\01\00\00\00\00\00\00\00\04\00\00\00\0c\00\00\00\04\00\00\00\05\00\00\00\06\00\00\00\07\00\00\00/rust/deps/dlmalloc-0.2.6/src/dlmalloc.rsassertion failed: psize >= size + min_overhead\00L\00\10\00)\00\00\00\a8\04\00\00\09\00\00\00assertion failed: psize <= size + max_overhead\00\00L\00\10\00)\00\00\00\ae\04\00\00\0d\00\00\00memory allocation of  bytes failed\00\00\f4\00\10\00\15\00\00\00\09\01\10\00\0d\00\00\00library/std/src/alloc.rs(\01\10\00\18\00\00\00d\01\00\00\09\00\00\00\04\00\00\00\0c\00\00\00\04\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\04\00\00\00\09\00\00\00\00\00\00\00\08\00\00\00\04\00\00\00\0a\00\00\00\0b\00\00\00\0c\00\00\00\0d\00\00\00\0e\00\00\00\10\00\00\00\04\00\00\00\0f\00\00\00\10\00\00\00\11\00\00\00\12\00\00\00Error\00\00\00\13\00\00\00\0c\00\00\00\04\00\00\00\14\00\00\00\15\00\00\00\16\00\00\00capacity overflow\00\00\00\c8\01\10\00\11\00\00\00library/alloc/src/raw_vec.rs\e4\01\10\00\1c\00\00\00\19\00\00\00\05\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\17\00\00\00a formatting trait implementation returned an error when the underlying stream did notlibrary/alloc/src/fmt.rs\00\00v\02\10\00\18\00\00\00\7f\02\00\00\0e\00\00\00index out of bounds: the len is  but the index is \00\00\a0\02\10\00 \00\00\00\c0\02\10\00\12\00\00\00: \00\00\01\00\00\00\00\00\00\00\e4\02\10\00\02\00\00\0000010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899")
)
