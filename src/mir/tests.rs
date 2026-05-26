use chumsky::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use super::*;
use crate::{Error, Vm, mir::parse::grammar};

#[test]
fn reverse_list() {
    for mut list in [
        vec![],
        vec![1],
        vec![1, 2],
        vec![1, 2, 3],
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5, 6],
        vec![1, 2, 3, 4, 5, 6, 7],
        vec![1, 2, 3, 4, 5, 6, 7, 8],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    ] {
        let ops = reverse_list_ops(list.len());
        // dbg!(&ops);
        let mut vm = Vm::new(Object::from_ops(ops).assemble(), vec![]);
        vm.stack = list.clone();
        vm.run().unwrap();
        list.reverse();
        assert_eq!(vm.stack, list);
    }
}

#[test]
fn compile_expr() {
    init();

    let raw = "
            var <- add(1, 2)
            var2 <- mul(4, add(var, 1))
            var <- add(var, var2)
            ";

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    compile(&mut ctx, &ast).unwrap();

    let obj = ctx.into_object();

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, vec![]);

    let res = vm.run().unwrap();

    assert_eq!(res, None);

    assert_eq!(
        vm.stack,
        [
            19, // var
            16, // var2
        ]
    );
}

fn init() {
    let _ = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        .try_init();
}

#[test]
fn compile_if() {
    init();

    let raw = "
            var <- 2
            var2 <- 10
            if eq(1, sub(var, 1)) {
                var <- add(var, var2)
            }
            ";

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    compile(&mut ctx, &ast).unwrap();

    let obj = ctx.into_object();

    // dbg!(&obj);

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, vec![]);

    let res = vm.run().unwrap();

    assert_eq!(res, None);

    assert_eq!(
        vm.stack,
        [
            12, // var
            10  // var2
        ]
    );
}

#[test]
fn compile_if_else_if_branch() {
    init();

    let raw = "
            var <- 2
            var2 <- 10
            if 1 {
                var <- add(var, var2)
            } else {
                trap(1)
            }
            ";

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    compile(&mut ctx, &ast).unwrap();

    let obj = ctx.into_object();

    // dbg!(&obj);

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, vec![]);

    let res = vm.run().unwrap();

    assert_eq!(res, None);

    assert_eq!(
        vm.stack,
        [
            12, // var
            10  // var2
        ]
    );
}

#[test]
fn compile_if_else_else_branch() {
    init();

    let raw = "
            var <- 2
            var2 <- 10
            if eq(2, sub(var, 1)) {
                trap(1)
            } else {
                var <- add(var, var2)
            }
            ";

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    compile(&mut ctx, &ast).unwrap();

    let obj = ctx.into_object();

    // dbg!(&obj);

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, vec![]);

    let res = vm.run().unwrap();

    assert_eq!(res, None);

    assert_eq!(
        vm.stack,
        [
            12, // var
            10  // var2
        ]
    );
}

#[test]
fn compile_if_else_if() {
    init();

    let raw = "
            var <- 2
            var2 <- 10
            if 0 {
                trap(1)
            } else if 0 {
                trap(2)
            } else {
                var <- add(var, var2)
            }
            ";

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    compile(&mut ctx, &ast).unwrap();

    let obj = ctx.into_object();

    // dbg!(&obj);

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, vec![]);

    let res = vm.run().unwrap();

    assert_eq!(res, None);

    assert_eq!(
        vm.stack,
        [
            12, // var
            10  // var2
        ]
    );
}

#[test]
fn compile_def_single_arg() {
    init();

    let raw = "
            def square(i) -> o {
                o <- mul(i, i)
            }

            five <- add(1, 4)
            v <- square(five)

            u <- add(1, v)

            alloc(16)
            write8(0, v)
            write8(8, u)
            exit(0, 16)
            ";

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    compile(&mut ctx, &ast).unwrap();

    let obj = ctx.into_object();

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, vec![]);

    let res = vm.run().unwrap();

    assert_eq!(
        res,
        Some(
            [25_u64.to_be_bytes(), 26_u64.to_be_bytes()]
                .as_flattened()
                .to_vec()
        )
    );
}

