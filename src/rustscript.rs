use pd_vm_runtime::{compile_source, CallOutcome, CallReturn, JitConfig, Vm, VmResult, VmStatus};
use rand::RngExt;
use std::cell::RefCell;

pub use pd_vm_runtime::Value;

#[derive(Clone)]
struct RustData(Value);

thread_local! {
    static RUST_DATA: RefCell<Vec<RustData>> = const { RefCell::new(Vec::new()) };
}

fn rand(args: &[Value]) -> VmResult<CallOutcome> {
    let Value::Int(n) = args[0] else {
        unreachable!("rand argument is statically typed as int")
    };
    Ok(CallOutcome::Return(CallReturn::one(Value::Int(
        rand::rng().random_range(0..n),
    ))))
}

fn rust_data_new(args: &[Value]) -> VmResult<CallOutcome> {
    let Value::String(_) = &args[0] else {
        unreachable!("RustData value is statically typed as a string")
    };
    let handle = RUST_DATA.with(|values| {
        let mut values = values.borrow_mut();
        let handle = i64::try_from(values.len()).expect("RustData handle must fit i64");
        values.push(RustData(args[0].clone()));
        handle
    });
    Ok(CallOutcome::Return(CallReturn::one(Value::Int(handle))))
}

fn rust_data_lt(args: &[Value]) -> VmResult<CallOutcome> {
    let (Value::Int(lhs), Value::Int(rhs)) = (&args[0], &args[1]) else {
        unreachable!("RustData arguments are statically typed as int handles")
    };
    let is_less = RUST_DATA.with(|values| {
        let values = values.borrow();
        let Value::String(lhs) = &values[*lhs as usize].0 else {
            unreachable!("RustData must wrap a string Value")
        };
        let Value::String(rhs) = &values[*rhs as usize].0 else {
            unreachable!("RustData must wrap a string Value")
        };
        lhs < rhs
    });
    Ok(CallOutcome::Return(CallReturn::one(Value::Bool(is_less))))
}

fn reset_rust_data() {
    RUST_DATA.with(|values| *values.borrow_mut() = Vec::new());
}

pub fn validate_sorted_handles(stack: &[Value]) {
    let [Value::Array(handles)] = stack else {
        panic!("expected the sorted handle array on the stack, got {stack:?}");
    };
    RUST_DATA.with(|values| {
        let values = values.borrow();
        assert_eq!(values.len(), 10000);
        assert_eq!(handles.len(), 10000);
        assert!(handles.windows(2).all(|pair| match pair {
            [Value::Int(lhs), Value::Int(rhs)] => {
                let Value::String(lhs) = &values[*lhs as usize].0 else {
                    return false;
                };
                let Value::String(rhs) = &values[*rhs as usize].0 else {
                    return false;
                };
                lhs <= rhs
            }
            _ => false,
        }));
    });
}

pub fn sort_userdata(
    jit: bool,
    run: impl FnOnce(&mut dyn FnMut()),
    validate: impl FnOnce(&[Value]),
) -> Result<(), Box<dyn std::error::Error>> {
    reset_rust_data();
    let compiled = compile_source(include_str!("../scripts/sort_userdata.rss"))?;
    let mut jit_config = JitConfig::default();
    jit_config.enabled = jit;
    let mut vm = Vm::new_with_jit_config(compiled.program, jit_config);
    vm.bind_static_non_yielding_args_function("rand", rand);
    vm.bind_static_non_yielding_args_function("rust_data_new", rust_data_new);
    vm.bind_static_non_yielding_args_function("rust_data_lt", rust_data_lt);

    assert_eq!(vm.run()?, VmStatus::Halted);
    validate(vm.stack());
    if jit {
        assert!(
            vm.jit_native_exec_count() > 0,
            "RustScript JIT benchmark did not execute a native trace"
        );
    }
    run(&mut || {
        reset_rust_data();
        vm.reset_for_reuse();
        assert_eq!(vm.run().unwrap(), VmStatus::Halted);
    });

    Ok(())
}
