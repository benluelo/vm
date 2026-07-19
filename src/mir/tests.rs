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

    ctx.compile(&ast).unwrap();

    let obj = ctx.into_object();

    dbg!(&obj);

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

    ctx.compile(&ast).unwrap();

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

    ctx.compile(&ast).unwrap();

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

    ctx.compile(&ast).unwrap();

    let obj = ctx.into_object();

    dbg!(&obj);

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

    ctx.compile(&ast).unwrap();

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

    ctx.compile(&ast).unwrap();

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

    ctx.compile(&ast).unwrap();

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

    ctx.compile(&ast).unwrap();

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

    ctx.compile(&ast).unwrap();

    let obj = ctx.into_object();

    println!("{obj}");

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, b"123".to_vec());

    let res = vm.run().unwrap();

    assert_eq!(res, Some(123_u64.to_be_bytes().to_vec()));
}

#[test]
fn multiple_if_statements() {
    init();

    let raw = "
    alloc(8)

    if 1 {
      write1(0, 1)
    }

    if 0 {
      write1(1, 2)
    }
    ";

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    ctx.compile(&ast).unwrap();

    let obj = ctx.into_object();

    // dbg!(&obj);

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, vec![]);

    let res = vm.run().unwrap();

    assert_eq!(res, None);

    assert_eq!(vm.stack, []);

    assert_eq!(vm.memory, [1, 0, 0, 0, 0, 0, 0, 0]);
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

    ctx.compile(&ast).unwrap();

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

    ctx.compile(&ast).unwrap();

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

    ctx.compile(&ast).unwrap();

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

    ctx.compile(&ast).unwrap();

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

    ctx.compile(&ast).unwrap();

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

    ctx.compile(&ast).unwrap();

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

    ctx.compile(&ast).unwrap();

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

    ctx.compile(&ast).unwrap();

    let obj = ctx.into_object();

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, b"1234567".into());

    let res = vm.run().unwrap();

    assert_eq!(res.unwrap(), 1234567_u64.to_be_bytes());
}

#[test]
fn compile_aoc_2025_1() {
    init();

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

    ctx.compile(&ast).unwrap();

    let obj = ctx.into_object();

    println!("{obj}");
    // dbg!(&obj);

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

    ctx.compile(&ast).unwrap();

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

    ctx.compile(&ast).unwrap();

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

    ctx.compile(&ast).unwrap();

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

    let err = ctx.compile(&ast).unwrap_err();

    assert_eq!(
        err,
        CompileError::VarNotFound {
            var: "inner_at".to_owned()
        }
    );
}

