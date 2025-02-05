(module
  (func (export "f32.no_fma") (param $x f32) (param $y f32) (param $z f32) (result f32)
    (f32.add (f32.mul (local.get $x) (local.get $y)) (local.get $z)))
  (func (export "f64.no_fma") (param $x f64) (param $y f64) (param $z f64) (result f64)
    (f64.add (f64.mul (local.get $x) (local.get $y)) (local.get $z)))

  (func (export "f32.no_fold_add_zero") (param $x f32) (result f32)
    (f32.add (local.get $x) (f32.const 0.0)))
  (func (export "f64.no_fold_add_zero") (param $x f64) (result f64)
    (f64.add (local.get $x) (f64.const 0.0)))

  (func (export "f32.no_fold_zero_sub") (param $x f32) (result f32)
    (f32.sub (f32.const 0.0) (local.get $x)))
  (func (export "f64.no_fold_zero_sub") (param $x f64) (result f64)
    (f64.sub (f64.const 0.0) (local.get $x)))

  (func (export "f32.no_fold_sub_zero") (param $x f32) (result f32)
    (f32.sub (local.get $x) (f32.const 0.0)))
  (func (export "f64.no_fold_sub_zero") (param $x f64) (result f64)
    (f64.sub (local.get $x) (f64.const 0.0)))

  (func (export "f32.no_fold_mul_zero") (param $x f32) (result f32)
    (f32.mul (local.get $x) (f32.const 0.0)))
  (func (export "f64.no_fold_mul_zero") (param $x f64) (result f64)
    (f64.mul (local.get $x) (f64.const 0.0)))

  (func (export "f32.no_fold_mul_one") (param $x f32) (result f32)
    (f32.mul (local.get $x) (f32.const 1.0)))
  (func (export "f64.no_fold_mul_one") (param $x f64) (result f64)
    (f64.mul (local.get $x) (f64.const 1.0)))

  (func (export "f32.no_fold_zero_div") (param $x f32) (result f32)
    (f32.div (f32.const 0.0) (local.get $x)))
  (func (export "f64.no_fold_zero_div") (param $x f64) (result f64)
    (f64.div (f64.const 0.0) (local.get $x)))

  (func (export "f32.no_fold_div_one") (param $x f32) (result f32)
    (f32.div (local.get $x) (f32.const 1.0)))
  (func (export "f64.no_fold_div_one") (param $x f64) (result f64)
    (f64.div (local.get $x) (f64.const 1.0)))

  (func (export "f32.no_fold_div_neg1") (param $x f32) (result f32)
    (f32.div (local.get $x) (f32.const -1.0)))
  (func (export "f64.no_fold_div_neg1") (param $x f64) (result f64)
    (f64.div (local.get $x) (f64.const -1.0)))

  (func (export "f32.no_fold_neg0_sub") (param $x f32) (result f32)
    (f32.sub (f32.const -0.0) (local.get $x)))
  (func (export "f64.no_fold_neg0_sub") (param $x f64) (result f64)
    (f64.sub (f64.const -0.0) (local.get $x)))

  (func (export "f32.no_fold_neg1_mul") (param $x f32) (result f32)
    (f32.mul (f32.const -1.0) (local.get $x)))
  (func (export "f64.no_fold_neg1_mul") (param $x f64) (result f64)
    (f64.mul (f64.const -1.0) (local.get $x)))

  (func (export "f32.no_fold_eq_self") (param $x f32) (result i32)
    (f32.eq (local.get $x) (local.get $x)))
  (func (export "f64.no_fold_eq_self") (param $x f64) (result i32)
    (f64.eq (local.get $x) (local.get $x)))

  (func (export "f32.no_fold_ne_self") (param $x f32) (result i32)
    (f32.ne (local.get $x) (local.get $x)))
  (func (export "f64.no_fold_ne_self") (param $x f64) (result i32)
    (f64.ne (local.get $x) (local.get $x)))

  (func (export "f32.no_fold_sub_self") (param $x f32) (result f32)
    (f32.sub (local.get $x) (local.get $x)))
  (func (export "f64.no_fold_sub_self") (param $x f64) (result f64)
    (f64.sub (local.get $x) (local.get $x)))

  (func (export "f32.no_fold_div_self") (param $x f32) (result f32)
    (f32.div (local.get $x) (local.get $x)))
  (func (export "f64.no_fold_div_self") (param $x f64) (result f64)
    (f64.div (local.get $x) (local.get $x)))

  (func (export "f32.no_fold_div_3") (param $x f32) (result f32)
    (f32.div (local.get $x) (f32.const 3.0)))
  (func (export "f64.no_fold_div_3") (param $x f64) (result f64)
    (f64.div (local.get $x) (f64.const 3.0)))

  (func (export "f32.no_factor") (param $x f32) (param $y f32) (param $z f32) (result f32)
    (f32.add (f32.mul (local.get $x) (local.get $z)) (f32.mul (local.get $y) (local.get $z))))
  (func (export "f64.no_factor") (param $x f64) (param $y f64) (param $z f64) (result f64)
    (f64.add (f64.mul (local.get $x) (local.get $z)) (f64.mul (local.get $y) (local.get $z))))

  (func (export "f32.no_distribute") (param $x f32) (param $y f32) (param $z f32) (result f32)
    (f32.mul (f32.add (local.get $x) (local.get $y)) (local.get $z)))
  (func (export "f64.no_distribute") (param $x f64) (param $y f64) (param $z f64) (result f64)
    (f64.mul (f64.add (local.get $x) (local.get $y)) (local.get $z)))

  (func (export "f32.no_regroup_div_mul") (param $x f32) (param $y f32) (param $z f32) (result f32)
    (f32.mul (local.get $x) (f32.div (local.get $y) (local.get $z))))
  (func (export "f64.no_regroup_div_mul") (param $x f64) (param $y f64) (param $z f64) (result f64)
    (f64.mul (local.get $x) (f64.div (local.get $y) (local.get $z))))

  (func (export "f32.no_regroup_mul_div") (param $x f32) (param $y f32) (param $z f32) (result f32)
    (f32.div (f32.mul (local.get $x) (local.get $y)) (local.get $z)))
  (func (export "f64.no_regroup_mul_div") (param $x f64) (param $y f64) (param $z f64) (result f64)
    (f64.div (f64.mul (local.get $x) (local.get $y)) (local.get $z)))

  (func (export "f32.no_reassociate_add") (param $x f32) (param $y f32) (param $z f32) (param $w f32) (result f32)
    (f32.add (f32.add (f32.add (local.get $x) (local.get $y)) (local.get $z)) (local.get $w)))
  (func (export "f64.no_reassociate_add") (param $x f64) (param $y f64) (param $z f64) (param $w f64) (result f64)
    (f64.add (f64.add (f64.add (local.get $x) (local.get $y)) (local.get $z)) (local.get $w)))

  (func (export "f32.no_reassociate_mul") (param $x f32) (param $y f32) (param $z f32) (param $w f32) (result f32)
    (f32.mul (f32.mul (f32.mul (local.get $x) (local.get $y)) (local.get $z)) (local.get $w)))
  (func (export "f64.no_reassociate_mul") (param $x f64) (param $y f64) (param $z f64) (param $w f64) (result f64)
    (f64.mul (f64.mul (f64.mul (local.get $x) (local.get $y)) (local.get $z)) (local.get $w)))

  (func (export "f32.no_fold_div_0") (param $x f32) (result f32)
    (f32.div (local.get $x) (f32.const 0.0)))
  (func (export "f64.no_fold_div_0") (param $x f64) (result f64)
    (f64.div (local.get $x) (f64.const 0.0)))

  (func (export "f32.no_fold_div_neg0") (param $x f32) (result f32)
    (f32.div (local.get $x) (f32.const -0.0)))
  (func (export "f64.no_fold_div_neg0") (param $x f64) (result f64)
    (f64.div (local.get $x) (f64.const -0.0)))

  (func (export "f32.no_fold_to_hypot") (param $x f32) (param $y f32) (result f32)
    (f32.sqrt (f32.add (f32.mul (local.get $x) (local.get $x))
                       (f32.mul (local.get $y) (local.get $y)))))
  (func (export "f64.no_fold_to_hypot") (param $x f64) (param $y f64) (result f64)
    (f64.sqrt (f64.add (f64.mul (local.get $x) (local.get $x))
                       (f64.mul (local.get $y) (local.get $y)))))

  (func (export "f32.no_approximate_reciprocal") (param $x f32) (result f32)
    (f32.div (f32.const 1.0) (local.get $x)))

  (func (export "f32.no_approximate_reciprocal_sqrt") (param $x f32) (result f32)
    (f32.div (f32.const 1.0) (f32.sqrt (local.get $x))))
  (func (export "f64.no_fuse_reciprocal_sqrt") (param $x f64) (result f64)
    (f64.div (f64.const 1.0) (f64.sqrt (local.get $x))))

  (func (export "f32.no_approximate_sqrt_reciprocal") (param $x f32) (result f32)
    (f32.sqrt (f32.div (f32.const 1.0) (local.get $x))))

  (func (export "i32.no_fold_f32_s") (param i32) (result i32)
    (i32.trunc_f32_s (f32.convert_i32_s (local.get 0))))
  (func (export "i32.no_fold_f32_u") (param i32) (result i32)
    (i32.trunc_f32_u (f32.convert_i32_u (local.get 0))))
  (func (export "i64.no_fold_f64_s") (param i64) (result i64)
    (i64.trunc_f64_s (f64.convert_i64_s (local.get 0))))
  (func (export "i64.no_fold_f64_u") (param i64) (result i64)
    (i64.trunc_f64_u (f64.convert_i64_u (local.get 0))))

  (func (export "f32.no_fold_add_sub") (param $x f32) (param $y f32) (result f32)
    (f32.sub (f32.add (local.get $x) (local.get $y)) (local.get $y)))
  (func (export "f64.no_fold_add_sub") (param $x f64) (param $y f64) (result f64)
    (f64.sub (f64.add (local.get $x) (local.get $y)) (local.get $y)))

  (func (export "f32.no_fold_sub_add") (param $x f32) (param $y f32) (result f32)
    (f32.add (f32.sub (local.get $x) (local.get $y)) (local.get $y)))
  (func (export "f64.no_fold_sub_add") (param $x f64) (param $y f64) (result f64)
    (f64.add (f64.sub (local.get $x) (local.get $y)) (local.get $y)))

  (func (export "f32.no_fold_mul_div") (param $x f32) (param $y f32) (result f32)
    (f32.div (f32.mul (local.get $x) (local.get $y)) (local.get $y)))
  (func (export "f64.no_fold_mul_div") (param $x f64) (param $y f64) (result f64)
    (f64.div (f64.mul (local.get $x) (local.get $y)) (local.get $y)))

  (func (export "f32.no_fold_div_mul") (param $x f32) (param $y f32) (result f32)
    (f32.mul (f32.div (local.get $x) (local.get $y)) (local.get $y)))
  (func (export "f64.no_fold_div_mul") (param $x f64) (param $y f64) (result f64)
    (f64.mul (f64.div (local.get $x) (local.get $y)) (local.get $y)))

  (func (export "f32.no_fold_div2_mul2") (param $x f32) (result f32)
    (f32.mul (f32.div (local.get $x) (f32.const 2.0)) (f32.const 2.0)))
  (func (export "f64.no_fold_div2_mul2") (param $x f64) (result f64)
    (f64.mul (f64.div (local.get $x) (f64.const 2.0)) (f64.const 2.0)))

  (func (export "no_fold_demote_promote") (param $x f64) (result f64)
    (f64.promote_f32 (f32.demote_f64 (local.get $x))))

  (func (export "no_fold_promote_demote") (param $x f32) (result f32)
    (f32.demote_f64 (f64.promote_f32 (local.get $x))))

  (func (export "no_demote_mixed_add") (param $x f64) (param $y f32) (result f32)
    (f32.demote_f64 (f64.add (local.get $x) (f64.promote_f32 (local.get $y)))))
  (func (export "no_demote_mixed_add_commuted") (param $y f32) (param $x f64) (result f32)
    (f32.demote_f64 (f64.add (f64.promote_f32 (local.get $y)) (local.get $x))))

  (func (export "no_demote_mixed_sub") (param $x f64) (param $y f32) (result f32)
    (f32.demote_f64 (f64.sub (local.get $x) (f64.promote_f32 (local.get $y)))))
  (func (export "no_demote_mixed_sub_commuted") (param $y f32) (param $x f64) (result f32)
    (f32.demote_f64 (f64.sub (f64.promote_f32 (local.get $y)) (local.get $x))))

  (func (export "no_demote_mixed_mul") (param $x f64) (param $y f32) (result f32)
    (f32.demote_f64 (f64.mul (local.get $x) (f64.promote_f32 (local.get $y)))))
  (func (export "no_demote_mixed_mul_commuted") (param $y f32) (param $x f64) (result f32)
    (f32.demote_f64 (f64.mul (f64.promote_f32 (local.get $y)) (local.get $x))))

  (func (export "no_demote_mixed_div") (param $x f64) (param $y f32) (result f32)
    (f32.demote_f64 (f64.div (local.get $x) (f64.promote_f32 (local.get $y)))))
  (func (export "no_demote_mixed_div_commuted") (param $y f32) (param $x f64) (result f32)
    (f32.demote_f64 (f64.div (f64.promote_f32 (local.get $y)) (local.get $x))))

  (func (export "f32.i32.no_fold_trunc_s_convert_s") (param $x f32) (result f32)
    (f32.convert_i32_s (i32.trunc_f32_s (local.get $x))))
  (func (export "f32.i32.no_fold_trunc_u_convert_s") (param $x f32) (result f32)
    (f32.convert_i32_s (i32.trunc_f32_u (local.get $x))))
  (func (export "f32.i32.no_fold_trunc_s_convert_u") (param $x f32) (result f32)
    (f32.convert_i32_u (i32.trunc_f32_s (local.get $x))))
  (func (export "f32.i32.no_fold_trunc_u_convert_u") (param $x f32) (result f32)
    (f32.convert_i32_u (i32.trunc_f32_u (local.get $x))))
  (func (export "f64.i32.no_fold_trunc_s_convert_s") (param $x f64) (result f64)
    (f64.convert_i32_s (i32.trunc_f64_s (local.get $x))))
  (func (export "f64.i32.no_fold_trunc_u_convert_s") (param $x f64) (result f64)
    (f64.convert_i32_s (i32.trunc_f64_u (local.get $x))))
  (func (export "f64.i32.no_fold_trunc_s_convert_u") (param $x f64) (result f64)
    (f64.convert_i32_u (i32.trunc_f64_s (local.get $x))))
  (func (export "f64.i32.no_fold_trunc_u_convert_u") (param $x f64) (result f64)
    (f64.convert_i32_u (i32.trunc_f64_u (local.get $x))))
  (func (export "f32.i64.no_fold_trunc_s_convert_s") (param $x f32) (result f32)
    (f32.convert_i64_s (i64.trunc_f32_s (local.get $x))))
  (func (export "f32.i64.no_fold_trunc_u_convert_s") (param $x f32) (result f32)
    (f32.convert_i64_s (i64.trunc_f32_u (local.get $x))))
  (func (export "f32.i64.no_fold_trunc_s_convert_u") (param $x f32) (result f32)
    (f32.convert_i64_u (i64.trunc_f32_s (local.get $x))))
  (func (export "f32.i64.no_fold_trunc_u_convert_u") (param $x f32) (result f32)
    (f32.convert_i64_u (i64.trunc_f32_u (local.get $x))))
  (func (export "f64.i64.no_fold_trunc_s_convert_s") (param $x f64) (result f64)
    (f64.convert_i64_s (i64.trunc_f64_s (local.get $x))))
  (func (export "f64.i64.no_fold_trunc_u_convert_s") (param $x f64) (result f64)
    (f64.convert_i64_s (i64.trunc_f64_u (local.get $x))))
  (func (export "f64.i64.no_fold_trunc_s_convert_u") (param $x f64) (result f64)
    (f64.convert_i64_u (i64.trunc_f64_s (local.get $x))))
  (func (export "f64.i64.no_fold_trunc_u_convert_u") (param $x f64) (result f64)
    (f64.convert_i64_u (i64.trunc_f64_u (local.get $x))))

  (func (export "f32.ult") (param $x f32) (param $y f32) (result i32) (i32.eqz (f32.ge (local.get $x) (local.get $y))))
  (func (export "f32.ule") (param $x f32) (param $y f32) (result i32) (i32.eqz (f32.gt (local.get $x) (local.get $y))))
  (func (export "f32.ugt") (param $x f32) (param $y f32) (result i32) (i32.eqz (f32.le (local.get $x) (local.get $y))))
  (func (export "f32.uge") (param $x f32) (param $y f32) (result i32) (i32.eqz (f32.lt (local.get $x) (local.get $y))))

  (func (export "f64.ult") (param $x f64) (param $y f64) (result i32) (i32.eqz (f64.ge (local.get $x) (local.get $y))))
  (func (export "f64.ule") (param $x f64) (param $y f64) (result i32) (i32.eqz (f64.gt (local.get $x) (local.get $y))))
  (func (export "f64.ugt") (param $x f64) (param $y f64) (result i32) (i32.eqz (f64.le (local.get $x) (local.get $y))))
  (func (export "f64.uge") (param $x f64) (param $y f64) (result i32) (i32.eqz (f64.lt (local.get $x) (local.get $y))))

  (func (export "f32.no_fold_lt_select") (param $x f32) (param $y f32) (result f32) (select (local.get $x) (local.get $y) (f32.lt (local.get $x) (local.get $y))))
  (func (export "f32.no_fold_le_select") (param $x f32) (param $y f32) (result f32) (select (local.get $x) (local.get $y) (f32.le (local.get $x) (local.get $y))))
  (func (export "f32.no_fold_gt_select") (param $x f32) (param $y f32) (result f32) (select (local.get $x) (local.get $y) (f32.gt (local.get $x) (local.get $y))))
  (func (export "f32.no_fold_ge_select") (param $x f32) (param $y f32) (result f32) (select (local.get $x) (local.get $y) (f32.ge (local.get $x) (local.get $y))))

  (func (export "f64.no_fold_lt_select") (param $x f64) (param $y f64) (result f64) (select (local.get $x) (local.get $y) (f64.lt (local.get $x) (local.get $y))))
  (func (export "f64.no_fold_le_select") (param $x f64) (param $y f64) (result f64) (select (local.get $x) (local.get $y) (f64.le (local.get $x) (local.get $y))))
  (func (export "f64.no_fold_gt_select") (param $x f64) (param $y f64) (result f64) (select (local.get $x) (local.get $y) (f64.gt (local.get $x) (local.get $y))))
  (func (export "f64.no_fold_ge_select") (param $x f64) (param $y f64) (result f64) (select (local.get $x) (local.get $y) (f64.ge (local.get $x) (local.get $y))))

  (func (export "f32.no_fold_lt_if") (param $x f32) (param $y f32) (result f32)
    (if (result f32) (f32.lt (local.get $x) (local.get $y))
      (then (local.get $x)) (else (local.get $y))
    )
  )
  (func (export "f32.no_fold_le_if") (param $x f32) (param $y f32) (result f32)
    (if (result f32) (f32.le (local.get $x) (local.get $y))
      (then (local.get $x)) (else (local.get $y))
    )
  )
  (func (export "f32.no_fold_gt_if") (param $x f32) (param $y f32) (result f32)
    (if (result f32) (f32.gt (local.get $x) (local.get $y))
      (then (local.get $x)) (else (local.get $y))
    )
  )
  (func (export "f32.no_fold_ge_if") (param $x f32) (param $y f32) (result f32)
    (if (result f32) (f32.ge (local.get $x) (local.get $y))
      (then (local.get $x)) (else (local.get $y))
    )
  )

  (func (export "f64.no_fold_lt_if") (param $x f64) (param $y f64) (result f64)
    (if (result f64) (f64.lt (local.get $x) (local.get $y))
      (then (local.get $x)) (else (local.get $y))
    )
  )
  (func (export "f64.no_fold_le_if") (param $x f64) (param $y f64) (result f64)
    (if (result f64) (f64.le (local.get $x) (local.get $y))
      (then (local.get $x)) (else (local.get $y))
    )
  )
  (func (export "f64.no_fold_gt_if") (param $x f64) (param $y f64) (result f64)
    (if (result f64) (f64.gt (local.get $x) (local.get $y))
      (then (local.get $x)) (else (local.get $y))
    )
  )
  (func (export "f64.no_fold_ge_if") (param $x f64) (param $y f64) (result f64)
    (if (result f64) (f64.ge (local.get $x) (local.get $y))
      (then (local.get $x)) (else (local.get $y))
    )
  )

  (func (export "f32.no_fold_lt_select_to_abs") (param $x f32) (result f32) (select (f32.neg (local.get $x)) (local.get $x) (f32.lt (local.get $x) (f32.const 0.0))))
  (func (export "f32.no_fold_le_select_to_abs") (param $x f32) (result f32) (select (f32.neg (local.get $x)) (local.get $x) (f32.le (local.get $x) (f32.const -0.0))))
  (func (export "f32.no_fold_gt_select_to_abs") (param $x f32) (result f32) (select (local.get $x) (f32.neg (local.get $x)) (f32.gt (local.get $x) (f32.const -0.0))))
  (func (export "f32.no_fold_ge_select_to_abs") (param $x f32) (result f32) (select (local.get $x) (f32.neg (local.get $x)) (f32.ge (local.get $x) (f32.const 0.0))))

  (func (export "f64.no_fold_lt_select_to_abs") (param $x f64) (result f64) (select (f64.neg (local.get $x)) (local.get $x) (f64.lt (local.get $x) (f64.const 0.0))))
  (func (export "f64.no_fold_le_select_to_abs") (param $x f64) (result f64) (select (f64.neg (local.get $x)) (local.get $x) (f64.le (local.get $x) (f64.const -0.0))))
  (func (export "f64.no_fold_gt_select_to_abs") (param $x f64) (result f64) (select (local.get $x) (f64.neg (local.get $x)) (f64.gt (local.get $x) (f64.const -0.0))))
  (func (export "f64.no_fold_ge_select_to_abs") (param $x f64) (result f64) (select (local.get $x) (f64.neg (local.get $x)) (f64.ge (local.get $x) (f64.const 0.0))))

  (func (export "f32.no_fold_lt_if_to_abs") (param $x f32) (result f32)
    (if (result f32) (f32.lt (local.get $x) (f32.const 0.0))
      (then (f32.neg (local.get $x))) (else (local.get $x))
    )
  )
  (func (export "f32.no_fold_le_if_to_abs") (param $x f32) (result f32)
    (if (result f32) (f32.le (local.get $x) (f32.const -0.0))
      (then (f32.neg (local.get $x))) (else (local.get $x))
    )
  )
  (func (export "f32.no_fold_gt_if_to_abs") (param $x f32) (result f32)
    (if (result f32) (f32.gt (local.get $x) (f32.const -0.0))
      (then (local.get $x)) (else (f32.neg (local.get $x)))
    )
  )
  (func (export "f32.no_fold_ge_if_to_abs") (param $x f32) (result f32)
    (if (result f32) (f32.ge (local.get $x) (f32.const 0.0))
      (then (local.get $x)) (else (f32.neg (local.get $x)))
    )
  )

  (func (export "f64.no_fold_lt_if_to_abs") (param $x f64) (result f64)
    (if (result f64) (f64.lt (local.get $x) (f64.const 0.0))
      (then (f64.neg (local.get $x))) (else (local.get $x))
    )
  )
  (func (export "f64.no_fold_le_if_to_abs") (param $x f64) (result f64)
    (if (result f64) (f64.le (local.get $x) (f64.const -0.0))
      (then (f64.neg (local.get $x))) (else (local.get $x))
    )
  )
  (func (export "f64.no_fold_gt_if_to_abs") (param $x f64) (result f64)
    (if (result f64) (f64.gt (local.get $x) (f64.const -0.0))
      (then (local.get $x)) (else (f64.neg (local.get $x)))
    )
  )
  (func (export "f64.no_fold_ge_if_to_abs") (param $x f64) (result f64)
    (if (result f64) (f64.ge (local.get $x) (f64.const 0.0))
      (then (local.get $x)) (else (f64.neg (local.get $x)))
    )
  )

  (func (export "f32.incorrect_correction") (result f32)
    (f32.sub (f32.sub (f32.add (f32.const 1.333) (f32.const 1.225)) (f32.const 1.333)) (f32.const 1.225))
  )
  (func (export "f64.incorrect_correction") (result f64)
    (f64.sub (f64.sub (f64.add (f64.const 1.333) (f64.const 1.225)) (f64.const 1.333)) (f64.const 1.225))
  )

  (memory 1 1)
  (func (export "init") (param $i i32) (param $x f32) (f32.store (local.get $i) (local.get $x)))

  (func (export "run") (param $n i32) (param $z f32)
    (local $i i32)
    (block $exit
      (loop $cont
        (f32.store
          (local.get $i)
          (f32.div (f32.load (local.get $i)) (local.get $z))
        )
        (local.set $i (i32.add (local.get $i) (i32.const 4)))
        (br_if $cont (i32.lt_u (local.get $i) (local.get $n)))
      )
    )
  )

  (func (export "check") (param $i i32) (result f32) (f32.load (local.get $i)))

  (func (export "calculate") (result f32)
    (local $x f32)
    (local $r f32)
    (local $q f32)
    (local $z0 f32)
    (local $z1 f32)
    (local.set $x (f32.const 156.25))
    (local.set $r (f32.const 208.333333334))
    (local.set $q (f32.const 1.77951304201))
    (local.set $z0 (f32.div (f32.mul (f32.neg (local.get $r)) (local.get $x)) (f32.sub (f32.mul (local.get $x) (local.get $q)) (local.get $r))))
    (local.set $z1 (f32.div (f32.mul (f32.neg (local.get $r)) (local.get $x)) (f32.sub (f32.mul (local.get $x) (local.get $q)) (local.get $r))))
    (block (br_if 0 (f32.eq (local.get $z0) (local.get $z1))) (unreachable))
    (local.get $z1)
  )

  (func (export "thepast0") (param $a f64) (param $b f64) (param $c f64) (param $d f64) (result f64)
    (f64.div (f64.mul (local.get $a) (local.get $b)) (f64.mul (local.get $c) (local.get $d)))
  )

  (func (export "thepast1") (param $a f64) (param $b f64) (param $c f64) (result f64)
    (f64.sub (f64.mul (local.get $a) (local.get $b)) (local.get $c))
  )

  (func (export "thepast2") (param $a f32) (param $b f32) (param $c f32) (result f32)
    (f32.mul (f32.mul (local.get $a) (local.get $b)) (local.get $c))
  )

  (func (export "inverse") (param $x f32) (result f32)
    (f32.div (f32.const 1.0) (local.get $x))
  )

  (func (export "f32_sqrt_minus_2") (param $x f32) (result f32)
    (f32.sub (f32.sqrt (local.get $x)) (f32.const 2.0))
  )

  (func (export "f64_sqrt_minus_2") (param $x f64) (result f64)
    (f64.sub (f64.sqrt (local.get $x)) (f64.const 2.0))
  )

  (func (export "f32.no_fold_recip_recip") (param $x f32) (result f32)
    (f32.div (f32.const 1.0) (f32.div (f32.const 1.0) (local.get $x))))

  (func (export "f64.no_fold_recip_recip") (param $x f64) (result f64)
    (f64.div (f64.const 1.0) (f64.div (f64.const 1.0) (local.get $x))))

  (func (export "f32.no_algebraic_factoring") (param $x f32) (param $y f32) (result f32)
    (f32.mul (f32.add (local.get $x) (local.get $y))
             (f32.sub (local.get $x) (local.get $y))))

  (func (export "f64.no_algebraic_factoring") (param $x f64) (param $y f64) (result f64)
    (f64.mul (f64.add (local.get $x) (local.get $y))
             (f64.sub (local.get $x) (local.get $y))))

  (func (export "f32.no_algebraic_factoring") (param $x f32) (param $y f32) (result f32)
    (f32.sub (f32.mul (local.get $x) (local.get $x))
             (f32.mul (local.get $y) (local.get $y))))

  (func (export "f64.no_algebraic_factoring") (param $x f64) (param $y f64) (result f64)
    (f64.sub (f64.mul (local.get $x) (local.get $x))
             (f64.mul (local.get $y) (local.get $y))))

  (func (export "f32.no_fold_neg_add") (param $x f32) (param $y f32) (result f32)
    (f32.neg (f32.add (local.get $x) (local.get $y))))

  (func (export "f64.no_fold_neg_add") (param $x f64) (param $y f64) (result f64)
    (f64.neg (f64.add (local.get $x) (local.get $y))))

  (func (export "f32.no_fold_add_neg") (param $x f32) (result f32)
    (f32.add (f32.neg (local.get $x)) (local.get $x)))

  (func (export "f64.no_fold_add_neg") (param $x f64) (result f64)
    (f64.add (f64.neg (local.get $x)) (local.get $x)))

  (func (export "f32.no_fold_6x_via_add") (param $x f32) (result f32)
    (f32.add (f32.add (f32.add (f32.add (f32.add
    (local.get $x)
    (local.get $x)) (local.get $x)) (local.get $x))
    (local.get $x)) (local.get $x)))

  (func (export "f64.no_fold_6x_via_add") (param $x f64) (result f64)
    (f64.add (f64.add (f64.add (f64.add (f64.add
    (local.get $x)
    (local.get $x)) (local.get $x)) (local.get $x))
    (local.get $x)) (local.get $x)))

  (func (export "f32.no_fold_div_div") (param $x f32) (param $y f32) (param $z f32) (result f32)
    (f32.div (f32.div (local.get $x) (local.get $y)) (local.get $z)))

  (func (export "f64.no_fold_div_div") (param $x f64) (param $y f64) (param $z f64) (result f64)
    (f64.div (f64.div (local.get $x) (local.get $y)) (local.get $z)))

  (func (export "f32.no_fold_mul_divs") (param $x f32) (param $y f32) (param $z f32) (param $w f32) (result f32)
    (f32.mul (f32.div (local.get $x) (local.get $y)) (f32.div (local.get $z) (local.get $w))))

  (func (export "f64.no_fold_mul_divs") (param $x f64) (param $y f64) (param $z f64) (param $w f64) (result f64)
    (f64.mul (f64.div (local.get $x) (local.get $y)) (f64.div (local.get $z) (local.get $w))))

  (func (export "f32.no_fold_add_divs") (param $x f32) (param $y f32) (param $z f32) (result f32)
    (f32.add (f32.div (local.get $x) (local.get $z)) (f32.div (local.get $y) (local.get $z))))

  (func (export "f64.no_fold_add_divs") (param $x f64) (param $y f64) (param $z f64) (result f64)
    (f64.add (f64.div (local.get $x) (local.get $z)) (f64.div (local.get $y) (local.get $z))))

  (func (export "f32.no_fold_sqrt_square") (param $x f32) (result f32)
    (f32.sqrt (f32.mul (local.get $x) (local.get $x))))

  (func (export "f64.no_fold_sqrt_square") (param $x f64) (result f64)
    (f64.sqrt (f64.mul (local.get $x) (local.get $x))))

  (func (export "f32.no_fold_mul_sqrts") (param $x f32) (param $y f32) (result f32)
    (f32.mul (f32.sqrt (local.get $x)) (f32.sqrt (local.get $y))))

  (func (export "f64.no_fold_mul_sqrts") (param $x f64) (param $y f64) (result f64)
    (f64.mul (f64.sqrt (local.get $x)) (f64.sqrt (local.get $y))))

  (func (export "f32.no_fold_div_sqrts") (param $x f32) (param $y f32) (result f32)
    (f32.div (f32.sqrt (local.get $x)) (f32.sqrt (local.get $y))))

  (func (export "f64.no_fold_div_sqrts") (param $x f64) (param $y f64) (result f64)
    (f64.div (f64.sqrt (local.get $x)) (f64.sqrt (local.get $y))))

  (func (export "f32.no_fold_mul_sqrt_div") (param $x f32) (param $y f32) (result f32)
    (f32.div (f32.mul (local.get $x) (f32.sqrt (local.get $y))) (local.get $y)))

  (func (export "f64.no_fold_mul_sqrt_div") (param $x f64) (param $y f64) (result f64)
    (f64.div (f64.mul (local.get $x) (f64.sqrt (local.get $y))) (local.get $y)))

  (func (export "f32.no_flush_intermediate_subnormal") (param $x f32) (param $y f32) (param $z f32) (result f32)
    (f32.mul (f32.mul (local.get $x) (local.get $y)) (local.get $z)))

  (func (export "f64.no_flush_intermediate_subnormal") (param $x f64) (param $y f64) (param $z f64) (result f64)
    (f64.mul (f64.mul (local.get $x) (local.get $y)) (local.get $z)))

  (func (export "f32.recoding_eq") (param $x f32) (param $y f32) (result i32)
    (f32.eq (f32.mul (local.get $x) (local.get $y)) (local.get $x)))

  (func (export "f32.recoding_le") (param $x f32) (param $y f32) (result i32)
    (f32.le (f32.mul (local.get $x) (local.get $y)) (local.get $x)))

  (func (export "f32.recoding_lt") (param $x f32) (param $y f32) (result i32)
    (f32.lt (f32.mul (local.get $x) (local.get $y)) (local.get $x)))

  (func (export "f64.recoding_eq") (param $x f64) (param $y f64) (result i32)
    (f64.eq (f64.mul (local.get $x) (local.get $y)) (local.get $x)))

  (func (export "f64.recoding_le") (param $x f64) (param $y f64) (result i32)
    (f64.le (f64.mul (local.get $x) (local.get $y)) (local.get $x)))

  (func (export "f64.recoding_lt") (param $x f64) (param $y f64) (result i32)
    (f64.lt (f64.mul (local.get $x) (local.get $y)) (local.get $x)))

  (func (export "recoding_demote") (param $x f64) (param $y f32) (result f32)
    (f32.mul (f32.demote_f64 (local.get $x)) (local.get $y)))

  (func (export "f32.no_extended_precision_div") (param $x f32) (param $y f32) (param $z f32) (result i32)
    (f32.eq (f32.div (local.get $x) (local.get $y)) (local.get $z)))

  (func (export "f64.no_extended_precision_div") (param $x f64) (param $y f64) (param $z f64) (result i32)
    (f64.eq (f64.div (local.get $x) (local.get $y)) (local.get $z)))

  (func (export "f32.no_distribute_exact") (param $x f32) (result f32)
    (f32.add (f32.mul (f32.const -8.0) (local.get $x)) (f32.mul (f32.const 8.0) (local.get $x))))

  (func (export "f64.no_distribute_exact") (param $x f64) (result f64)
    (f64.add (f64.mul (f64.const -8.0) (local.get $x)) (f64.mul (f64.const 8.0) (local.get $x))))

  (func (export "f32.sqrt") (param f32) (result f32)
    (f32.sqrt (local.get 0)))

  (func (export "f32.xkcd_sqrt_2") (param f32) (param f32) (param f32) (param f32) (result f32)
    (f32.add (f32.div (local.get 0) (local.get 1)) (f32.div (local.get 2) (f32.sub (local.get 3) (local.get 2)))))

  (func (export "f32.xkcd_sqrt_3") (param f32) (param f32) (param f32) (result f32)
    (f32.div (f32.mul (local.get 0) (local.get 1)) (local.get 2)))

  (func (export "f32.xkcd_sqrt_5") (param f32) (param f32) (param f32) (result f32)
    (f32.add (f32.div (local.get 0) (local.get 1)) (f32.div (local.get 2) (local.get 0))))

  (func (export "f32.xkcd_better_sqrt_5") (param f32) (param f32) (param f32) (param f32) (result f32)
    (f32.div (f32.add (local.get 0) (f32.mul (local.get 1) (local.get 2))) (f32.sub (local.get 3) (f32.mul (local.get 1) (local.get 2)))))

  (func (export "f64.sqrt") (param f64) (result f64)
    (f64.sqrt (local.get 0)))

  (func (export "f64.xkcd_sqrt_2") (param f64) (param f64) (param f64) (param f64) (result f64)
    (f64.add (f64.div (local.get 0) (local.get 1)) (f64.div (local.get 2) (f64.sub (local.get 3) (local.get 2)))))

  (func (export "f64.xkcd_sqrt_3") (param f64) (param f64) (param f64) (result f64)
    (f64.div (f64.mul (local.get 0) (local.get 1)) (local.get 2)))

  (func (export "f64.xkcd_sqrt_5") (param f64) (param f64) (param f64) (result f64)
    (f64.add (f64.div (local.get 0) (local.get 1)) (f64.div (local.get 2) (local.get 0))))

  (func (export "f64.xkcd_better_sqrt_5") (param f64) (param f64) (param f64) (param f64) (result f64)
    (f64.div (f64.add (local.get 0) (f64.mul (local.get 1) (local.get 2))) (f64.sub (local.get 3) (f64.mul (local.get 1) (local.get 2)))))

  (func (export "f32.compute_radix") (param $0 f32) (param $1 f32) (result f32)
    (loop $label$0
      (br_if $label$0
        (f32.eq
          (f32.add
            (f32.sub
              (f32.add
                (local.tee $0 (f32.add (local.get $0) (local.get $0)))
                (f32.const 1)
              )
              (local.get $0)
            )
            (f32.const -1)
          )
          (f32.const 0)
        )
      )
    )
    (loop $label$2
      (br_if $label$2
        (f32.ne
          (f32.sub
            (f32.sub
              (f32.add
                (local.get $0)
                (local.tee $1 (f32.add (local.get $1) (f32.const 1)))
              )
              (local.get $0)
            )
            (local.get $1)
          )
          (f32.const 0)
        )
      )
    )
    (local.get $1)
  )

  (func (export "f64.compute_radix") (param $0 f64) (param $1 f64) (result f64)
    (loop $label$0
      (br_if $label$0
        (f64.eq
          (f64.add
            (f64.sub
              (f64.add
                (local.tee $0 (f64.add (local.get $0) (local.get $0)))
                (f64.const 1)
              )
              (local.get $0)
            )
            (f64.const -1)
          )
          (f64.const 0)
        )
      )
    )
    (loop $label$2
      (br_if $label$2
        (f64.ne
          (f64.sub
            (f64.sub
              (f64.add
                (local.get $0)
                (local.tee $1 (f64.add (local.get $1) (f64.const 1)))
              )
              (local.get $0)
            )
            (local.get $1)
          )
          (f64.const 0)
        )
      )
    )
    (local.get $1)
  )

  (func (export "f32.no_fold_sub1_mul_add") (param $x f32) (param $y f32) (result f32)
    (f32.add (f32.mul (f32.sub (local.get $x) (f32.const 1.0)) (local.get $y)) (local.get $y)))

  (func (export "f64.no_fold_sub1_mul_add") (param $x f64) (param $y f64) (result f64)
    (f64.add (f64.mul (f64.sub (local.get $x) (f64.const 1.0)) (local.get $y)) (local.get $y)))

  (func (export "f32.no_fold_add_le_monotonicity") (param $x f32) (param $y f32) (param $z f32) (result i32)
    (f32.le (f32.add (local.get $x) (local.get $z)) (f32.add (local.get $y) (local.get $z))))

  (func (export "f32.no_fold_add_ge_monotonicity") (param $x f32) (param $y f32) (param $z f32) (result i32)
    (f32.ge (f32.add (local.get $x) (local.get $z)) (f32.add (local.get $y) (local.get $z))))

  (func (export "f64.no_fold_add_le_monotonicity") (param $x f64) (param $y f64) (param $z f64) (result i32)
    (f64.le (f64.add (local.get $x) (local.get $z)) (f64.add (local.get $y) (local.get $z))))

  (func (export "f64.no_fold_add_ge_monotonicity") (param $x f64) (param $y f64) (param $z f64) (result i32)
    (f64.ge (f64.add (local.get $x) (local.get $z)) (f64.add (local.get $y) (local.get $z))))

  (func (export "f32.not_lt") (param $x f32) (param $y f32) (result i32)
    (i32.eqz (f32.lt (local.get $x) (local.get $y))))

  (func (export "f32.not_le") (param $x f32) (param $y f32) (result i32)
    (i32.eqz (f32.le (local.get $x) (local.get $y))))

  (func (export "f32.not_gt") (param $x f32) (param $y f32) (result i32)
    (i32.eqz (f32.gt (local.get $x) (local.get $y))))

  (func (export "f32.not_ge") (param $x f32) (param $y f32) (result i32)
    (i32.eqz (f32.ge (local.get $x) (local.get $y))))

  (func (export "f64.not_lt") (param $x f64) (param $y f64) (result i32)
    (i32.eqz (f64.lt (local.get $x) (local.get $y))))

  (func (export "f64.not_le") (param $x f64) (param $y f64) (result i32)
    (i32.eqz (f64.le (local.get $x) (local.get $y))))

  (func (export "f64.not_gt") (param $x f64) (param $y f64) (result i32)
    (i32.eqz (f64.gt (local.get $x) (local.get $y))))

  (func (export "f64.not_ge") (param $x f64) (param $y f64) (result i32)
    (i32.eqz (f64.ge (local.get $x) (local.get $y))))

  (func (export "f32.epsilon") (result f32)
    (f32.sub (f32.const 1.0) (f32.mul (f32.const 3.0) (f32.sub (f32.div (f32.const 4.0) (f32.const 3.0)) (f32.const 1.0)))))

  (func (export "f64.epsilon") (result f64)
    (f64.sub (f64.const 1.0) (f64.mul (f64.const 3.0) (f64.sub (f64.div (f64.const 4.0) (f64.const 3.0)) (f64.const 1.0)))))

  (func (export "f32.epsilon") (result f32)
    (local $x f32)
    (local $result f32)
    (local.set $x (f32.const 1))
    (loop $loop
      (br_if $loop
        (f32.gt
          (f32.add
            (local.tee $x
              (f32.mul
                (local.tee $result (local.get $x))
                (f32.const 0.5)
              )
            )
            (f32.const 1)
          )
          (f32.const 1)
        )
      )
    )
    (local.get $result)
  )

  (func (export "f64.epsilon") (result f64)
    (local $x f64)
    (local $result f64)
    (local.set $x (f64.const 1))
    (loop $loop
      (br_if $loop
        (f64.gt
          (f64.add
            (local.tee $x
              (f64.mul
                (local.tee $result (local.get $x))
                (f64.const 0.5)
              )
            )
            (f64.const 1)
          )
          (f64.const 1)
        )
      )
    )
    (local.get $result)
  )

  (func (export "f32.no_trichotomy_lt") (param $x f32) (param $y f32) (result i32)
    (i32.or (f32.lt (local.get $x) (local.get $y)) (f32.ge (local.get $x) (local.get $y))))
  (func (export "f32.no_trichotomy_le") (param $x f32) (param $y f32) (result i32)
    (i32.or (f32.le (local.get $x) (local.get $y)) (f32.gt (local.get $x) (local.get $y))))
  (func (export "f32.no_trichotomy_gt") (param $x f32) (param $y f32) (result i32)
    (i32.or (f32.gt (local.get $x) (local.get $y)) (f32.le (local.get $x) (local.get $y))))
  (func (export "f32.no_trichotomy_ge") (param $x f32) (param $y f32) (result i32)
    (i32.or (f32.ge (local.get $x) (local.get $y)) (f32.lt (local.get $x) (local.get $y))))

  (func (export "f64.no_trichotomy_lt") (param $x f64) (param $y f64) (result i32)
    (i32.or (f64.lt (local.get $x) (local.get $y)) (f64.ge (local.get $x) (local.get $y))))
  (func (export "f64.no_trichotomy_le") (param $x f64) (param $y f64) (result i32)
    (i32.or (f64.le (local.get $x) (local.get $y)) (f64.gt (local.get $x) (local.get $y))))
  (func (export "f64.no_trichotomy_gt") (param $x f64) (param $y f64) (result i32)
    (i32.or (f64.gt (local.get $x) (local.get $y)) (f64.le (local.get $x) (local.get $y))))
  (func (export "f64.no_trichotomy_ge") (param $x f64) (param $y f64) (result i32)
    (i32.or (f64.ge (local.get $x) (local.get $y)) (f64.lt (local.get $x) (local.get $y))))

(func (export "f32.arithmetic_nan_bitpattern")
        (param $x i32) (param $y i32) (result i32)
    (i32.and (i32.reinterpret_f32
               (f32.div
                 (f32.reinterpret_i32 (local.get $x))
                 (f32.reinterpret_i32 (local.get $y))))
             (i32.const 0x7fc00000)))
  (func (export "f32.canonical_nan_bitpattern")
        (param $x i32) (param $y i32) (result i32)
    (i32.and (i32.reinterpret_f32
               (f32.div
                 (f32.reinterpret_i32 (local.get $x))
                 (f32.reinterpret_i32 (local.get $y))))
             (i32.const 0x7fffffff)))
  (func (export "f32.nonarithmetic_nan_bitpattern")
        (param $x i32) (result i32)
    (i32.reinterpret_f32 (f32.neg (f32.reinterpret_i32 (local.get $x)))))

  (func (export "f64.arithmetic_nan_bitpattern")
        (param $x i64) (param $y i64) (result i64)
    (i64.and (i64.reinterpret_f64
               (f64.div
                 (f64.reinterpret_i64 (local.get $x))
                 (f64.reinterpret_i64 (local.get $y))))
             (i64.const 0x7ff8000000000000)))
  (func (export "f64.canonical_nan_bitpattern")
        (param $x i64) (param $y i64) (result i64)
    (i64.and (i64.reinterpret_f64
               (f64.div
                 (f64.reinterpret_i64 (local.get $x))
                 (f64.reinterpret_i64 (local.get $y))))
             (i64.const 0x7fffffffffffffff)))
  (func (export "f64.nonarithmetic_nan_bitpattern")
        (param $x i64) (result i64)
    (i64.reinterpret_f64 (f64.neg (f64.reinterpret_i64 (local.get $x)))))

  ;; Versions of no_fold testcases that only care about NaN bitpatterns.
  (func (export "f32.no_fold_sub_zero") (param $x i32) (result i32)
    (i32.and (i32.reinterpret_f32 (f32.sub (f32.reinterpret_i32 (local.get $x)) (f32.const 0.0)))
             (i32.const 0x7fc00000)))
  (func (export "f32.no_fold_neg0_sub") (param $x i32) (result i32)
    (i32.and (i32.reinterpret_f32 (f32.sub (f32.const -0.0) (f32.reinterpret_i32 (local.get $x))))
             (i32.const 0x7fc00000)))
  (func (export "f32.no_fold_mul_one") (param $x i32) (result i32)
    (i32.and (i32.reinterpret_f32 (f32.mul (f32.reinterpret_i32 (local.get $x)) (f32.const 1.0)))
             (i32.const 0x7fc00000)))
  (func (export "f32.no_fold_neg1_mul") (param $x i32) (result i32)
    (i32.and (i32.reinterpret_f32 (f32.mul (f32.const -1.0) (f32.reinterpret_i32 (local.get $x))))
             (i32.const 0x7fc00000)))
  (func (export "f32.no_fold_div_one") (param $x i32) (result i32)
    (i32.and (i32.reinterpret_f32 (f32.div (f32.reinterpret_i32 (local.get $x)) (f32.const 1.0)))
             (i32.const 0x7fc00000)))
  (func (export "f32.no_fold_div_neg1") (param $x i32) (result i32)
    (i32.and (i32.reinterpret_f32 (f32.div (f32.reinterpret_i32 (local.get $x)) (f32.const -1.0)))
             (i32.const 0x7fc00000)))
  (func (export "f64.no_fold_sub_zero") (param $x i64) (result i64)
    (i64.and (i64.reinterpret_f64 (f64.sub (f64.reinterpret_i64 (local.get $x)) (f64.const 0.0)))
             (i64.const 0x7ff8000000000000)))
  (func (export "f64.no_fold_neg0_sub") (param $x i64) (result i64)
    (i64.and (i64.reinterpret_f64 (f64.sub (f64.const -0.0) (f64.reinterpret_i64 (local.get $x))))
             (i64.const 0x7ff8000000000000)))
  (func (export "f64.no_fold_mul_one") (param $x i64) (result i64)
    (i64.and (i64.reinterpret_f64 (f64.mul (f64.reinterpret_i64 (local.get $x)) (f64.const 1.0)))
             (i64.const 0x7ff8000000000000)))
  (func (export "f64.no_fold_neg1_mul") (param $x i64) (result i64)
    (i64.and (i64.reinterpret_f64 (f64.mul (f64.const -1.0) (f64.reinterpret_i64 (local.get $x))))
             (i64.const 0x7ff8000000000000)))
  (func (export "f64.no_fold_div_one") (param $x i64) (result i64)
    (i64.and (i64.reinterpret_f64 (f64.div (f64.reinterpret_i64 (local.get $x)) (f64.const 1.0)))
             (i64.const 0x7ff8000000000000)))
  (func (export "f64.no_fold_div_neg1") (param $x i64) (result i64)
    (i64.and (i64.reinterpret_f64 (f64.div (f64.reinterpret_i64 (local.get $x)) (f64.const -1.0)))
             (i64.const 0x7ff8000000000000)))
  (func (export "no_fold_promote_demote") (param $x i32) (result i32)
    (i32.and (i32.reinterpret_f32 (f32.demote_f64 (f64.promote_f32 (f32.reinterpret_i32 (local.get $x)))))
             (i32.const 0x7fc00000)))

  (func (export "dot_product_example")
        (param $x0 f64) (param $x1 f64) (param $x2 f64) (param $x3 f64)
        (param $y0 f64) (param $y1 f64) (param $y2 f64) (param $y3 f64)
        (result f64)
    (f64.add (f64.add (f64.add
      (f64.mul (local.get $x0) (local.get $y0))
      (f64.mul (local.get $x1) (local.get $y1)))
      (f64.mul (local.get $x2) (local.get $y2)))
      (f64.mul (local.get $x3) (local.get $y3)))
  )

  (func (export "with_binary_sum_collapse")
        (param $x0 f64) (param $x1 f64) (param $x2 f64) (param $x3 f64)
        (param $y0 f64) (param $y1 f64) (param $y2 f64) (param $y3 f64)
        (result f64)
      (f64.add (f64.add (f64.mul (local.get $x0) (local.get $y0))
                        (f64.mul (local.get $x1) (local.get $y1)))
               (f64.add (f64.mul (local.get $x2) (local.get $y2))
                        (f64.mul (local.get $x3) (local.get $y3))))
  )

  (func (export "f32.contract2fma")
        (param $x f32) (param $y f32) (result f32)
    (f32.sqrt (f32.sub (f32.mul (local.get $x) (local.get $x))
                       (f32.mul (local.get $y) (local.get $y)))))
  (func (export "f64.contract2fma")
        (param $x f64) (param $y f64) (result f64)
    (f64.sqrt (f64.sub (f64.mul (local.get $x) (local.get $x))
                       (f64.mul (local.get $y) (local.get $y)))))

  (func (export "f32.division_by_small_number")
        (param $a f32) (param $b f32) (param $c f32) (result f32)
    (f32.sub (local.get $a) (f32.div (local.get $b) (local.get $c))))
  (func (export "f64.division_by_small_number")
        (param $a f64) (param $b f64) (param $c f64) (result f64)
    (f64.sub (local.get $a) (f64.div (local.get $b) (local.get $c))))

  (func (export "f32.golden_ratio") (param $a f32) (param $b f32) (param $c f32) (result f32)
    (f32.mul (local.get 0) (f32.add (local.get 1) (f32.sqrt (local.get 2)))))
  (func (export "f64.golden_ratio") (param $a f64) (param $b f64) (param $c f64) (result f64)
    (f64.mul (local.get 0) (f64.add (local.get 1) (f64.sqrt (local.get 2)))))

  (func (export "f32.silver_means") (param $n f32) (result f32)
    (f32.mul (f32.const 0.5)
             (f32.add (local.get $n)
                      (f32.sqrt (f32.add (f32.mul (local.get $n) (local.get $n))
                                         (f32.const 4.0))))))
  (func (export "f64.silver_means") (param $n f64) (result f64)
    (f64.mul (f64.const 0.5)
             (f64.add (local.get $n)
                      (f64.sqrt (f64.add (f64.mul (local.get $n) (local.get $n))
                                         (f64.const 4.0))))))

  (func (export "point_four") (param $four f64) (param $ten f64) (result i32)
    (f64.lt (f64.div (local.get $four) (local.get $ten)) (f64.const 0.4)))

  (func (export "tau") (param i32) (result f64)
    (local f64 f64 f64 f64)
    f64.const 0x0p+0
    local.set 1
    block
      local.get 0
      i32.const 1
      i32.lt_s
      br_if 0
      f64.const 0x1p+0
      local.set 2
      f64.const 0x0p+0
      local.set 3
      loop
        local.get 1
        local.get 2
        f64.const 0x1p+3
        local.get 3
        f64.const 0x1p+3
        f64.mul
        local.tee 4
        f64.const 0x1p+0
        f64.add
        f64.div
        f64.const 0x1p+2
        local.get 4
        f64.const 0x1p+2
        f64.add
        f64.div
        f64.sub
        f64.const 0x1p+1
        local.get 4
        f64.const 0x1.4p+2
        f64.add
        f64.div
        f64.sub
        f64.const 0x1p+1
        local.get 4
        f64.const 0x1.8p+2
        f64.add
        f64.div
        f64.sub
        f64.mul
        f64.add
        local.set 1
        local.get 3
        f64.const 0x1p+0
        f64.add
        local.set 3
        local.get 2
        f64.const 0x1p-4
        f64.mul
        local.set 2
        local.get 0
        i32.const -1
        i32.add
        local.tee 0
        br_if 0
      end
    end
    local.get 1
  )

  (func (export "f32.no_fold_conditional_inc") (param $x f32) (param $y f32) (result f32)
    (select (local.get $x)
            (f32.add (local.get $x) (f32.const 1.0))
            (f32.lt (local.get $y) (f32.const 0.0))))
  (func (export "f64.no_fold_conditional_inc") (param $x f64) (param $y f64) (result f64)
    (select (local.get $x)
            (f64.add (local.get $x) (f64.const 1.0))
            (f64.lt (local.get $y) (f64.const 0.0))))

)
