use std::fs;

use chumsky::Parser;
use serde::{Deserialize, Serialize};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use vm::{
    Vm, ffi,
    mir::{
        CheckCtx, Ctx,
        parse::grammar,
        pass::{ConstEval, ConstProp, DeadCodeRemoval, DefInline, LoopUnroll, MergeAlloc, Pass},
    },
};

fn load_nist_vectors(file_name: &str) -> Vec<(usize, Vec<u8>, Vec<u8>)> {
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
    let (seed, tail) = file.split_once("[L = 256]").unwrap().1.trim().split_once("\n").unwrap();

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

    (const_hex::decode(seed.strip_prefix("Seed = ").unwrap().trim()).unwrap(), mds)
}

#[test]
fn nist_vectors() {
    let _ = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        .try_init();

    let raw = fs::read_to_string("tests/sha3-256.mir").unwrap();

    let ast = grammar().block.parse(&raw).unwrap();

    let mut ctx = CheckCtx::new("root");
    ctx.check(&ast).unwrap();
    let ast = DefInline::new().run(&ctx, ast);

    let mut ctx = CheckCtx::new("root");
    ctx.check(&ast).unwrap();
    let ast = DefInline::new().run(&ctx, ast);

    let mut ast = ast;

    for _ in 1..=3 {
        let mut ctx = CheckCtx::new("root");
        ast = ctx.check_with(&ast, &mut LoopUnroll).unwrap();

        for i in 1.. {
            let mut ctx = CheckCtx::new("root");
            let new_ast = ctx.check_with(&ast, &mut ConstProp).unwrap();

            let mut ctx = CheckCtx::new("root");
            ctx.check(&new_ast).unwrap();
            let new_ast = DefInline::new().run(&ctx, new_ast);

            let mut ctx = CheckCtx::new("root");
            let new_ast = ctx.check_with(&new_ast, &mut ConstEval::new()).unwrap();

            let mut ctx = CheckCtx::new("root");
            let new_ast = ctx.check_with(&new_ast, &mut DeadCodeRemoval).unwrap();

            let mut ctx = CheckCtx::new("root");
            let new_ast = ctx.check_with(&new_ast, &mut MergeAlloc).unwrap();

            if new_ast == ast {
                info!("ran const prop/eval loop {i} times");
                break;
            } else {
                ast = new_ast;
            }
        }
    }

    let mut ctx = Ctx::new_root();

    ctx.compile(&ast).unwrap();

    let obj = ctx.into_object();

    let asm = obj.assemble();

    let run_tests = |file_name| {
        println!("{file_name}");
        for (i, (len, msg, md)) in load_nist_vectors(file_name).into_iter().enumerate() {
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

    run_monte_tests("SHA3_256Monte.rsp");
}

pub fn make_test_input(len: usize) -> Vec<u8> {
    (0..).map(|i| (i % 251) as u8).take(len).collect()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cases {
    pub _comment: String,
    pub key: String,
    pub context_string: String,
    pub cases: Vec<Case>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Case {
    pub input_len: usize,
    pub hash: String,
    pub keyed_hash: String,
    pub derive_key: String,
}

fn load_blake3_vectors() -> Cases {
    serde_json::from_str(&fs::read_to_string(".blake3-vectors/test_vectors.json").unwrap()).unwrap()
}

#[test]
fn blake3_vectors() {
    let _ = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        .try_init();

    let raw = fs::read_to_string("tests/blake3.mir").unwrap();

    let ast = grammar().block.parse(&raw).unwrap();

    let mut ctx = CheckCtx::new("root");
    ctx.check(&ast).unwrap();
    let ast = DefInline::new().run(&ctx, ast);

    let mut ctx = CheckCtx::new("root");
    ctx.check(&ast).unwrap();
    let ast = DefInline::new().run(&ctx, ast);

    let mut ast = ast;

    for _ in 1..=3 {
        let mut ctx = CheckCtx::new("root");
        ast = ctx.check_with(&ast, &mut LoopUnroll).unwrap();

        for i in 1.. {
            let mut ctx = CheckCtx::new("root");
            let new_ast = ctx.check_with(&ast, &mut ConstProp).unwrap();

            let mut ctx = CheckCtx::new("root");
            ctx.check(&new_ast).unwrap();
            let new_ast = DefInline::new().run(&ctx, new_ast);

            let mut ctx = CheckCtx::new("root");
            let new_ast = ctx.check_with(&new_ast, &mut ConstEval::new()).unwrap();

            let mut ctx = CheckCtx::new("root");
            let new_ast = ctx.check_with(&new_ast, &mut DeadCodeRemoval).unwrap();

            let mut ctx = CheckCtx::new("root");
            let new_ast = ctx.check_with(&new_ast, &mut MergeAlloc).unwrap();

            if new_ast == ast {
                info!("ran const prop/eval loop {i} times");
                break;
            } else {
                ast = new_ast;
            }
        }
    }

    let mut ctx = Ctx::new_root();

    ctx.compile(&ast).unwrap();

    let obj = ctx.into_object();

    let asm = obj.assemble();

    for case in load_blake3_vectors().cases {
        let input = make_test_input(case.input_len);
        let expected = const_hex::decode(case.hash).unwrap();

        let mut vm = ffi::Vm::new(asm.clone(), input);

        let res = vm.run().unwrap().unwrap();

        assert_eq!(
            res,
            &expected[..32],
            "[{}] failed: {}",
            case.input_len,
            const_hex::encode(&res)
        );

        println!("[{}] pass", case.input_len);
    }
}