#[test]
fn broken() {
    init();

    let raw = r#"
def load64(at) -> u {
  u <- read8(at)
  u <- or(and(shl(u, 8), 18374966859414961920), and(shr(u, 8), 71777214294589695))
  u <- or(and(shl(u, 16), 18446462603027742720), and(shr(u, 16), 281470681808895))
  u <- or(shl(u, 32), shr(u, 32))
}
def store64(at, u) {
  u <- or(and(shl(u, 8), 18374966859414961920), and(shr(u, 8), 71777214294589695))
  u <- or(and(shl(u, 16), 18446462603027742720), and(shr(u, 16), 281470681808895))
  write8(at, or(shl(u, 32), shr(u, 32)))
}
def xor64(at, u) {
  u <- or(and(shl(u, 8), 18374966859414961920), and(shr(u, 8), 71777214294589695))
  u <- or(and(shl(u, 16), 18446462603027742720), and(shr(u, 16), 281470681808895))
  write8(at, xor(read8(at), or(shl(u, 32), shr(u, 32))))
}
alloc(200)
temp_arr_ptr <- 200
alloc(40)
round_constants_ptr <- 240
alloc(192)
write8(240, 1)
write8(248, 32898)
write8(256, 9223372036854808714)
write8(264, 9223372039002292224)
write8(272, 32907)
write8(280, 2147483649)
write8(288, 9223372039002292353)
write8(296, 9223372036854808585)
write8(304, 138)
write8(312, 136)
write8(320, 2147516425)
write8(328, 2147483658)
write8(336, 2147516555)
write8(344, 9223372036854775947)
write8(352, 9223372036854808713)
write8(360, 9223372036854808579)
write8(368, 9223372036854808578)
write8(376, 9223372036854775936)
write8(384, 32778)
write8(392, 9223372039002259466)
write8(400, 9223372039002292353)
write8(408, 9223372036854808704)
write8(416, 2147483649)
write8(424, 9223372039002292232)
block_count <- add(1, div(dlen(), 136))
blocks_ptr <- 432
alloc(mul(136, block_count))
dcopy(0, 432, dlen())
write1(add(dlen(), 432), 6)
write1(add(431, mul(136, block_count)), or(read1(add(431, mul(136, block_count))), 128))
block_number <- 0
loop :blocks {
  if eq(block_number, block_count) {
    break :blocks
  }
  write8(0, xor(read8(0), read8(add(432, mul(block_number, 136)))))
  write8(8, xor(read8(8), read8(add(440, mul(block_number, 136)))))
  write8(16, xor(read8(16), read8(add(448, mul(block_number, 136)))))
  write8(24, xor(read8(24), read8(add(456, mul(block_number, 136)))))
  write8(32, xor(read8(32), read8(add(464, mul(block_number, 136)))))
  write8(40, xor(read8(40), read8(add(472, mul(block_number, 136)))))
  write8(48, xor(read8(48), read8(add(480, mul(block_number, 136)))))
  write8(56, xor(read8(56), read8(add(488, mul(block_number, 136)))))
  write8(64, xor(read8(64), read8(add(496, mul(block_number, 136)))))
  write8(72, xor(read8(72), read8(add(504, mul(block_number, 136)))))
  write8(80, xor(read8(80), read8(add(512, mul(block_number, 136)))))
  write8(88, xor(read8(88), read8(add(520, mul(block_number, 136)))))
  write8(96, xor(read8(96), read8(add(528, mul(block_number, 136)))))
  write8(104, xor(read8(104), read8(add(536, mul(block_number, 136)))))
  write8(112, xor(read8(112), read8(add(544, mul(block_number, 136)))))
  write8(120, xor(read8(120), read8(add(552, mul(block_number, 136)))))
  write8(128, xor(read8(128), read8(add(560, mul(block_number, 136)))))
  i <- 17
  j <- 0
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 1 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 1 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 1 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 1 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 1 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 1 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 1 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 1 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 1 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 1 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 1 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 1 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 1 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 1 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 1 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 1 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 1 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 1 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 1 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 1 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 1 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 1 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 1 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 576460752437643264)
  }
  C <- 200
  write8(200, xor(xor(xor(xor(load64(0), load64(40)), load64(80)), load64(120)), load64(160)))
  write8(208, xor(xor(xor(xor(load64(8), load64(48)), load64(88)), load64(128)), load64(168)))
  write8(216, xor(xor(xor(xor(load64(16), load64(56)), load64(96)), load64(136)), load64(176)))
  write8(224, xor(xor(xor(xor(load64(24), load64(64)), load64(104)), load64(144)), load64(184)))
  write8(232, xor(xor(xor(xor(load64(32), load64(72)), load64(112)), load64(152)), load64(192)))
  D <- xor(read8(232), xor(shl(read8(208), 1), shr(read8(208), 63)))
  xor64(0, D)
  xor64(40, D)
  xor64(80, D)
  xor64(120, D)
  xor64(160, D)
  D <- xor(read8(200), xor(shl(read8(216), 1), shr(read8(216), 63)))
  xor64(8, D)
  xor64(48, D)
  xor64(88, D)
  xor64(128, D)
  xor64(168, D)
  D <- xor(read8(208), xor(shl(read8(224), 1), shr(read8(224), 63)))
  xor64(16, D)
  xor64(56, D)
  xor64(96, D)
  xor64(136, D)
  xor64(176, D)
  D <- xor(read8(216), xor(shl(read8(232), 1), shr(read8(232), 63)))
  xor64(24, D)
  xor64(64, D)
  xor64(104, D)
  xor64(144, D)
  xor64(184, D)
  D <- xor(read8(224), xor(shl(read8(200), 1), shr(read8(200), 63)))
  xor64(32, D)
  xor64(72, D)
  xor64(112, D)
  xor64(152, D)
  xor64(192, D)
  current <- load64(8)
  temp <- load64(80)
  store64(80, xor(shl(current, 1), shr(current, 63)))
  current <- temp
  temp <- load64(56)
  store64(56, xor(shl(current, 3), shr(current, 61)))
  current <- temp
  temp <- load64(88)
  store64(88, xor(shl(current, 6), shr(current, 58)))
  current <- temp
  temp <- load64(136)
  store64(136, xor(shl(current, 10), shr(current, 54)))
  current <- temp
  temp <- load64(144)
  store64(144, xor(shl(current, 15), shr(current, 49)))
  current <- temp
  temp <- load64(24)
  store64(24, xor(shl(current, 21), shr(current, 43)))
  current <- temp
  temp <- load64(40)
  store64(40, xor(shl(current, 28), shr(current, 36)))
  current <- temp
  temp <- load64(128)
  store64(128, xor(shl(current, 36), shr(current, 28)))
  current <- temp
  temp <- load64(64)
  store64(64, xor(shl(current, 45), shr(current, 19)))
  current <- temp
  temp <- load64(168)
  store64(168, xor(shl(current, 55), shr(current, 9)))
  current <- temp
  temp <- load64(192)
  store64(192, xor(shl(current, 2), shr(current, 62)))
  current <- temp
  temp <- load64(32)
  store64(32, xor(shl(current, 14), shr(current, 50)))
  current <- temp
  temp <- load64(120)
  store64(120, xor(shl(current, 27), shr(current, 37)))
  current <- temp
  temp <- load64(184)
  store64(184, xor(shl(current, 41), shr(current, 23)))
  current <- temp
  temp <- load64(152)
  store64(152, xor(shl(current, 56), shr(current, 8)))
  current <- temp
  temp <- load64(104)
  store64(104, xor(shl(current, 8), shr(current, 56)))
  current <- temp
  temp <- load64(96)
  store64(96, xor(shl(current, 25), shr(current, 39)))
  current <- temp
  temp <- load64(16)
  store64(16, xor(shl(current, 43), shr(current, 21)))
  current <- temp
  temp <- load64(160)
  store64(160, xor(shl(current, 62), shr(current, 2)))
  current <- temp
  temp <- load64(112)
  store64(112, xor(shl(current, 18), shr(current, 46)))
  current <- temp
  temp <- load64(176)
  store64(176, xor(shl(current, 39), shr(current, 25)))
  current <- temp
  temp <- load64(72)
  store64(72, xor(shl(current, 61), shr(current, 3)))
  current <- temp
  temp <- load64(48)
  store64(48, xor(shl(current, 20), shr(current, 44)))
  current <- temp
  r <- 44
  Y <- 0
  temp <- load64(8)
  store64(8, xor(shl(current, 44), shr(current, 20)))
  current <- temp
  t <- 24
  write8(200, load64(0))
  write8(208, load64(8))
  write8(216, load64(16))
  write8(224, load64(24))
  write8(232, load64(32))
  store64(0, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(8, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(16, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(24, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(32, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(40))
  write8(208, load64(48))
  write8(216, load64(56))
  write8(224, load64(64))
  write8(232, load64(72))
  store64(40, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(48, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(56, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(64, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(72, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(80))
  write8(208, load64(88))
  write8(216, load64(96))
  write8(224, load64(104))
  write8(232, load64(112))
  store64(80, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(88, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(96, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(104, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(112, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(120))
  write8(208, load64(128))
  write8(216, load64(136))
  write8(224, load64(144))
  write8(232, load64(152))
  store64(120, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(128, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(136, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(144, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(152, xor(read8(232), and(neg(read8(200)), read8(208))))
  write8(200, load64(160))
  write8(208, load64(168))
  write8(216, load64(176))
  write8(224, load64(184))
  write8(232, load64(192))
  store64(160, xor(read8(200), and(neg(read8(208)), read8(216))))
  store64(168, xor(read8(208), and(neg(read8(216)), read8(224))))
  store64(176, xor(read8(216), and(neg(read8(224)), read8(232))))
  store64(184, xor(read8(224), and(neg(read8(232)), read8(200))))
  store64(192, xor(read8(232), and(neg(read8(200)), read8(208))))
  x <- 5
  y <- 5
  if 0 {
    xor64(0, 1)
  }
  if 0 {
    xor64(0, 32898)
  }
  if 0 {
    xor64(0, 9223372036854808714)
  }
  if 0 {
    xor64(0, 9223372039002292224)
  }
  if 0 {
    xor64(0, 32907)
  }
  if 0 {
    xor64(0, 2147483649)
  }
  if 0 {
    xor64(0, 9223372039002292353)
  }
  if 0 {
    xor64(0, 9223372036854808585)
  }
  if 0 {
    xor64(0, 138)
  }
  if 0 {
    xor64(0, 136)
  }
  if 0 {
    xor64(0, 134219776)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 0 {
    xor64(0, 134219784)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303425536)
  }
  if 0 {
    xor64(0, 576460752303423496)
  }
  if 0 {
    xor64(0, 2048)
  }
  if 0 {
    xor64(0, 576460752437641216)
  }
  if 0 {
    xor64(0, 576460752437643272)
  }
  if 0 {
    xor64(0, 576460752303425544)
  }
  if 0 {
    xor64(0, 134217728)
  }
  if 1 {
    xor64(0, 576460752437643264)
  }
  round <- 24
  block_number <- add(block_number, 1)
}
exit(0, 32)

    "#;

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    ctx.compile(&ast).unwrap();

    let obj = ctx.into_object();

    let asm = obj.assemble();

    let mut vm = Vm::new(asm, b"".into());

    let err = vm.run().unwrap_err();

    assert_eq!(err, Error::Trap(0xaa));
}
