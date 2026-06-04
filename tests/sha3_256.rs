use std::fs;

use chumsky::Parser;
use vm::{
    Vm,
    mir::{
        CheckCtx, Ctx,
        parse::grammar,
        pass::{ConstEval, DefInline, Pass},
    },
};

fn load_vectors(file_name: &str) -> Vec<(usize, Vec<u8>, Vec<u8>)> {
    fs::read_to_string(format!(".nist-vectors/{file_name}"))
        .unwrap()
        .split_once("[L = 256]")
        .unwrap()
        .1
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|chunk| {
            let [len, msg, md] = chunk else { panic!() };
            (
                len.split_once(" = ").unwrap().1.parse().unwrap(),
                const_hex::decode(msg.split_once(" = ").unwrap().1).unwrap(),
                const_hex::decode(md.split_once(" = ").unwrap().1).unwrap(),
            )
        })
        .collect()
}

fn load_monte_vectors(file_name: &str) -> (Vec<u8>, Vec<Vec<u8>>) {
    let file = fs::read_to_string(format!(".nist-vectors/{file_name}")).unwrap();
    let (seed, tail) = file
        .split_once("[L = 256]")
        .unwrap()
        .1
        .trim()
        .split_once("\n")
        .unwrap();

    let mds = tail
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|chunk| {
            let [_count, md] = chunk else { panic!() };

            const_hex::decode(md.split_once(" = ").unwrap().1).unwrap()
        })
        .collect();

    (
        const_hex::decode(seed.strip_prefix("Seed = ").unwrap().trim()).unwrap(),
        mds,
    )
}

#[test]
fn nist_vectors() {
    let raw = fs::read_to_string("tests/sha3-256.mir").unwrap();

    let ast = grammar().block.parse(&raw).unwrap();

    let mut ctx = CheckCtx::new("root");
    ctx.check(&ast).unwrap();
    let ast = ConstEval {}.run(&ctx, ast);

    let mut ctx = CheckCtx::new("root");
    ctx.check(&ast).unwrap();
    let ast = DefInline {}.run(&ctx, ast);

    let mut ctx = CheckCtx::new("root");
    ctx.check(&ast).unwrap();
    let ast = ConstEval {}.run(&ctx, ast);

    let mut ctx = Ctx::new_root();

    ctx.compile(&ast).unwrap();

    let obj = ctx.into_object();

    let asm = obj.assemble();

    let run_tests = |file_name| {
        println!("{file_name}");
        for (i, (len, msg, md)) in load_vectors(file_name).into_iter().enumerate() {
            let mut vm = Vm::new(asm.clone(), msg[0..(len / 8)].to_vec());

            let res = vm.run().unwrap().unwrap();

            assert_eq!(res, md, "[{i}] failed: {}", const_hex::encode(msg));

            println!("[{i}] pass");
        }
    };

    let run_monte_tests = |file_name| {
        println!("{file_name}");
        let (mut seed, mds) = load_monte_vectors(file_name);
        for (i, md) in mds.into_iter().enumerate() {
            for _ in 0..1000 {
                let mut vm = Vm::new(asm.clone(), seed.clone());

                let res = vm.run().unwrap().unwrap();

                seed = res;
            }

            assert_eq!(seed, md, "[{i}] failed");

            println!("[{i}] pass");
        }
    };

    run_tests("SHA3_256ShortMsg.rsp");
    run_tests("SHA3_256LongMsg.rsp");

    // run_monte_tests("SHA3_256Monte.rsp");
}
