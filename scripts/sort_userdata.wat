(module
  (import "RustData" "new" (func $RustData/new (param i32 i32) (result i32)))
  (import "RustData" "lt" (func $RustData/lt (param i32 i32) (result i32)))
  (import "RustData" "rand" (func $RustData/rand (param i32) (result i32)))
  (import "RustData" "clear" (func $RustData/clear))
  (func $swap (param $i i32) (param $j i32)
    (local $tmp i32)
    (local.set $tmp (i32.load (local.get $i)))
    (i32.store (local.get $i) (i32.load (local.get $j)))
    (i32.store (local.get $j) (local.get $tmp)))
  (func $quicksort (param $lo i32) (param $hi i32)
    (local $i i32) (local $j i32)
    (if (i32.lt_u (local.get $lo) (local.get $hi)) (then
      (loop ;; @2
        (call $swap
          (i32.shl (i32.shr_u (i32.add (local.get $hi) (local.get $lo)) (i32.const 3)) (i32.const 2))
          (local.get $hi))
        (if (i32.gt_u (local.get $hi) (local.get $lo)) (then
          (local.set $i (local.tee $j (local.get $lo)))
          (loop ;; @4
            (if (call $RustData/lt (i32.load (local.get $i)) (i32.load (local.get $hi))) (then
              (call $swap (local.get $i) (local.get $j))
              (local.set $j (i32.add (local.get $j) (i32.const 4)))))
            (br_if 0 (;@4;)
              (i32.ne (local.get $hi) (local.tee $i (i32.add (local.get $i) (i32.const 4))))))))
        (call $swap (local.get $j) (local.get $hi))
        (call $quicksort (local.get $lo) (i32.sub (local.get $j) (i32.const 4)))
        (br_if 0 (;@2;)
          (i32.lt_u (local.tee $lo (i32.add (local.get $j) (i32.const 4))) (local.get $hi)))))))
  (func $bench
    (local $n i32) (local $p i32) (local $len i32) (local $i i32)
    (call $RustData/clear)
    (local.set $n (i32.const 10000))
    (local.set $p (i32.const 44))
    (loop ;; @1
      (local.set $len (i32.add (call $RustData/rand (i32.const 16)) (i32.const 8)))
      (local.set $len (i32.and (local.get $len) (i32.const 31)))
      (local.set $i (i32.const 0))
      (loop ;; @3
        (i32.store8
          (i32.add (local.get $i) (i32.const 16))
          (i32.load8_u
            (i32.and (call $RustData/rand (i32.const 16)) (i32.const 15))))
        (br_if 0 (;@3;)
          (i32.ne (local.get $len) (local.tee $i (i32.add (local.get $i) (i32.const 1))))))
      (local.set $p (i32.add (local.get $p) (i32.const 4)))
      (i32.store (local.get $p) (call $RustData/new (i32.const 16) (local.get $len)))
      (br_if 0 (;@1;)
	(local.tee $n (i32.sub (local.get $n) (i32.const 1)))))
    (call $quicksort (i32.const 48) (local.get $p)))
  (memory $memory 1)
  (export "memory" (memory $memory))
  (export "bench" (func $bench))
  (data (;0;) (i32.const 0) "0123456789abcdef"))
