(module
  (func $difference (param $stop i32) (param $start i32) (result i32)
        get_local $stop
        get_local $start
        i32.sub)

  (export "difference" (func $difference))
)
