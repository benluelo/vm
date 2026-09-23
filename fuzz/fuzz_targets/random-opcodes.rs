#![no_main]

use libfuzzer_sys::{Corpus, fuzz_target};
use vm::{CycleCountHook, CycleCountVm, Op, Vm, VmT, ffi, tail::TcVm, zig};

fuzz_target!(|input: (Vec<Op>, &[u8])| -> Corpus { go(input) });

fn go(input: (Vec<Op>, &[u8])) -> Corpus {
    let (obj, data) = input;
    if obj.iter().any(|s| {
        matches!(
            s,
            Op::ALLOC
                | Op::PUSH0
                | Op::PUSH1([0])
                | Op::PUSH2([0, 0])
                | Op::PUSH3([0, 0, 0])
                | Op::PUSH4([0, 0, 0, 0])
                | Op::PUSH5([0, 0, 0, 0, 0])
                | Op::PUSH6([0, 0, 0, 0, 0, 0])
                | Op::PUSH7([0, 0, 0, 0, 0, 0, 0])
                | Op::PUSH8([0, 0, 0, 0, 0, 0, 0, 0])
        ) || (data.is_empty() && s == &Op::DLEN)
    }) {
        return Corpus::Reject;
    }
    let obj = obj.into_iter().flat_map(Op::to_bytes).collect::<Vec<_>>();
    let mut canonical_vm = Vm::new_with(obj.clone(), data.to_vec(), CycleCountHook::new());
    let canonical = canonical_vm.run();

    dbg!(&canonical);

    let mut tc_vm = TcVm::new(Vm::new_with(obj.clone(), data.to_vec(), CycleCountHook::new()));
    let tc = tc_vm.run();
    assert_eq!(canonical, tc, "tc");
    assert_eq!(canonical_vm.cycles(), tc_vm.cycles(), "tc");

    let mut ffi_vm = ffi::Vm::new(obj.clone(), data.to_vec());
    let ffi_res = ffi_vm.run();
    assert_eq!(canonical, ffi_res, "c");
    assert_eq!(canonical_vm.cycles(), ffi_vm.cycles(), "c");

    let mut zig_vm = zig::Vm::new(obj.clone(), data.to_vec());
    let zig_res = zig_vm.run();
    assert_eq!(canonical, zig_res, "zig");
    assert_eq!(canonical_vm.cycles(), zig_vm.cycles(), "zig");

    Corpus::Keep
}