#[test]
fn compile_def_multiple_args() {
    init();

    let raw = "
            def add_mul(a, b) -> o {
                o <- mul(a, add(a, b))
            }

            three <- 3
            v <- add_mul(three, 5)

            alloc(8)
            write8(0, v)
            exit(0, 8)
            ";

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    compile(&mut ctx, &ast).unwrap();

    let obj = ctx.into_object();

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, vec![]);

    let res = vm.run().unwrap();

    assert_eq!(res, Some(24_u64.to_be_bytes().to_vec()));
}

#[test]
fn fib_recursive() {
    init();

    let raw = "
            def fib(n) -> m {
                if eq(n, 0) {
                    m <- 0
                }

                if eq(n, 1) {
                    m <- 1
                }

                if gt(n, 1) {
                    m <- add(fib(sub(n, 1)), fib(sub(n, 2)))
                }
            }

            res <- fib(10)

            alloc(8)
            write8(0, res)
            exit(0, 8)
            ";

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    compile(&mut ctx, &ast).unwrap();

    let obj = ctx.into_object();

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, vec![]);

    let res = vm.run().unwrap();

    assert_eq!(res, Some(55_u64.to_be_bytes().to_vec()));
}

#[test]
fn compile_def_shadowing() {
    init();

    let raw = "
            def digit_to_place(digit, idx) -> n {
              n <- mul(digit, exp(10, sub(dlen(), add(idx, 1))))
            }

            if eq(dlen(), 0) {
              trap(1)
            }

            n <- 0
            idx <- 0

            loop :a {
              if eq(dlen(), idx) {
                break :a
              }

              ascii_digit <- dread1(idx)

              if lt(ascii_digit, 0x30) {
                trap(2)
              }

              if gt(ascii_digit, 0x39) {
                trap(3)
              }

              digit <- sub(ascii_digit, 0x30)
              n <- add(n, digit_to_place(digit, idx))
              idx <- add(idx, 1)
            }

            alloc(8)
            write8(0, n)
            exit(0, 8)
        ";

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    compile(&mut ctx, &ast).unwrap();

    let obj = ctx.into_object();

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, b"123".to_vec());

    let res = vm.run().unwrap();

    assert_eq!(res, Some(123_u64.to_be_bytes().to_vec()));
}

#[test]
fn multiple_return_values() {
    init();

    let raw = "
            def many(a) -> b, c, d, e, f {
                b <- add(a, 1)
                c <- add(a, 2)
                d <- add(a, 3)
                e <- add(a, 4)
                f <- add(a, 5)
            }

            a <- 100

            b, c, d, e, f <- many(a)

            alloc(6)
            write1(0, a)
            write1(1, b)
            write1(2, c)
            write1(3, d)
            write1(4, e)
            write1(5, f)
            exit(0, 6)
        ";

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    compile(&mut ctx, &ast).unwrap();

    let obj = ctx.into_object();

    // dbg!(&obj);

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, vec![]);

    let res = vm.run().unwrap();

    assert_eq!(res, Some(vec![100, 101, 102, 103, 104, 105]));

    assert_eq!(vm.stack, [100, 101, 102, 103, 104, 105]);
}

#[test]
fn multiple_return_values_update_and_init() {
    init();

    let raw = "
            def foo(a, b) -> c, d, e {
                c <- b
                d <- a
                e <- 0x22
            }

            a <- 0x11
            c <- 0x33

            a, b, c <- foo(a, c)

            alloc(3)
            write1(0, a)
            write1(1, b)
            write1(2, c)
            exit(0, 3)
        ";

    // # 0x33, 0x11, 0x22

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    compile(&mut ctx, &ast).unwrap();

    let obj = ctx.into_object();

    // dbg!(&obj);

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, vec![]);

    let res = vm.run().unwrap();

    assert_eq!(
        res,
        Some(vec![
            0x33, // a
            0x11, // b
            0x22, // c
        ])
    );

    // a and c are pushed to the stack, then b when it is first set in the multi
    // assignment along with a and c being updated
    assert_eq!(
        vm.stack,
        [
            0x33, // a
            0x22, // c
            0x11, // b
        ]
    );
}

