use super::*;

fn return_3_as_a_single_byte() -> impl IntoIterator<Item = u8> {
    use Op::*;

    [
        // init counter
        PUSH1([0]),
        // begin loop
        // add 1
        PUSH1([1]),
        ADD,
        // loop check
        DUP,
        PUSH1([3]),
        SUB,
        // jump to beginning of loop (offset 2 in the bytecode) if the value on the top of the
        // stack is non-zero (value - 3)
        PUSH1([2]),
        JNZ,
        // end loop
        // init memory for value
        PUSH1([1]),
        ALLOC,
        // update value in memory
        PUSH1([0]),
        WRITE1,
        PUSH1([1]), // len
        PUSH1([0]), // ptr
        EXIT,
    ]
    .into_iter()
    .flat_map(Op::to_bytes)
}

#[derive(Default)]
struct Case {
    code: Vec<u8>,
    data: Vec<u8>,
    before_pc: usize,
    before_stack: Vec<u64>,
    before_memory: Vec<u8>,
    after_pc: usize,
    after_stack: Vec<u64>,
    after_memory: Vec<u8>,
}

fn asm_to_code(asm: &str) -> Vec<u8> {
    parse_asm().parse(asm).unwrap().assemble()
}

fn ops_to_code(ops: impl IntoIterator<Item = Op>) -> Vec<u8> {
    ops.into_iter().flat_map(Op::to_bytes).collect()
}

macro_rules! test_cases {
    ($(
        $name:ident {
            $($tt:tt)*
        };
    )*) => {
        $(
            #[test]
            fn $name() {
                let Case {
                    code,
                    data,
                    before_pc,
                    before_stack,
                    before_memory,
                    after_pc,
                    after_stack,
                    after_memory,
                } = Case {$($tt)* ..Default::default()};
                let mut vm = Vm::new(code, data);
                vm.stack = before_stack;
                vm.memory = before_memory;
                let mut pc = before_pc;
                vm.step(&mut pc).unwrap();
                assert_eq!(pc, after_pc);
                assert_eq!(vm.stack, after_stack);
                assert_eq!(vm.memory, after_memory);
            }
        )*
    };
}

#[cfg(test)]
mod arithmetic {
    use super::*;

    test_cases! {
        add {
            code: ops_to_code([Op::ADD]),
            before_stack: vec![1, 2],
            after_pc: 1,
            after_stack: vec![3],
        };

        add_wraps {
            code: ops_to_code([Op::ADD]),
            before_stack: vec![u64::MAX, 10],
            after_pc: 1,
            after_stack: vec![9],
        };

        sub {
            code: ops_to_code([Op::SUB]),
            before_stack: vec![5, 3],
            after_pc: 1,
            after_stack: vec![2],
        };

        sub_wraps {
            code: ops_to_code([Op::SUB]),
            before_stack: vec![3, 4],
            after_pc: 1,
            after_stack: vec![u64::MAX],
        };

        mul {
            code: ops_to_code([Op::MUL]),
            before_stack: vec![3, 4],
            after_pc: 1,
            after_stack: vec![12],
        };

        mul_wraps {
            code: ops_to_code([Op::MUL]),
            before_stack: vec![u64::MAX, 2],
            after_pc: 1,
            after_stack: vec![u64::MAX - 1],
        };

        div {
            code: ops_to_code([Op::DIV]),
            before_stack: vec![4, 2],
            after_pc: 1,
            after_stack: vec![2],
        };

        div_floors_half {
            code: ops_to_code([Op::DIV]),
            before_stack: vec![5, 2],
            after_pc: 1,
            after_stack: vec![2],
        };

        div_floors_less_than_half {
            code: ops_to_code([Op::DIV]),
            before_stack: vec![14, 10],
            after_pc: 1,
            after_stack: vec![1],
        };

        div_floors_more_than_half {
            code: ops_to_code([Op::DIV]),
            before_stack: vec![14, 10],
            after_pc: 1,
            after_stack: vec![1],
        };

        exp {
            code: ops_to_code([Op::EXP]),
            before_stack: vec![5, 3],
            after_pc: 1,
            after_stack: vec![125],
        };

        exp_wraps {
            code: ops_to_code([Op::EXP]),
            before_stack: vec![1_000_000_000, 3],
            after_pc: 1,
            after_stack: vec![1_000_000_000_000_000_000_000_000_000_u128 as u64],
        };

        mod_ {
            code: ops_to_code([Op::MOD]),
            before_stack: vec![5, 3],
            after_pc: 1,
            after_stack: vec![2],
        };

        eq_true {
            code: ops_to_code([Op::EQ]),
            before_stack: vec![1, 1],
            after_pc: 1,
            after_stack: vec![1],
        };

        eq_false {
            code: ops_to_code([Op::EQ]),
            before_stack: vec![1, 0],
            after_pc: 1,
            after_stack: vec![0],
        };

        neq_true {
            code: ops_to_code([Op::NEQ]),
            before_stack: vec![0, 1],
            after_pc: 1,
            after_stack: vec![1],
        };

        neq_false {
            code: ops_to_code([Op::NEQ]),
            before_stack: vec![1, 0],
            after_pc: 1,
            after_stack: vec![1],
        };

        not_0 {
            code: ops_to_code([Op::NOT]),
            before_stack: vec![0],
            after_pc: 1,
            after_stack: vec![1],
        };

        not_1 {
            code: ops_to_code([Op::NOT]),
            before_stack: vec![1],
            after_pc: 1,
            after_stack: vec![0],
        };

        not_gt_1 {
            code: ops_to_code([Op::NOT]),
            before_stack: vec![123456789],
            after_pc: 1,
            after_stack: vec![0],
        };

        // not_0 {
        //     code: ops_to_code([Op::NOT]),
        //     before_stack: vec![0],
        //     after_pc: 1,
        //     after_stack: vec![u64::MAX],
        // };

        // not_1 {
        //     code: ops_to_code([Op::NOT]),
        //     before_stack: vec![1],
        //     after_pc: 1,
        //     after_stack: vec![u64::MAX - 1],
        // };

        // not_gt_1 {
        //     code: ops_to_code([Op::NOT]),
        //     before_stack: vec![123456789],
        //     after_pc: 1,
        //     after_stack: vec![!123456789],
        // };
    }
}

#[cfg(test)]
mod stack {
    use super::*;

    test_cases! {
        swap_0 {
            code: ops_to_code([Op::SWAP]),
            before_stack: vec![3, 2, 0],
            after_pc: 1,
            after_stack: vec![2, 3],
        };

        swap_1 {
            code: ops_to_code([Op::SWAP]),
            before_stack: vec![3, 0, 2, 1],
            after_pc: 1,
            after_stack: vec![2, 0, 3],
        };

        swap_10 {
            code: ops_to_code([Op::SWAP]),
            before_stack: vec![2000, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1000, 10],
            after_pc: 1,
            after_stack: vec![1000, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 2000],
        };

        dup_0 {
            code: ops_to_code([Op::DUP]),
            before_stack: vec![2, 0],
            after_pc: 1,
            after_stack: vec![2, 2],
        };

        dup_1 {
            code: ops_to_code([Op::DUP]),
            before_stack: vec![2, 3, 1],
            after_pc: 1,
            after_stack: vec![2, 3, 2],
        };

        dup_10 {
            code: ops_to_code([Op::DUP]),
            before_stack: vec![1000, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            after_pc: 1,
            after_stack: vec![1000, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1000],
        };
    }
}