#[test]
fn multiple_return_values_as_args() {
    init();

    let raw = "
            def foo(a) -> c, d {
                c <- mul(10, a)
                d <- mul(2, a)
            }

            a <- sub(...foo(4))

            alloc(1)
            write1(0, a)
            exit(0, 1)
        ";

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    compile(&mut ctx, &ast).unwrap();

    let obj = ctx.into_object();

    // dbg!(&obj);

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, vec![]);

    let res = vm.run().unwrap();

    assert_eq!(res, Some(vec![(10 * 4) - (2 * 4)]));

    assert_eq!(vm.stack, [(10 * 4) - (2 * 4)]);
}

#[test]
fn multiple_return_values_as_args_complex() {
    init();

    let raw = "
            def foo(a) -> c, d {
                c <- mul(10, a)
                d <- mul(2, a)
            }

            def bar(a, b, c) -> d {
                d <- mul(a, add(b, c))
            }

            def baz(a) -> d {
                d <- add(a, 1)
            }

            a <- 1
            res <- bar(...foo(4), a)

            alloc(3)
            write1(0, a)
            write2(1, res)
            exit(0, 3)
        ";

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    compile(&mut ctx, &ast).unwrap();

    let obj = ctx.into_object();

    // dbg!(&obj);

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, vec![]);

    let res = vm.run().unwrap();

    assert_eq!(
        res,
        Some(
            [[1_u8].as_slice(), 360_u16.to_be_bytes().as_slice()]
                .into_iter()
                .flatten()
                .copied()
                .collect::<Vec<_>>()
        )
    );
}

#[test]
fn multiple_return_swap_params() {
    init();

    let raw = "
            def swap(a_, b_) -> c, d {
                d <- a_
                c <- b_
            }

            a <- 0xaa
            b <- 0xbb

            a, b <- swap(a, b)

            alloc(2)
            write1(0, a)
            write1(1, b)
            exit(0, 2)
        ";

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    compile(&mut ctx, &ast).unwrap();

    let obj = ctx.into_object();

    // dbg!(&obj);

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, vec![]);

    let res = vm.run().unwrap();

    assert_eq!(res, Some(vec![0xbb, 0xaa]));

    assert_eq!(vm.stack, [0xbb, 0xaa]);
}

#[test]
fn compile_loop() {
    init();

    let raw = "
            counter <- 0x00

            loop :a {
              counter <- add(counter, 1)
              if eq(counter, 10) {
                break :a
              }
            }

            alloc(1)
            write1(0, counter)
            exit(0, 1)
            ";

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    compile(&mut ctx, &ast).unwrap();

    let obj = ctx.into_object();

    // dbg!(&obj);

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, vec![]);

    let res = vm.run().unwrap();

    assert_eq!(res, Some(vec![10]));
}

#[test]
fn compile_loop_shadow_label() {
    init();

    let raw = "
            counter <- 0x00

            loop :a {
              loop :a {
                counter <- add(counter, 1)
                if eq(counter, 10) {
                  break :a
                }
              }
              break :a
            }

            alloc(1)
            write1(0, counter)
            exit(0, 1)
            ";

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    compile(&mut ctx, &ast).unwrap();

    let obj = ctx.into_object();

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, vec![]);

    let res = vm.run().unwrap();

    assert_eq!(res, Some(vec![10]));
}

#[test]
fn compile_atoi() {
    // # n += digit * (10 ** (dlen() - (idx + 1)))
    let raw = "
            if eq(dlen(), 0) {
              trap(1)
            }

            n <- 0
            idx <- 0

            loop :a {
              if eq(dlen(), idx) {
                break :a
              }

              ascii_digit <- dread1(idx)

              if lt(ascii_digit, 0x30) {
                trap(2)
              }

              if gt(ascii_digit, 0x39) {
                trap(3)
              }

              digit <- sub(ascii_digit, 0x30)
              n <- add(n, mul(digit, exp(10, sub(dlen(), add(idx, 1)))))
              idx <- add(idx, 1)
            }

            alloc(8)
            write8(0, n)
            exit(0, 8)
            ";

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    compile(&mut ctx, &ast).unwrap();

    let obj = ctx.into_object();

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, b"1234567".into());

    let res = vm.run().unwrap();

    assert_eq!(res.unwrap(), 1234567_u64.to_be_bytes());
}

#[test]
fn compile_aoc_2025_1() {
    let raw = "
            if eq(dlen(), 0) {
              trap(1)
            }

            dial <- 100050
            total <- 0
            n <- 0
            idx <- 0
            is_right <- 0

            loop :a {
              if eq(dlen(), idx) {
                break :a
              }

              ascii_digit <- dread1(idx)

              if eq(ascii_digit, 76) {
                is_right <- 0

                idx <- add(idx, 1)
                n <- 0
                continue :a
              } else if eq(ascii_digit, 82) {
                is_right <- 1

                idx <- add(idx, 1)
                n <- 0
                continue :a
              } else if eq(ascii_digit, 10) {
                idx <- add(idx, 1)
                if is_right {
                  dial <- add(dial, n)
                  if eq(0, mod(dial, 100)) {
                    total <- add(total, 1)
                  }

                  continue :a
                }

                dial <- sub(dial, n)
                if eq(0, mod(dial, 100)) {
                  total <- add(total, 1)
                }

                continue :a
              }

              if lt(ascii_digit, 0x30) {
                trap(2)
              }

              if gt(ascii_digit, 0x39) {
                trap(3)
              }

              digit <- sub(ascii_digit, 0x30)
              n <- mul(n, 10)
              n <- add(n, digit)
              idx <- add(idx, 1)
            }

            alloc(8)
            write8(0, total)
            exit(0, 8)
            ";

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    compile(&mut ctx, &ast).unwrap();

    let obj = ctx.into_object();

    let asm = obj.assemble();

    let mut vm = Vm::new(
        asm,
        b"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"
        .into(),
    );

    let res = vm.run().unwrap();

    assert_eq!(res, Some(3_u64.to_be_bytes().to_vec()))
}

#[test]
fn drop_vars_in_if_block() {
    init();

    let raw = "
x <- 1
y <- 0
t <- 0
loop :a {
  if lt(t, 24) {
    Y <- mod(add(mul(2, x), mul(3, y)), 5)
    t <- add(t, 1)
  } else {
    break :a
  }
}
alloc(8)
write8(0, t)
exit(0, 8)
";

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    compile(&mut ctx, &ast).unwrap();

    let obj = ctx.into_object();

    println!("{obj}");

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, b"".into());

    let res = vm.run().unwrap();

    assert_eq!(res, Some(24_u64.to_be_bytes().to_vec()))
}

#[test]
fn drop_vars_in_def_body() {
    init();

    let raw = r#"
def f(at) -> u {
  i <- 7
}

f(0)
        "#;

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    compile(&mut ctx, &ast).unwrap();

    let obj = ctx.into_object();

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, b"".into());

    let res = vm.run().unwrap();

    assert_eq!(res, None)
}

#[test]
fn stack_depth_after_call_is_correct() {
    init();

    let raw = r#"
def inner(a, b, inner_at, value, c, d) {
  write8(inner_at, value)
}

def outer(a, b, outer_at, value, c, d) -> n, m {
  inner(a, b, outer_at, value, c, d)
  n <- 0xaa
  m <- 0xbb
  # inner(y, inner_at, z)
}

alloc(8)
n, m <- outer(0xa, 0xb, 0, 0xFFF, 0xc, 0xd)
trap(n)
        "#;

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    compile(&mut ctx, &ast).unwrap();

    let obj = ctx.into_object();

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, b"".into());

    let err = vm.run().unwrap_err();

    assert_eq!(err, Error::Trap(0xaa));
}

#[test]
fn outer_def_cannot_refer_to_called_def_arg() {
    init();

    let raw = r#"
def inner(inner_at) {
  write8(inner_at, 0x0)
}

def outer(outer_at) -> n, m {
  inner(inner_at)
}

alloc(8)
outer(1)
        "#;

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    let err = compile(&mut ctx, &ast).unwrap_err();

    assert_eq!(
        err,
        CompileError::VarNotFound {
            var: "inner_at".to_owned()
        }
    );
}
